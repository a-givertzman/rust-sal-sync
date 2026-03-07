#[cfg(test)]
mod stress_tests {
    use sal_core::dbg::Dbg;

    use crate::services::{MultiQueue, MultiQueueConf, Services, SubscriptionCriteria};
    use crate::services::entity::Cot;

    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, Instant};
    // Предполагаемые импорты ваших типов:
    // use crate::services::entity::{Point, PointTxId};
    // use crate::services::subscription::SubscriptionCriteria;

    // Заглушка для мока зависимостей. Замените на реальные конструкторы вашей системы.
    fn setup_multiqueue(dbg: &Dbg) -> Arc<MultiQueue> {

        let mut conf = serde_yaml::from_str(r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                send-to:
        "#).unwrap();
        let mq_conf = MultiQueueConf::from_yaml(dbg, &conf);
        let services = Arc::new(Services::new()); // Мок сервисов
        
        let mq = Arc::new(MultiQueue::new(conf, services, None));
        mq.run().expect("Failed to start MultiQueue");
        mq
    }

    #[test]
    fn test_multiqueue_under_heavy_load() {
        let mq = setup_multiqueue();
        let running = Arc::new(AtomicBool::new(true));
        let events_sent = Arc::new(AtomicUsize::new(0));
        let events_received = Arc::new(AtomicUsize::new(0));

        let mut handles = vec![];

        // 1. Spammer Threads (Генерация безумного трафика)
        for spammer_id in 0..2 {
            let mq_clone = mq.clone();
            let running = running.clone();
            let events_sent = events_sent.clone();
            let tx_link = mq_clone.get_link("mq_rx"); // Получаем Sender для спама

            handles.push(thread::spawn(move || {
                let spammer_txid = PointTxId::from_str(&format!("spammer_{}", spammer_id));
                let mut counter = 0;
                
                while running.load(Ordering::Relaxed) {
                    // Чередуем два разных destination
                    let dest = if counter % 2 == 0 { "Sensor_A" } else { "Sensor_B" };
                    
                    // Конструируем Point (замените на ваш реальный конструктор)
                    // Важно: txid отправителя не должен совпадать с именами подписчиков!
                    let point = Point::mock(spammer_txid, dest, counter); 
                    
                    if tx_link.send(point).is_ok() {
                        events_sent.fetch_add(1, Ordering::Relaxed);
                    }
                    counter += 1;
                }
            }));
        }

        // 2. Multicast Subscriber (Стабильный)
        let mq_mc = mq.clone();
        let running_mc = running.clone();
        let ev_recv_mc = events_received.clone();
        handles.push(thread::spawn(move || {
            let criteria = vec![SubscriptionCriteria::new("Sensor_A", Cot::Inf)];
            let (_, rx) = mq_mc.subscribe("multicast_client_1", &criteria);
            
            while running_mc.load(Ordering::Relaxed) {
                if let Ok(point) = rx.recv_timeout(Duration::from_millis(10)) {
                    // ПРОВЕРКА: Multicast должен получать только Sensor_A
                    assert_eq!(point.dest(), "Sensor_A", "Multicast received wrong destination!");
                    ev_recv_mc.fetch_add(1, Ordering::Relaxed);
                }
            }
        }));

        // 3. Broadcast Subscriber (Стабильный)
        let mq_bc = mq.clone();
        let running_bc = running.clone();
        let ev_recv_bc = events_received.clone();
        handles.push(thread::spawn(move || {
            let (_, rx) = mq_bc.subscribe("broadcast_client_1", &[]); // Пустой срез = Broadcast
            
            while running_bc.load(Ordering::Relaxed) {
                if rx.recv_timeout(Duration::from_millis(10)).is_ok() {
                    // Broadcast получает всё, просто считаем
                    ev_recv_bc.fetch_add(1, Ordering::Relaxed);
                }
            }
        }));

        // 4. Chaotic Subscriber (Подписка -> Отписка -> Проверка)
        let mq_chaos = mq.clone();
        let running_chaos = running.clone();
        handles.push(thread::spawn(move || {
            let criteria = vec![SubscriptionCriteria::new("Sensor_B", Cot::Inf)];
            let client_name = "chaotic_client_1";

            while running_chaos.load(Ordering::Relaxed) {
                // Подписываемся
                let (_, rx) = mq_chaos.subscribe(client_name, &criteria);
                
                // Читаем немного сообщений (даем время спамерам накидать)
                let mut read_count = 0;
                while read_count < 100 && running_chaos.load(Ordering::Relaxed) {
                    if let Ok(p) = rx.recv_timeout(Duration::from_millis(5)) {
                        assert_eq!(p.dest(), "Sensor_B");
                        read_count += 1;
                    }
                }

                // Отписываемся
                mq_chaos.unsubscribe(client_name, &criteria).expect("Unsubscribe failed");

                // Выгребаем остатки, которые успели залететь в канал до отписки
                while rx.try_recv().is_ok() {}

                // ПРОВЕРКА: Канал должен быть реально пуст, новые события не приходят
                let timeout_res = rx.recv_timeout(Duration::from_millis(50));
                assert!(
                    timeout_res.is_err(), 
                    "Chaotic client received a message after unsubscribing and draining! Memory leak or subscription bug."
                );

                // Немного ждем перед новой подпиской
                thread::sleep(Duration::from_millis(10));
            }
        }));

        // 5. Запуск стресс-теста на определенное время
        println!("Starting stress test for 5 seconds...");
        let start_time = Instant::now();
        thread::sleep(Duration::from_secs(5));

        // 6. Остановка
        running.store(false, Ordering::Relaxed);
        mq.exit(); // Корректно завершаем сам MultiQueue
        mq.wait().unwrap(); // Ждем завершения потока
        
        for h in handles {
            let _ = h.join();
        }

        // 7. Подсчет метрик производительности
        let elapsed = start_time.elapsed();
        let sent = events_sent.load(Ordering::Relaxed);
        let received = events_received.load(Ordering::Relaxed);
        
        let mcs_per_event = elapsed.as_micros() as f64 / sent as f64;
        let events_per_sec = (sent as f64 / elapsed.as_secs_f64()) as usize;

        println!("--- STRESS TEST RESULTS ---");
        println!("Duration: {:?}", elapsed);
        println!("Events Sent: {}", sent);
        println!("Events Delivered (Broadcast + Multicast): {}", received);
        println!("Throughput: {} events/sec", events_per_sec);
        println!("Performance: {:.3} µs per event", mcs_per_event);
        
        // Базовые ассерты, чтобы тест падал, если диспетчер вообще не работает
        assert!(sent > 1000, "Queue is suspiciously slow, sent less than 1000 events.");
        assert!(received > sent, "Received should be > sent (since broadcast duplicates events).");
    }
}
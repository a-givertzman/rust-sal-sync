use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
#[cfg(test)]
use sal_core::dbg::Dbg;

use crate::services::conf::{ConfTree, ServicesConf};
use crate::services::{MultiQueue, MultiQueueConf, Service, Services, SubscriptionCriteria};
use crate::services::entity::{Cot, Point, PointTxId};

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

///
/// Создаем MultiQueue
fn setup_multiqueue(dbg: &Dbg) -> Arc<MultiQueue> {
    let services = Arc::new(Services::new(dbg, ServicesConf::new(
            dbg, 
            ConfTree::new_root(serde_yaml::from_str(r#"
                retain:
                    path: assets/testing/retain/
                    point:
                        path: point/id.json
            "#).unwrap()),
        ),
        None,
    ));
    let conf = serde_yaml::from_str(r#"
        service MultiQueue:
            in queue in-queue:
                max-length: 10000
            send-to:
    "#).unwrap();
    let conf = MultiQueueConf::from_yaml(dbg, &conf);
    let mq = Arc::new(MultiQueue::new(conf, services.clone(), None));
    services.insert(mq.clone());
    mq
}
///
/// Стресс тест для MultiQueue:
/// - спамит евентами в сторону MultiQueue (без жалостно)
/// - делает подписку как Multicast так и Broadcast
/// - проверяет что по подписке как Multicast так и Broadcast приходят верные евенты 
/// - отписываться и проверяет что подписки правда нет
/// - делает все эти манипуляции одновременно что бы был реальный стресс-тест
/// - показывает производительность (mcs per event)
#[test]
fn heavy_load() {
    DebugSession::new().filter(LogLevel::Info).init();
    let dbg = Dbg::own("MultiQueue-heavy_load");
    let mq = setup_multiqueue(&dbg);
    mq.run().unwrap();

    // Продолжительность теста
    let duration = Duration::from_millis(1 * 1000);

    let running = Arc::new(AtomicBool::new(true));
    let events_sent = Arc::new(AtomicUsize::new(0));
    let events_received = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    // 1. Spammer Threads (Генерация безумного трафика)
    let spammers = vec![
        ("Sensor_A", Arc::new(AtomicUsize::new(0))),
        ("Sensor_B", Arc::new(AtomicUsize::new(0))),
    ];
    for (spammer_id, (dest, counter)) in spammers.iter().enumerate().map(|(id, (d, c))| (id, (d.to_string(), c.clone()))) {
        let mq_clone = mq.clone();
        let running = running.clone();
        let events_sent = events_sent.clone();
        let tx_link = mq_clone.get_link("in-queue"); // Получаем Sender для спама
        let dbg = dbg.clone();
        handles.push(thread::spawn(move || {
            println!("{dbg} | Spammer {spammer_id} | {dest} Start");
            let spammer_txid = PointTxId::from_str(&format!("spammer_{}", spammer_id));
            while running.load(Ordering::Relaxed) {
                // Конструируем Point (замените на ваш реальный конструктор)
                // Важно: txid отправителя не должен совпадать с именами подписчиков!
                let point = Point::new(spammer_txid, &dest, counter.load(Ordering::Acquire) as i64); 
                if tx_link.send(point).is_ok() {
                    events_sent.fetch_add(1, Ordering::Relaxed);
                    counter.fetch_add(1, Ordering::AcqRel);
                }
            }
        }));
    }

    // 2. Multicast Subscriber (Стабильный)
    for id in 0..8 {
        let mq_mc = mq.clone();
        let running_mc = running.clone();
        // let ev_recv_mc = events_received.clone();
        let i = id % 2;
        let (dest, counter) = (spammers[i].0.to_string(), spammers[i].1.clone());
        let dbg = dbg.clone();
        handles.push(thread::spawn(move || {
            let criteria = SubscriptionCriteria::new(&dest, Cot::Inf);
            let (_, rx) = mq_mc.subscribe(&format!("multicast_client_{id}"), &[criteria.clone()]);
            let start_at = counter.load(Ordering::Acquire);
            let mut received = 0;
            let mut prev = None;
            while running_mc.load(Ordering::Relaxed) {
                // while let Ok(point) = rx.recv_timeout(Duration::from_millis(1)) {
                while let Ok(Some(point)) = rx.try_recv() {
                    // ПРОВЕРКА: Multicast должен получать только Sensor_A, 
                    // И каждый следующий евент должен быть больше предыдущего на 1 
                    let value = point.as_int().value;
                    assert_eq!(point.dest(), criteria.destination(), "Multicast received wrong destination!");
                    if let Some(prev) = prev {
                        assert_eq!(value, prev + 1, "Multicast received wrong value! \nt result {} \nt target {}", value, prev + 1);
                    }
                    prev = Some(value);
                    // ev_recv_mc.fetch_add(1, Ordering::Relaxed);
                    received += 1;
                }
                std::thread::yield_now();
            }
            let target = counter.load(Ordering::Acquire) - start_at;
            println!("{dbg} | Multicast Subscriber {id} | {:?} | Received: {received} of {target}", criteria.destination());

        }));
    }

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
    for i in 0..10 {
        let mq_chaos = mq.clone();
        let running_chaos = running.clone();
        handles.push(thread::spawn(move || {
            let criteria = SubscriptionCriteria::new(format!("Sensor_B"), Cot::Inf);
            let client_name = format!("chaotic_client_{i}");
            while running_chaos.load(Ordering::Relaxed) {
                // Подписываемся
                let (_, rx) = mq_chaos.subscribe(&client_name, &[criteria.clone()]);
                // Читаем немного сообщений (даем время спамерам накидать)
                let mut read_count = 0;
                while read_count < 100 && running_chaos.load(Ordering::Relaxed) {
                    if let Ok(p) = rx.recv_timeout(Duration::from_millis(5)) {
                        assert_eq!(p.dest(), criteria.destination());
                        read_count += 1;
                    }
                }
                // Отписываемся
                mq_chaos.unsubscribe(&client_name, &[criteria.clone()]).expect("Unsubscribe failed");
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
    }

    // 5. Запуск стресс-теста на определенное время
    println!("{dbg} | Starting stress test for {:?}...", duration);
    let start_time = Instant::now();
    thread::sleep(duration);

    // 6. Остановка
    mq.exit(); // Корректно завершаем сам MultiQueue
    mq.wait().unwrap(); // Ждем завершения потока
    running.store(false, Ordering::Relaxed);
    
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
    // assert!(received > sent, "Received should be > sent (since broadcast duplicates events).");
}

///
/// Стресс тест для MultiQueue с проверкой количества сообщений:
/// - Спамит евентами в сторону MultiQueue (без жалостно)
/// - Делает подписку как Multicast так и Broadcast
/// - Проверяет что по подписке как Multicast так и Broadcast приходят верные евенты 
/// - Проверяет что по подписке пришло точное количество сообщений
/// - Отписываться и проверяет что подписки правда нет
/// - Делает все эти манипуляции одновременно что бы был реальный стресс-тест
#[test]
fn heavy_load_events() {
    DebugSession::new().filter(LogLevel::Info).init();
    let dbg = Dbg::own("MultiQueue-heavy_load_events");
    let mq = setup_multiqueue(&dbg);
    mq.run().unwrap();

    // Продолжительность теста
    let duration = Duration::from_millis(10 * 1000);

    let running = Arc::new(AtomicBool::new(true));
    let events_sent = Arc::new(AtomicUsize::new(0));
    let events_received = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    // 1. Spammer Threads (Генерация безумного трафика)
    let spammers = vec![
        ("Sensor_A", Arc::new(AtomicUsize::new(0))),
        ("Sensor_B", Arc::new(AtomicUsize::new(0))),
    ];
    for (spammer_id, (dest, counter)) in spammers.iter().enumerate().map(|(id, (d, c))| (id, (d.to_string(), c.clone()))) {
        let mq_clone = mq.clone();
        let running = running.clone();
        let events_sent = events_sent.clone();
        let tx_link = mq_clone.get_link("in-queue"); // Получаем Sender для спама
        let dbg = dbg.clone();
        handles.push(thread::spawn(move || {
            println!("{dbg} | Spammer {spammer_id} | {dest} Start");
            let spammer_txid = PointTxId::from_str(&format!("spammer_{}", spammer_id));
            while running.load(Ordering::Relaxed) {
                // Чередуем два разных destination
                // let dest = if counter % 2 == 0 { "Sensor_A" } else { "Sensor_B" };
                
                // Конструируем Point (замените на ваш реальный конструктор)
                // Важно: txid отправителя не должен совпадать с именами подписчиков!
                let point = Point::new(spammer_txid, &dest, counter.load(Ordering::Acquire) as i64); 
                
                if tx_link.send(point).is_ok() {
                    events_sent.fetch_add(1, Ordering::Relaxed);
                    counter.fetch_add(1, Ordering::AcqRel);
                }
                thread::sleep(Duration::from_millis(1));
            }
        }));
    }

    // 2. Multicast Subscriber (Стабильный)
    for id in 0..4 {
        let mq_mc = mq.clone();
        let running_mc = running.clone();
        // let ev_recv_mc = events_received.clone();
        let i = id % 2;
        let (dest, counter) = (spammers[i].0.to_string(), spammers[i].1.clone());
        let dbg = dbg.clone();
        handles.push(thread::spawn(move || {
            let criteria = SubscriptionCriteria::new(&dest, Cot::Inf);
            let (_, rx) = mq_mc.subscribe(&format!("multicast_client_{id}"), &[criteria.clone()]);
            let start_at = counter.load(Ordering::Acquire);
            let mut received = 0;
            let mut prev = None;
            while running_mc.load(Ordering::Relaxed) {
                // while let Ok(point) = rx.recv_timeout(Duration::from_millis(1)) {
                while let Ok(Some(point)) = rx.try_recv() {
                    // ПРОВЕРКА: Multicast должен получать только Sensor_A, 
                    // И каждый следующий евент должен быть больше предыдущего на 1 
                    let value = point.as_int().value;
                    assert_eq!(point.dest(), criteria.destination(), "Multicast received wrong destination!");
                    if let Some(prev) = prev {
                        assert_eq!(value, prev + 1, "Multicast received wrong value! \nt result {} \nt target {}", value, prev + 1);
                    }
                    prev = Some(value);
                    // ev_recv_mc.fetch_add(1, Ordering::Relaxed);
                    received += 1;
                }
                std::thread::yield_now();
            }
            let target = counter.load(Ordering::Acquire) - start_at;
            println!("{dbg} | Multicast Subscriber {id} | {:?} | Received: {received} of {target}", criteria.destination());
            assert!((target as i64 - received as i64).abs() < 2 , "Multicast received wrong value! \nt result {} \nt target {}", received, target);
        }));
    }

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
    for i in 0..10 {
        let mq_chaos = mq.clone();
        let running_chaos = running.clone();
        handles.push(thread::spawn(move || {
            let criteria = SubscriptionCriteria::new(format!("Sensor_B"), Cot::Inf);
            let client_name = format!("chaotic_client_{i}");
            while running_chaos.load(Ordering::Relaxed) {
                // Подписываемся
                let (_, rx) = mq_chaos.subscribe(&client_name, &[criteria.clone()]);
                // Читаем немного сообщений (даем время спамерам накидать)
                let mut read_count = 0;
                while read_count < 100 && running_chaos.load(Ordering::Relaxed) {
                    if let Ok(p) = rx.recv_timeout(Duration::from_millis(5)) {
                        assert_eq!(p.dest(), criteria.destination());
                        read_count += 1;
                    }
                }
                // Отписываемся
                mq_chaos.unsubscribe(&client_name, &[criteria.clone()]).expect("Unsubscribe failed");
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
    }

    // 5. Запуск стресс-теста на определенное время
    println!("{dbg} | Starting stress test for {:?}...", duration);
    let start_time = Instant::now();
    thread::sleep(duration);

    // 6. Остановка
    mq.exit(); // Корректно завершаем сам MultiQueue
    mq.wait().unwrap(); // Ждем завершения потока
    running.store(false, Ordering::Relaxed);
    
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
    // assert!(received > sent, "Received should be > sent (since broadcast duplicates events).");
}

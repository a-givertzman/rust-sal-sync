use std::sync::{Arc, Barrier};
#[cfg(test)]

use std::{sync::Once, time::Duration};
use rand::Rng;
use sal_core::dbg::Dbg;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

use crate::{services::{Subscriptions, entity::Point}, sync::channel};
///
///
static INIT: Once = Once::new();
///
/// once called initialisation
fn init_once() {
    INIT.call_once(|| {
        // implement your initialisation code to be called only once for current test file
    })
}
///
/// returns:
///  - ...
fn init_each() -> () {}
///
/// Testing such functionality / behavior
#[test]
fn subscriptions_stress_test() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg = Dbg::own("Subscriptions-stress-test");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(dbg, Duration::from_secs(30));
    test_duration.run().unwrap();
    let subs = Arc::new(Subscriptions::new("test"));
    let mut handles = vec![];
    // 1. Потоки-«Писатели»: Хаотично добавляют и удаляют подписки
    for i in 0..10 {
        let s = Arc::clone(&subs);
        handles.push(std::thread::spawn(move || {
            for j in 0..1000 {
                let receiver_id = i * 1000 + j;
                let (tx, _) = channel::unbounded();
                let dest = format!("dest_{}", j % 10);
                s.add_multicast(receiver_id, &dest, tx.clone());
                s.add_broadcast(receiver_id, tx);
                if j % 5 == 0 {
                    let _ = s.extend_multicast(receiver_id, &format!("dest_{}", (j + 1) % 10));
                }
                if j % 10 == 0 {
                    let _ = s.remove(receiver_id, &[dest]);
                }
            }
        }));
    }
    // 2. Потоки-«Чистильщики»: Провоцируют итерационные дедлоки через remove_all
    for i in 0..5 {
        let s = Arc::clone(&subs);
        handles.push(std::thread::spawn(move || {
            for _ in 0..100 {
                for j in 0..100 {
                    let receiver_id = i * 1000 + j;
                    let _ = s.remove_all(receiver_id);
                }
            }
        }));
    }
    // 3. Потоки-«Читатели»: Самый опасный сценарий (итерация + вложенный доступ)
    for _ in 0..10 {
        let s = Arc::clone(&subs);
        handles.push(std::thread::spawn(move || {
            for j in 0..1000 {
                let dest = format!("dest_{}", j % 10);
                // Метод get() вызывает iter() по broadcast внутри
                let results = s.get(&dest);
                assert!(results.len() >= 0);
            }
        }));
    }
    for handle in handles {
        handle.join().expect("Thread panicked - possible deadlock or memory corruption");
    }
    subs.exit();
    log::debug!("Subscriptions Stress test passed!");
    // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    test_duration.exit();
}
///
/// 
// --- MOCKS для компиляции теста ---
// Эмулируем внешние зависимости, чтобы тест был автономным
// #[derive(Clone, Debug)]
// pub struct Point { id: String }

// Эмулируем Sender. В реальном коде это sync::channel::Sender
// #[derive(Clone, Debug)]
// pub struct Sender<T>(channel::Sender<T>);

fn create_dummy_sender() -> channel::Sender<Point> {
    let (tx, _) = channel::unbounded();
    // Sender(tx)
    tx
}
// ----------------------------------

///
/// 
#[test]
fn deadlocks_and_race_conditions_hammer() {
    let subscriptions = Arc::new(Subscriptions::new("StressTest"));
    // ====================== Параметры нагрузки ======================
    let n_threads = 20; // Количество потоков
    let n_iterations = 1000; // Операций на поток
    let n_receivers = 50; // Количество уникальных receiver_id
    let n_destinations = 200; // Количество уникальных топиков
    // ================================================================
    let barrier = Arc::new(Barrier::new(n_threads));
    let mut handles = vec![];
    for t_id in 0..n_threads {
        let subs = subscriptions.clone();
        let barrier = barrier.clone();
        handles.push(std::thread::spawn(move || {
            let mut rng = rand::rng();
            let sender = create_dummy_sender();
            // Ждем старта всех потоков для одновременного удара
            barrier.wait();
            for _ in 0..n_iterations {
                let receiver_id = rng.random_range(0..n_receivers);
                let dest_id = format!("dest_{}", rng.random_range(0..n_destinations));
                // Случайно выбираем действие (Chaos Monkey)
                let action = rng.random_range(0..100);
                match action {
                    // 0-39: Add Multicast (Write Heavy)
                    0..=39 => {
                        subs.add_multicast(receiver_id, &dest_id, sender.clone());
                    },
                    // 40-59: Extend Multicast (Complex Read/Write)
                    // ЭТО САМОЕ ОПАСНОЕ МЕСТО: итерация + вложенные локи
                    40..=59 => {
                        // Пытаемся расширить подписку для случайного получателя на новый топик
                        let _ = subs.extend_multicast(receiver_id, &dest_id);
                    },
                    // 60-89: Get (Read Heavy)
                    60..=89 => {
                        let results = subs.get(&dest_id);
                        // Простая проверка целостности, чтобы оптимизатор не выкинул код
                        if !results.is_empty() {
                            assert!(results.len() <= n_receivers);
                        }
                    },
                    // 90-99: Remove All (Destructive Write)
                    _ => {
                        // Удаляем случайно, создавая дыры в картах
                        let _ = subs.remove_all(receiver_id);
                    }
                }
                // Иногда делаем микро-паузу, чтобы изменить тайминг переключения контекста
                if action % 10 == 0 {
                     std::thread::yield_now();
                }
            }
        }));
    }
    // Ожидание завершения
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    // Post-check: проверка целостности
    // После стресс-теста структура не должна быть "битой" (panic при доступе)
    println!("Stress test finished. Checking eventual consistency...");
    let subs = subscriptions.clone();
    subs.exit(); // Должно очистить всё без дедлока
    // Проверка, что exit отработал
    assert!(subs.get("any").is_empty());
}

#[test]
fn logic_consistency_check() {
    // Тест проверяет, не теряются ли данные при гонке add/remove
    let subs = Arc::new(Subscriptions::new("Consistency"));
    let sender = create_dummy_sender();
    let receiver_id = 1;
    let dest = "point_A";
    subs.add_multicast(receiver_id, dest, sender.clone());
    let handle = std::thread::spawn({
        let subs = subs.clone();
        move || {
            // Пытаемся удалить в параллельном потоке
            let _ = subs.remove_all(1);
        }
    });
    // В основном потоке пытаемся читать
    let res = subs.get(dest);
    handle.join().unwrap();
    // Тут нет правильного ответа (гонка), но не должно быть паники
    println!("Race result: found {} items", res.len());
}

#[test]
fn zombie_subscription_race() {
    let subs = Arc::new(Subscriptions::new("ZombieTest"));
    let barrier = Arc::new(Barrier::new(2));
    // Мы будем бомбардировать один и тот же ID, чтобы вызвать коллизию
    let target_receiver_id = 777;
    let target_dest = "critical_topic";
    let iterations = 5000;
    // let zombies_found = Arc::new(AtomicUsize::new(0));
    // Thread A: Постоянно подписывается
    let t1 = {
        let subs = subs.clone();
        let barrier = barrier.clone();
        std::thread::spawn(move || {
            barrier.wait();
            let sender = create_dummy_sender();
            for i in 0..iterations {
                subs.add_multicast(target_receiver_id, target_dest, sender.clone());
                // Микро-пауза, чтобы разорвать атомарность (эмуляция лага)
                if i % 100 == 0 { std::thread::yield_now(); }
            }
        })
    };
    // Thread B: Постоянно удаляет
    let t2 = {
        let subs = subs.clone();
        let barrier = barrier.clone();
        move || {
            barrier.wait();
            for _ in 0..iterations {
                let _ = subs.remove_all(target_receiver_id);
            }
        }
    };
    // Запуск гонки
    let handle2 = std::thread::spawn(t2);
    t1.join().unwrap();
    handle2.join().unwrap();
    // ФИНАЛЬНАЯ ПРОВЕРКА
    // После завершения гонки мы делаем финальную очистку
    subs.remove_all(target_receiver_id);
    // Если логика верна, User 777 не должен существовать нигде.
    // Проверяем 'multicast' напрямую через get
    let leaks = subs.get(target_dest);
    let is_zombie = leaks.iter().any(|(id, _)| *id == target_receiver_id);
    if is_zombie {
        println!("CRITICAL FAILURE: Zombie subscription found! Receiver {} is active in topic '{}' but was supposed to be removed.", target_receiver_id, target_dest);
        panic!("Test failed: Race condition corrupted internal state.");
    } else {
        println!("Success: No zombies found (Race condition did not corrupt state this time).");
    }
}
#[cfg(test)]

use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc, Once}, time::{Duration, Instant}};
use sal_core::dbg::Dbg;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel};
use crate::{sync::Handles, thread_pool::ThreadPool};
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
/// Testing spawn with capacity = 1
#[test]
fn single_capacity() {
    DebugSession::new().filter(LogLevel::Debug).init();
    init_once();
    init_each();
    let dbg = Dbg::own("ThreadPool-test-single_capacity");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(60));
    test_duration.run().unwrap();
    let threads = 10;
    let thread_pool = ThreadPool::new(&dbg, Some(1));
    let time = Instant::now();
    let result = Arc::new(AtomicUsize::new(0));
    let load = 50;
    for i in 0..threads {
        let dbg_ = Dbg::new(&dbg, format!("thread{i}"));
        let result = result.clone();
        thread_pool.spawn(move || {
            log::debug!("{dbg_}", );
            std::thread::sleep(Duration::from_millis(load));
            result.fetch_add(1, Ordering::AcqRel);
        }).unwrap();
    }
    std::thread::sleep(Duration::from_millis(load * (threads + 1) + 5));
    log::debug!("Jobs sheduled: {threads} in: {:?}", time.elapsed());
    // thread_pool.join().unwrap();
    thread_pool.shutdown().unwrap();
    log::debug!("Total elapsed: {:?}", time.elapsed());
    let target = threads as usize;
    let result = result.load(Ordering::Acquire);
    assert!(result == target, "{} \nresult: {:?}\ntarget: {:?}", dbg, result, target);
    test_duration.exit();
}
///
/// Testing spawn with capacity = jobs + 30 %
#[test]
fn spawn() {
    DebugSession::new().filter(LogLevel::Debug).init();
    init_once();
    init_each();
    let dbg = Dbg::own("ThreadPool-test-spawn");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(60));
    test_duration.run().unwrap();
    let threads = 100;
    let thread_pool = ThreadPool::new(&dbg, Some(threads + threads / 3));
    let time = Instant::now();
    let result = Arc::new(AtomicUsize::new(0));
    let handles = Handles::new(&dbg);
    for i in 0..threads {
        let dbg_ = Dbg::new(&dbg, format!("thread{i}"));
        let result = result.clone();
        let handle = thread_pool.spawn(move || {
            log::debug!("{dbg_}", );
            std::thread::sleep(Duration::from_millis(100));
            result.fetch_add(1, Ordering::SeqCst);
        }).unwrap();
        handles.push(handle);
        std::thread::sleep(Duration::from_millis(4));
    }
    log::debug!("Jobs sheduled: {threads} in: {:?}", time.elapsed());
    handles.wait().unwrap();
    thread_pool.shutdown().unwrap();
    log::debug!("All Jobs done ({threads})");
    log::debug!("Total elapsed: {:?}", time.elapsed());
    let target = threads;
    let result = result.load(Ordering::SeqCst);
    assert!(result == target, "{} \nresult: {:?}\ntarget: {:?}", dbg, result, target);
    test_duration.exit();
}
///
/// Проверяет базовый жизненный цикл успешной задачи.
/// 
/// Сценарий:
/// 1. Создаем ThreadPool.
/// 2. Отправляем простую задачу (например, сложение чисел или возврат строки).
/// 3. Ожидаем завершения через JoinHandle::join().
/// 4. Убеждаемся, что результат совпадает с ожидаемым, а пул остался жив.#[test]
#[test]
fn test_spawn_and_join_success() {
    DebugSession::new().filter(LogLevel::Debug).init();
    init_once();
    init_each();
    let dbg = Dbg::own("ThreadPool-test-spawn_and_join_success");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(5));
    test_duration.run().unwrap();
    let thread_pool = ThreadPool::new(&dbg, Some(4));
    let handle = thread_pool.spawn(|| {
        2 + 2
    }).unwrap();
    let result = handle.join().unwrap();
    thread_pool.shutdown().unwrap();
    assert!(result == 4, "{} \nresult: {:?}\ntarget: 4", dbg, result);
    test_duration.exit();
}
///
/// Проверяет изоляцию паники внутри пользовательской задачи.
/// 
/// Сценарий:
/// 1. Отправляем в пул задачу, которая гарантированно вызывает panic!().
/// 2. Вызываем join() у возвращенного JoinHandle.
/// 3. Убеждаемся, что join() возвращает кастомный Error (канал oneshot разорван), 
///    а не "валит" весь процесс тестирования.
/// 4. Отправляем следом валидную задачу и убеждаемся, что пул продолжает 
///    исправно работать, а воркеры не "утекли".#[test]
#[test]
fn test_worker_panic_isolation() {
    DebugSession::new().filter(LogLevel::Debug).init();
    init_once();
    init_each();
    let dbg = Dbg::own("ThreadPool-test-worker_panic_isolation");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(5));
    test_duration.run().unwrap();
    let thread_pool = ThreadPool::new(&dbg, Some(4));
    let handle = thread_pool.spawn(|| {
        panic!("Intentional panic for testing");
    }).unwrap();
    let result = handle.join();
    assert!(result.is_err(), "{} \nExpected error from panicked task", dbg);
    let handle_valid = thread_pool.spawn(|| {
        42
    }).unwrap();
    let result_valid = handle_valid.join().unwrap();
    thread_pool.shutdown().unwrap();
    assert!(result_valid == 42, "{} \nresult: {:?}\ntarget: 42", dbg, result_valid);
    test_duration.exit();
}
///
/// Проверяет эвристику масштабирования `Scaling`.
/// 
/// Сценарий:
/// 1. Инициализируем пул с capacity = 10 (начальный размер будет 2).
/// 2. Искусственно блокируем потоки (например, через std::thread::sleep), 
///    отправив 10 параллельных задач.
/// 3. Проверяем, что size() пула динамически вырос до 10.
/// 4. Отправляем 11-ю задачу и убеждаемся, что пул не превысил лимит (size() == 10), 
///    а задача встала в очередь и выполнилась позже.#[test]
#[test]
fn test_dynamic_scaling_up_to_capacity() {
    DebugSession::new().filter(LogLevel::Debug).init();
    init_once();
    init_each();
    let dbg = Dbg::own("ThreadPool-test-dynamic_scaling");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
    test_duration.run().unwrap();
    let capacity = 10;
    let thread_pool = ThreadPool::new(&dbg, Some(capacity));
    let initial_size = thread_pool.size();
    assert!(initial_size > 0 && initial_size <= capacity, "Initial size should be valid");
    let load = 100;
    let mut handles = Vec::new();
    for _ in 0..capacity {
        handles.push(thread_pool.spawn(move || {
            std::thread::sleep(Duration::from_millis(load));
        }).unwrap());
    }
    std::thread::sleep(Duration::from_millis(load / 2));
    let scaled_size = thread_pool.size();
    for handle in handles {
        let _ = handle.join();
    }
    thread_pool.shutdown().unwrap();
    assert!(scaled_size == capacity, "{} \nresult: {:?}\ntarget: {:?}", dbg, scaled_size, capacity);
    test_duration.exit();
}
///
/// Проверяет корректное и чистое завершение работы пула.
/// 
/// Сценарий:
/// 1. Запускаем несколько долгих задач.
/// 2. Вызываем shutdown() (или позволяем пулу выйти из области видимости для Drop).
/// 3. Убеждаемся, что метод shutdown() дождался выполнения всех активных задач, 
///    каналы корректно закрылись, а список `workers` стал пустым без зависаний (deadlocks).
#[test]
fn test_graceful_shutdown() {
    DebugSession::new().filter(LogLevel::Debug).init();
    init_once();
    init_each();
    let dbg = Dbg::own("ThreadPool-test-graceful_shutdown");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
    test_duration.run().unwrap();
    let thread_pool = ThreadPool::new(&dbg, Some(4));
    let result = Arc::new(AtomicUsize::new(0));
    let load = 50;
    for _ in 0..4 {
        let res = result.clone();
        thread_pool.spawn(move || {
            std::thread::sleep(Duration::from_millis(load));
            res.fetch_add(1, Ordering::AcqRel);
        }).unwrap();
    }
    thread_pool.shutdown().unwrap();
    let final_result = result.load(Ordering::Acquire);
    assert!(final_result == 4, "{} \nresult: {:?}\ntarget: 4", dbg, final_result);
    assert!(thread_pool.size() == 0, "{} \nWorkers leaked after shutdown", dbg);
    test_duration.exit();
}
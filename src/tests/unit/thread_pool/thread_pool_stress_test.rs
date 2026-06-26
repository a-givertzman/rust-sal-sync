#[cfg(test)]
use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc, Once}, time::{Duration, Instant}};
use sal_core::dbg::Dbg;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel};
use crate::thread_pool::ThreadPool;
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
/// Стресс-тест на гонки данных (Data Races) и Cache-line Ping-Pong.
/// 
/// Сценарий:
/// 1. Создаем пул с максимальным capacity.
/// 2. Из 10 разных системных потоков (std::thread) одновременно спавним в пул 
///    по 1000 микро-задач через клонированный `Scheduler`.
/// 3. Каждая задача инкрементирует общий атомарный счетчик (Arc<AtomicUsize>).
/// 4. Дожидаемся завершения всех задач и проверяем, что счетчик равен ровно 10_000.
/// 5. Тест должен проходить быстро, доказывая отсутствие бутылочных горлышек в канале.#[test]
#[test]
fn test_heavy_concurrent_load() {
    DebugSession::new().filter(LogLevel::Debug).init();
    init_once();
    init_each();
    let dbg = Dbg::own("ThreadPool-test-heavy_concurrent_load");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(15));
    test_duration.run().unwrap();
    let thread_pool = ThreadPool::new(&dbg, Some(64));
    let result = Arc::new(AtomicUsize::new(0));
    let threads_count = 10;
    let tasks_per_thread = 1000;
    let mut thread_handles = Vec::new();
    let time = Instant::now();
    for i in 0..threads_count {
        let scheduler = thread_pool.scheduler();
        let res = result.clone();
        let dbg_ = Dbg::new(&dbg, format!("spawner{i}"));
        thread_handles.push(std::thread::spawn(move || {
            log::debug!("{dbg_} | Start scheduling...");
            for _ in 0..tasks_per_thread {
                let r = res.clone();
                scheduler.spawn(move || {
                    r.fetch_add(1, Ordering::AcqRel);
                }).unwrap();
            }
        }));
    }
    for handle in thread_handles {
        handle.join().unwrap();
    }
    log::debug!("All jobs scheduled in: {:?}", time.elapsed());
    thread_pool.shutdown().unwrap();
    log::debug!("Total elapsed: {:?}", time.elapsed());
    let target = threads_count * tasks_per_thread;
    let final_result = result.load(Ordering::Acquire);
    assert!(final_result == target, "{} \nresult: {:?}\ntarget: {:?}", dbg, final_result, target);
    test_duration.exit();
}
///
/// Проверяет защиту от отправки задач в "мертвый" пул.
/// 
/// Сценарий:
/// 1. Получаем экземпляр `Scheduler`.
/// 2. Принудительно вызываем `ThreadPool::shutdown()`.
/// 3. Пытаемся отправить новую задачу через изолированный `Scheduler::spawn()`.
/// 4. Убеждаемся, что метод возвращает Err("ThreadPool is shutting down") 
///    и не вызывает панику при попытке записи в закрытый канал.#[test]
#[test]
fn test_scheduler_spawn_after_shutdown() {
    DebugSession::new().filter(LogLevel::Debug).init();
    init_once();
    init_each();
    let dbg = Dbg::own("ThreadPool-test-scheduler_spawn_after_shutdown");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(5));
    test_duration.run().unwrap();
    let thread_pool = ThreadPool::new(&dbg, Some(4));
    let scheduler = thread_pool.scheduler();
    thread_pool.shutdown().unwrap();
    let spawn_result = scheduler.spawn(|| { 
        2 + 2 
    });
    assert!(spawn_result.is_err(), "{} \nExpected error when spawning to a shutdown pool", dbg);
    if let Err(err) = spawn_result {
        let err_msg = format!("{:?}", err);
        assert!(err_msg.contains("ThreadPool is shutting down"), "{} \nUnexpected error message: {}", dbg, err_msg);
    }
    test_duration.exit();
}
///
/// Проверяет защиту от некорректных параметров инициализации.
/// 
/// Сценарий:
/// 1. Инициализируем пул, передав `capacity: Some(0)`.
/// 2. Убеждаемся, что пул перехватил это значение и установил дефолтную 
///    вместимость (64), а не запаниковал при делении на ноль или создании 0 воркеров.#[test]
#[test]
fn test_zero_capacity_fallback() {
    DebugSession::new().filter(LogLevel::Debug).init();
    init_once();
    init_each();
    let dbg = Dbg::own("ThreadPool-test-zero_capacity_fallback");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(5));
    test_duration.run().unwrap();
    let thread_pool = ThreadPool::new(&dbg, Some(0));
    let capacity = thread_pool.capacity();
    let target = 64;
    assert!(capacity == target, "{} \nCapacity did not fallback to default. result: {:?}\ntarget: {:?}", dbg, capacity, target);
    thread_pool.shutdown().unwrap();
    test_duration.exit();
}
///
/// Доказывает, что задачи действительно выполняются параллельно, а не последовательно.
/// 
/// Сценарий:
/// 1. Инициализируем пул.
/// 2. Отправляем 4 задачи, каждая из которых засыпает ровно на 100 мс.
/// 3. Замеряем общее время выполнения пакета задач.
/// 4. Убеждаемся, что общее время выполнения составило ~100 мс (с небольшой 
///    погрешностью на оверхед), а не 400 мс.#[test]
#[test]
fn test_parallel_execution() {
    DebugSession::new().filter(LogLevel::Debug).init();
    init_once();
    init_each();
    let dbg = Dbg::own("ThreadPool-test-parallel_execution");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(5));
    test_duration.run().unwrap();
    let threads = 4;
    let thread_pool = ThreadPool::new(&dbg, Some(threads));
    let load_time_ms = 100;
    let time = Instant::now();
    let mut handles = Vec::new();
    for _ in 0..threads {
        handles.push(thread_pool.spawn(move || {
            std::thread::sleep(Duration::from_millis(load_time_ms));
        }).unwrap());
    }
    for handle in handles {
        let _ = handle.join();
    }
    let elapsed = time.elapsed().as_millis();
    thread_pool.shutdown().unwrap();
    assert!(elapsed < (load_time_ms * 2) as u128, "{} \nExecution was not parallel. Elapsed: {}ms", dbg, elapsed);
    test_duration.exit();
}

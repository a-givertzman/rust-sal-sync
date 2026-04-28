use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[cfg(test)]
use debugging::session::debug_session::{DebugSession, LogLevel};
use sal_core::dbg::Dbg;

use crate::kernel::state::ExitNotify;

///
/// 
#[test]
fn test_default_state() {
    DebugSession::new().filter(LogLevel::Debug).init();
    let dbg = Dbg::own("ExitNotify.test_default_state");
    log::info!("{}", dbg);
    let notify = ExitNotify::new(&dbg, None, None);
    // Все флаги false при создании
    assert_eq!(notify.get(), false, "{dbg} | \nresult: {}, \target: {}", notify.get(), false);
}
///
/// 
#[test]
fn test_local_exit_only() {
    DebugSession::new().filter(LogLevel::Debug).init();
    let dbg = Dbg::own("ExitNotify.test_local_exit_only");
    log::info!("{}", dbg);
    let notify = ExitNotify::new(&dbg, None, None);
    notify.exit();
    assert_eq!(notify.get(), true, "{dbg} | Локальный exit должен переводить get() в true");
    notify.reset();
    assert_eq!(notify.get(), false, "{dbg} | После reset() должен быть false");
}
///
/// 
#[test]
fn test_signals_propagation() {
    DebugSession::new().filter(LogLevel::Debug).init();
    let dbg = Dbg::own("ExitNotify.test_signals_propagation");
    log::info!("{}", dbg);
    let parent_flag = Arc::new(AtomicBool::new(false));
    let partner_flag = Arc::new(AtomicBool::new(false));
    let notify = ExitNotify::new(
        &dbg, 
        Some(parent_flag.clone()), 
        Some(partner_flag.clone())
    );
    // 1. Сигнал от родителя
    parent_flag.store(true, Ordering::Release);
    assert_eq!(notify.get(), true, "{dbg} | Должен реагировать на родительский флаг");
    parent_flag.store(false, Ordering::Release);
    assert_eq!(notify.get(), false);
    // 2. Сигнал от партнера
    partner_flag.store(true, Ordering::Release);
    assert_eq!(notify.get(), true, "{dbg} | Должен реагировать на флаг партнера");
}
///
/// 
#[test]
fn test_exit_all_affects_only_local_and_pair() {
    DebugSession::new().filter(LogLevel::Debug).init();
    let dbg = Dbg::own("ExitNotify.test_exit_all_affects_only_local_and_pair");
    log::info!("{}", dbg);
    let parent_flag = Arc::new(AtomicBool::new(false));
    let partner_flag = Arc::new(AtomicBool::new(false));
    let notify = ExitNotify::new(
        &dbg, 
        Some(parent_flag.clone()), 
        Some(partner_flag.clone())
    );
    notify.exit_all();
    // exit_all должен зажечь локальный флаг и флаг партнера
    assert_eq!(partner_flag.load(Ordering::Acquire), true, "{dbg} | Флаг партнера должен стать true");
    assert_eq!(notify.get(), true);
    // Родительский флаг при этом затронут быть не должен
    assert_eq!(parent_flag.load(Ordering::Acquire), false, "{dbg} | Родительский флаг не должен меняться");
}
///
/// 
#[test]
fn test_multithreaded_stress() {
    DebugSession::new().filter(LogLevel::Debug).init();
    let dbg = Dbg::own("ExitNotify.test_multithreaded_stress");
    log::info!("{}", dbg);
    // Создаем внешние флаги для родителя и партнера
    let parent_flag = Arc::new(AtomicBool::new(false));
    let partner_flag = Arc::new(AtomicBool::new(false));
    // Инициализируем структуру в Arc, чтобы прокинуть в потоки
    let notify = Arc::new(ExitNotify::new(
        &dbg,
        Some(parent_flag.clone()),
        Some(partner_flag.clone()),
    ));
    let mut handles = vec![];
    let thread_count = 100; // Количество потоков-мучителей
    let iterations = 10_000; // Количество итераций на поток
    for i in 0..thread_count {
        let n = Arc::clone(&notify);
        let p_parent = Arc::clone(&parent_flag);
        let p_partner = Arc::clone(&partner_flag);
        let handle = std::thread::spawn(move || {
            for j in 0..iterations {
                // Изображаем хаотичную бурную деятельность
                match (i + j) % 6 {
                    0 => { n.exit(); }
                    1 => { n.exit_pair(); }
                    2 => { n.exit_all(); }
                    3 => { n.reset(); }
                    4 => { n.reset_pair(); }
                    5 => {
                        // Потоки напрямую дергают внешние зависимости
                        p_parent.store(j % 2 == 0, Ordering::Release);
                        p_partner.store(j % 3 == 0, Ordering::Release);
                    }
                    _ => unreachable!(),
                }
                // Параллельно постоянно читаем состояние
                let _ = n.get();
            }
        });
        handles.push(handle);
    }
    // Ждем завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }
    // Финальная проверка: сбрасываем всё и смотрим, нет ли зависших флагов
    notify.reset_all();
    parent_flag.store(false, Ordering::Release);
    partner_flag.store(false, Ordering::Release);
    assert_eq!(notify.get(), false, "{dbg} | После жесткого сброса всех флагов метод get() должен вернуть false!");
}

#[cfg(test)]

mod change_notify {
    use std::{cell::RefCell, rc::Rc, sync::Once};
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use sal_core::dbg::Dbg;
    use crate::kernel::state::change_notify::ChangeNotify;
    ///
    ///
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    enum NotifyState {
        Start,
        Online,
        Offline,
        Stop,
    }
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
    /// returns tuple(
    ///     - initialState: ProcessState
    ///     - switches: Vec<Switch<ProcessState, u8>>
    /// )
    fn init_each() -> () {}
    ///
    ///
    #[test]
    fn add() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("change_notify.add");
        log::info!("{}", dbg);
        let test_data = vec![
            (3, NotifyState::Stop,    None),
            (0, NotifyState::Start,   Some("Start")),
            (1, NotifyState::Online,  Some("Online")),
            (1, NotifyState::Online,  None),
            (2, NotifyState::Offline, Some("Offline")),
            (2, NotifyState::Offline, None),
            (3, NotifyState::Stop,    Some("Stop")),
        ];
        let mut notify: ChangeNotify<NotifyState, (Rc<RefCell<Option<String>>>, &str)> = ChangeNotify::new(
            &dbg,
            NotifyState::Stop,
            vec![
                (NotifyState::Start,   Box::new(|(result, message)| *result.borrow_mut() = Some(format!("{} Start", message)))),
                (NotifyState::Online,  Box::new(|(result, message)| *result.borrow_mut() = Some(format!("{} Online", message)))),
                (NotifyState::Offline, Box::new(|(result, message)| *result.borrow_mut() = Some(format!("{} Offline", message)))),
                (NotifyState::Stop,    Box::new(|(result, message)| *result.borrow_mut() = Some(format!("{} Stop", message)))),
                ],
            );
            for (step, value, target) in test_data {
            let result = Rc::new(RefCell::new(None));
            let message = "Switched to";
            notify.add(value, (result.clone(), message));
            let result = result.borrow().clone();
            log::debug!("{} | Step {} | value: {:?}  message  |  result: {:?}", dbg, step, value, result);
            let target = target.map(|target| format!("{} {}", message, target));
            assert!(result == target, "{} | Step {} \nresult: {:?} \ntarget: {:?} ", dbg, step, result, target);
        }
    }
}

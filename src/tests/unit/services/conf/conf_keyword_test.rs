#[cfg(test)]
mod conf_keywd {
    use std::{sync::Once, str::FromStr};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::services::conf::{ConfKeywd, ConfKind};
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
    ///
    #[test]
    fn from_str() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("from_str");
        // let (initial, switches) = init_each();
        let test_data = vec![
            // input                            prefix,     kind,               name,           sufix: 
            ("service ApiClient",               ("",        ConfKind::Service,  "ApiClient",    "" )),
            ("service ApiClient ApiClient1",    ("",        ConfKind::Service,  "ApiClient",    "ApiClient1" )),
            ("service MultiQueue",              ("",        ConfKind::Service,  "MultiQueue",   "")),
            ("task Task1",                      ("",        ConfKind::Task,     "Task1",        "")),
            ("task task1",                      ("",        ConfKind::Task,     "task1",        "")),
            ("in queue queue1",                 ("in",      ConfKind::Queue,    "queue1",       "")),
            ("in link link",                    ("in",      ConfKind::Link,     "link",         "")),
            ("in queue in-queue",               ("in",      ConfKind::Queue,    "in-queue",     "")),
            ("out queue out-queue",             ("out",     ConfKind::Queue,    "out-queue",    "")),
        ];
        for (value, target) in test_data {
            let result = ConfKeywd::from_str(value).unwrap();
            log::debug!("value: {:?}   |   ConfKind: {:?}   |   target: {:?}", value, result, target);
            assert!(result.prefix() == target.0, "\nresult: {:?}\ntarget: {:?}", result, target.0);
            assert!(result.kind() == target.1.to_string(), "\nresult: {:?}\ntarget: {:?}", result, target.1);
            assert!(result.name() == target.2, "\nresult: {:?}\ntarget: {:?}", result, target.2);
            assert!(result.sufix() == target.3, "\nresult: {:?}\ntarget: {:?}", result, target.3);
            assert!(result.prefix == target.0, "\nresult: {:?}\ntarget: {:?}", result, target.0);
            assert!(result.kind == target.1.to_string(), "\nresult: {:?}\ntarget: {:?}", result, target.1);
            assert!(result.name == target.2, "\nresult: {:?}\ntarget: {:?}", result, target.2);
            assert!(result.sufix == target.3, "\nresult: {:?}\ntarget: {:?}", result, target.3);
        }
    }

    // #[test]
    // fn test_create_invalid() {
    //     DebugSession::init(LogLevel::Info, Backtrace::Short);
    //     init_once();
    //     init_each();
    //     info!("test_create_invalid");
    //     // let (initial, switches) = init_each();
    //     let test_data: Vec<(&str, Result<&str, ()>)> = vec![
    //         ("fn:name", Err(())),
    //         ("fn\nname", Err(())),
    //         ("fn: name", Err(())),
    //         ("fn :name", Err(())),
    //         ("fn : name", Err(())),
    //         ("Fn name", Err(())),
    //         ("FN name", Err(())),
    //         ("fnName", Err(())),
    //         ("fn_name", Err(())),
    //         ("let:name", Err(())),
    //         ("Let name", Err(())),
    //         ("LET name", Err(())),
    //         ("letName", Err(())),
    //         ("let_name", Err(())),
    //         ("const:name", Err(())),
    //         ("Const name", Err(())),
    //         ("CONST name", Err(())),
    //         ("constName", Err(())),
    //         ("const_name", Err(())),
    //         ("point:name", Err(())),
    //         ("Point name", Err(())),
    //         ("POINT name", Err(())),
    //         ("pointName", Err(())),
    //         ("point_name", Err(())),
    //     ];
    //     for (value, target) in test_data {
    //         let fnConfigType = ConfKeywd::from_str(value);
    //         debug!("value: {:?}   |   fnConfigType: {:?}   |   target: {:?}", value, fnConfigType, target);
    //         assert_eq!(fnConfigType.is_err(), true);
    //     }
    // }
}

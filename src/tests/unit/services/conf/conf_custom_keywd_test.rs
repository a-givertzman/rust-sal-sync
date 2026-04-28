#[cfg(test)]
use std::{sync::Once, str::FromStr};
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
use crate::services::conf::ConfCustomKeywd;
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
    DebugSession::new().filter(LogLevel::Info).init();
    init_once();
    init_each();
    log::debug!("ConfCustomKeyed-from_str");
    // let (initial, switches) = init_each();
    let test_data = vec![
        // input                         prefix,     name,           title
        (01, "camera Camera1",              ("",        "camera",       "Camera1")),
        (02, "camera",                      ("",        "camera",       "")),
        (03, "task Task1",                  ("",        "task",         "Task1")),
        (04, "task",                        ("",        "task",         "")),
        (05, "in queue Queue1",             ("in",      "queue",        "Queue1")),
        (06, "in link Link1",               ("in",      "link",         "Link1")),
        (07, "in queue in-queue",           ("in",      "queue",        "in-queue")),
        (08, "out queue out-queue",         ("out",     "queue",        "out-queue")),
    ];
    for (step, value, (target_prefix, target_name, target_title)) in test_data {
        let result = ConfCustomKeywd::from_str(value).unwrap();
        let target = ConfCustomKeywd::new(target_prefix, target_name, target_title);
        log::debug!("step {step}  value: {:?}:\n\tresult: {:?}\n\ttarget: {:?}", value, result, target);
        assert!(result == target, "step {step} \nresult: {:?}\ntarget: {:?}", result, target);
        assert!(result.prefix() == target_prefix, "step {step} \nresult: {:?}\ntarget: {:?}", result, target_prefix);
        assert!(result.name() == target_name, "step {step} \nresult: {:?}\ntarget: {:?}", result, target_name);
        assert!(result.title() == target_title, "step {step} \nresult: {:?}\ntarget: {:?}", result, target_title);
    }
}

// #[test]
// fn test_create_invalid() {
//     DebugSession::new().filter(LogLevel::Info).init();
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

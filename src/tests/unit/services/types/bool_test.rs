#[cfg(test)]

mod bool {
    use log::debug;
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::services::types::Bool;
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
    /// Testing Bool::add
    #[test]
    fn test_add() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (01, true, true, true),
            (02, true, false, true),
            (03, false, true, true),
            (04, false, false, false),
        ];
        for (step, value1, value2, target) in test_data {
            let result = Bool(value1) + Bool(value2);
            debug!("step: {}  |  value1: {}, value2: {}, target: {}, result: {}", step, value1, value2, target, result);
            assert!(result.0 == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
    ///
    /// Testing Bool::mul
    #[test]
    fn test_mul() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (01, true, true, true),
            (02, true, false, false),
            (03, false, true, false),
            (04, false, false, false),
        ];
        for (step, value1, value2, target) in test_data {
            let result = Bool(value1) * Bool(value2);
            debug!("step: {}  |  value1: {}, value2: {}, target: {}, result: {}", step, value1, value2, target, result);
            assert!(result.0 == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
    ///
    /// Testing Bool::BitOr
    #[test]
    fn test_bit_or() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (01, true, true, true),
            (02, true, false, true),
            (03, false, true, true),
            (04, false, false, false),
        ];
        for (step, value1, value2, target) in test_data {
            let result = Bool(value1) | Bool(value2);
            debug!("step: {}  |  value1: {}, value2: {}, target: {}, result: {}", step, value1, value2, target, result);
            assert!(result.0 == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
    ///
    /// Testing Bool::BitAnd
    #[test]
    fn test_bit_and() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (01, true, true, true),
            (02, true, false, false),
            (03, false, true, false),
            (04, false, false, false),
        ];
        for (step, value1, value2, target) in test_data {
            let result = Bool(value1) & Bool(value2);
            debug!("step: {}  |  value1: {}, value2: {}, target: {}, result: {}", step, value1, value2, target, result);
            assert!(result.0 == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}

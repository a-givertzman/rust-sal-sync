#[cfg(test)]

mod cot {
    use std::{sync::Once, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use log::debug;
    use testing::stuff::max_test_duration::TestDuration;
    use crate::services::entity::cot::Cot;
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
    /// Testing Cot::contains
    #[test]
    fn contains() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "cot_test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            // match
            (true, Cot::Inf, Cot::Read),
            (true, Cot::Act, Cot::Write),
            (true, Cot::ActCon, Cot::Read),
            (true, Cot::ActErr, Cot::Read),
            (true, Cot::Req, Cot::Write),
            (true, Cot::ReqCon, Cot::Read),
            (true, Cot::ReqErr, Cot::Read),
            // not match
            (false, Cot::Inf, Cot::Write),
            (false, Cot::Act, Cot::Read),
            (false, Cot::ActCon, Cot::Write),
            (false, Cot::ActErr, Cot::Write),
            (false, Cot::Req, Cot::Read),
            (false, Cot::ReqCon, Cot::Write),
            (false, Cot::ReqErr, Cot::Write),
        ];
        for (target, left, right) in test_data {
            let result = left & right;
            println!("cot: {:?}, direction: {:?} | result: {}", left, right, result);
            println!("left: {:#08b}({:?}), right: {:#08b}({:?}) | result: {:#08b}({:?})", left, left, right, right, result, result);
            assert!((result > 0) == target, "\nresult: {:?}\ntarget: {:?}", result, (left as u32) & (right as u32));
            assert!(right.contains(left) == target, "\nresult: {:?}\ntarget: {:?}", result, (left as u32) & (right as u32));
            let result = left & Cot::All;
            assert!((result > 0) == true, "\nresult: {:?}\ntarget: {:?}", result, (left as u32) & (Cot::All as u32));
        }
        test_duration.exit();
    }
    ///
    /// Testing Cot::contains
    #[test]
    fn default() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "cot_test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let result = Cot::default();
        let target = Cot::Inf;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
    ///
    /// Testing Cot::contains
    #[test]
    fn as_str() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "cot_test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (01, Cot::Inf, "Inf"),
            (01, Cot::Act, "Act"),
            (01, Cot::ActCon, "ActCon"),
            (01, Cot::ActErr, "ActErr"),
            (01, Cot::Req, "Req"),
            (01, Cot::ReqCon, "ReqCon"),
            (01, Cot::ReqErr, "ReqErr"),
            (01, Cot::Read, "Read"),
            (01, Cot::Write, "Write"),
            (01, Cot::All, ""),
        ];
        for (step, value, target) in test_data {
            let result = value.as_str();
            debug!("step: {}  |  cot: {:?}, result: {}", step, value, result);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        test_duration.exit();
    }
}

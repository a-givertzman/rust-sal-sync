#[cfg(test)]

mod point_hlr {
    use log::debug;
    use std::{sync::Once, time::Duration};
    use testing::{entities::test_value::Value, stuff::max_test_duration::TestDuration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::services::{entity::{cot::Cot, point::point_hlr::PointHlr, status::status::Status}, types::bool::Bool};
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
    /// Testing PointHlr::new
    #[test]
    fn new() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "test";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point1", Value::Int(100i64), Status::Invalid, Cot::default(), chrono::Utc::now()),
            (02, 02, "/App/Service/Point2", Value::Int(200i64), Status::Obsolete, Cot::default(), chrono::Utc::now()),
            (03, 03, "/App/Service/Point3", Value::Real(300.0f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (04, 04, "/App/Service/Point4", Value::Double(300.0f64), Status::TimeInvalid, Cot::default(), chrono::Utc::now()),
        ];
        for (step, tx_id, name, value, status, cot, timestamp) in test_data {
            match value {
                Value::Bool(value) => {
                    let result = PointHlr::new(tx_id, &name, value, status, cot, timestamp);
                    let target = PointHlr { tx_id, name: name.to_owned(), value, status, cot, timestamp };
                    assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::Int(value) => {
                    let result = PointHlr::new(tx_id, &name, value, status, cot, timestamp);
                    let target = PointHlr { tx_id, name: name.to_owned(), value, status, cot, timestamp };
                    assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::Real(value) => {
                    let result = PointHlr::new(tx_id, &name, value, status, cot, timestamp);
                    let target = PointHlr { tx_id, name: name.to_owned(), value, status, cot, timestamp };
                    assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::Double(value) => {
                    let result = PointHlr::new(tx_id, &name, value, status, cot, timestamp);
                    let target = PointHlr { tx_id, name: name.to_owned(), value, status, cot, timestamp };
                    assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::String(value) => {
                    let result = PointHlr::new(tx_id, &name, value.clone(), status, cot, timestamp);
                    let target = PointHlr { tx_id, name: name.to_owned(), value, status, cot, timestamp };
                    assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
            };
        }
        test_duration.exit();
    }
    ///
    /// Testing PointHlr::new
    #[test]
    fn new_bool() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "test";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point1", true),
            (02, 02, "/App/Service/Point2", true),
            (03, 03, "/App/Service/Point3", false),
            (04, 04, "/App/Service/Point4", false),
        ];
        for (step, tx_id, name, value) in test_data {
            let result = PointHlr::new_bool(tx_id, &name, value);
            let target = PointHlr { tx_id, name: name.to_owned(), value: Bool(value), status: Status::Ok, cot: Cot::Inf, timestamp: chrono::Utc::now() };
            assert!(result.tx_id == target.tx_id, "step {} \nresult: {:?}\ntarget: {:?}", step, result.tx_id, target.tx_id);
            assert!(result.name == target.name, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name, target.name);
            assert!(result.value == target.value, "step {} \nresult: {:?}\ntarget: {:?}", step, result.value, target.value);
        }
        test_duration.exit();
    }
}

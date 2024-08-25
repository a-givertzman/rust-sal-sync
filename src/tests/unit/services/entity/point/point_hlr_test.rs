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
        let self_id = "new";
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
    /// Testing PointHlr::new_bool
    #[test]
    fn new_bool() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "new_bool";
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
    ///
    /// Testing PointHlr::new_int
    #[test]
    fn new_int() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "new_int";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point1", i64::MIN),
            (02, 02, "/App/Service/Point2", -2),
            (03, 03, "/App/Service/Point3", -1),
            (04, 04, "/App/Service/Point4", 0),
            (05, 05, "/App/Service/Point5", 1),
            (06, 06, "/App/Service/Point6", 2),
            (07, 07, "/App/Service/Point7", i64::MAX),
        ];
        for (step, tx_id, name, value) in test_data {
            let result = PointHlr::new_int(tx_id, &name, value);
            let target = PointHlr { tx_id, name: name.to_owned(), value: value, status: Status::Ok, cot: Cot::Inf, timestamp: chrono::Utc::now() };
            assert!(result.tx_id == target.tx_id, "step {} \nresult: {:?}\ntarget: {:?}", step, result.tx_id, target.tx_id);
            assert!(result.name == target.name, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name, target.name);
            assert!(result.value == target.value, "step {} \nresult: {:?}\ntarget: {:?}", step, result.value, target.value);
        }
        test_duration.exit();
    }
    ///
    /// Testing PointHlr::new_real
    #[test]
    fn new_real() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "new_real";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point1", f32::MIN),
            (02, 02, "/App/Service/Point2", -2.0),
            (03, 03, "/App/Service/Point3", -1.0),
            (04, 04, "/App/Service/Point4", -0.1),
            (05, 05, "/App/Service/Point5", 0.0),
            (06, 06, "/App/Service/Point6", 0.1),
            (07, 07, "/App/Service/Point7", 1.0),
            (08, 08, "/App/Service/Point8", 2.0),
            (09, 09, "/App/Service/Point9", f32::MAX),
        ];
        for (step, tx_id, name, value) in test_data {
            let result = PointHlr::new_real(tx_id, &name, value);
            let target = PointHlr { tx_id, name: name.to_owned(), value: value, status: Status::Ok, cot: Cot::Inf, timestamp: chrono::Utc::now() };
            assert!(result.tx_id == target.tx_id, "step {} \nresult: {:?}\ntarget: {:?}", step, result.tx_id, target.tx_id);
            assert!(result.name == target.name, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name, target.name);
            assert!(result.value == target.value, "step {} \nresult: {:?}\ntarget: {:?}", step, result.value, target.value);
        }
        test_duration.exit();
    }
    ///
    /// Testing PointHlr::new_double
    #[test]
    fn new_double() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "new_double";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point1", f64::MIN),
            (02, 02, "/App/Service/Point2", -2.0),
            (03, 03, "/App/Service/Point3", -1.0),
            (04, 04, "/App/Service/Point4", -0.1),
            (05, 05, "/App/Service/Point5", 0.0),
            (06, 06, "/App/Service/Point6", 0.1),
            (07, 07, "/App/Service/Point7", 1.0),
            (08, 08, "/App/Service/Point8", 2.0),
            (09, 09, "/App/Service/Point9", f64::MAX),
        ];
        for (step, tx_id, name, value) in test_data {
            let result = PointHlr::new_double(tx_id, &name, value);
            let target = PointHlr { tx_id, name: name.to_owned(), value: value, status: Status::Ok, cot: Cot::Inf, timestamp: chrono::Utc::now() };
            assert!(result.tx_id == target.tx_id, "step {} \nresult: {:?}\ntarget: {:?}", step, result.tx_id, target.tx_id);
            assert!(result.name == target.name, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name, target.name);
            assert!(result.value == target.value, "step {} \nresult: {:?}\ntarget: {:?}", step, result.value, target.value);
        }
        test_duration.exit();
    }
    ///
    /// Testing PointHlr::new_string
    #[test]
    fn new_string() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "new_string";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point1", "/App/Service/Point1"),
            (02, 02, "/App/Service/Point2", "/App/Service/Point2"),
            (03, 03, "/App/Service/Point3", "/App/Service/Point3"),
            (04, 04, "/App/Service/Point4", "/App/Service/Point4"),
            (04, 04, "/App/Service/Point4", "/App/Service/Point4"),
            (04, 04, "/App/Service/Point4", "/App/Service/Point4"),
            (05, 05, "/App/Service/Point5", "/App/Service/Point5"),
            (06, 06, "/App/Service/Point6", "/App/Service/Point6"),
            (07, 07, "/App/Service/Point7", "/App/Service/Point7"),
        ];
        for (step, tx_id, name, value) in test_data {
            let result = PointHlr::new_string(tx_id, &name, value);
            let target = PointHlr { tx_id, name: name.to_owned(), value: value, status: Status::Ok, cot: Cot::Inf, timestamp: chrono::Utc::now() };
            assert!(result.tx_id == target.tx_id, "step {} \nresult: {:?}\ntarget: {:?}", step, result.tx_id, target.tx_id);
            assert!(result.name == target.name, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name, target.name);
            assert!(result.value == target.value, "step {} \nresult: {:?}\ntarget: {:?}", step, result.value, target.value);
        }
        test_duration.exit();
    }
    ///
    /// Testing PointHlr::to_bool
    #[test]
    fn to_bool() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "to_bool";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point1", Value::Bool(false), Status::Invalid, Cot::default(), chrono::Utc::now()),
            (02, 02, "/App/Service/Point2", Value::Bool(true), Status::Invalid, Cot::default(), chrono::Utc::now()),
            (03, 03, "/App/Service/Point3", Value::Int(100i64), Status::Invalid, Cot::default(), chrono::Utc::now()),
            (04, 04, "/App/Service/Point4", Value::Int(200i64), Status::Obsolete, Cot::default(), chrono::Utc::now()),
            (05, 05, "/App/Service/Point5", Value::Real(300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (06, 06, "/App/Service/Point6", Value::Double(300.2f64), Status::TimeInvalid, Cot::default(), chrono::Utc::now()),
            // (07, 07, "/App/Service/Point7", Value::String("/App/Service/Point7".to_owned()), Status::Ok, Cot::default(), chrono::Utc::now()),
        ];
        for (step, tx_id, name, value, status, cot, timestamp) in test_data {
            match value {
                Value::Bool(value) => {
                    let result = PointHlr::new(tx_id, &name, Bool(value), status, cot, timestamp);
                    let target = PointHlr { tx_id, name: name.to_owned(), value: Bool(value), status, cot, timestamp };
                    assert!(result.to_bool() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::Int(value) => {
                    let result = PointHlr::new(tx_id, &name, value, status, cot, timestamp);
                    let target = PointHlr { tx_id, name: name.to_owned(), value: Bool(value > 0), status, cot, timestamp };
                    assert!(result.to_bool() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::Real(value) => {
                    let result = PointHlr::new(tx_id, &name, value, status, cot, timestamp);
                    let target = PointHlr { tx_id, name: name.to_owned(), value: Bool(value > 0.0), status, cot, timestamp };
                    assert!(result.to_bool() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::Double(value) => {
                    let result = PointHlr::new(tx_id, &name, value, status, cot, timestamp);
                    let target = PointHlr { tx_id, name: name.to_owned(), value: Bool(value > 0.0), status, cot, timestamp };
                    assert!(result.to_bool() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::String(_) => {}
            };
        }
        test_duration.exit();
    }
    ///
    /// Testing PointHlr::to_int
    #[test]
    fn to_int() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "to_int";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point1", Value::Bool(false), Status::Invalid, Cot::default(), chrono::Utc::now()),
            (02, 02, "/App/Service/Point2", Value::Bool(true), Status::Invalid, Cot::default(), chrono::Utc::now()),
            (03, 03, "/App/Service/Point3", Value::Int(100i64), Status::Invalid, Cot::default(), chrono::Utc::now()),
            (04, 04, "/App/Service/Point4", Value::Int(200i64), Status::Obsolete, Cot::default(), chrono::Utc::now()),
            (05, 05, "/App/Service/Point5", Value::Real(300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (06, 06, "/App/Service/Point6", Value::Double(300.2f64), Status::TimeInvalid, Cot::default(), chrono::Utc::now()),
            (07, 07, "/App/Service/Point7", Value::String("101".to_owned()), Status::Ok, Cot::default(), chrono::Utc::now()),
        ];
        for (step, tx_id, name, value, status, cot, timestamp) in test_data {
            match value {
                Value::Bool(value) => {
                    let result = PointHlr::new(tx_id, &name, Bool(value), status, cot, timestamp);
                    let value: i64 = if value {1} else {0};
                    let target = PointHlr { tx_id, name: name.to_owned(), value, status, cot, timestamp };
                    assert!(result.to_int() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::Int(value) => {
                    let result = PointHlr::new(tx_id, &name, value, status, cot, timestamp);
                    let target = PointHlr { tx_id, name: name.to_owned(), value, status, cot, timestamp };
                    assert!(result.to_int() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::Real(value) => {
                    let result = PointHlr::new(tx_id, &name, value, status, cot, timestamp);
                    let value: i64 = value.round() as i64;
                    let target = PointHlr { tx_id, name: name.to_owned(), value, status, cot, timestamp };
                    assert!(result.to_int() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::Double(value) => {
                    let result = PointHlr::new(tx_id, &name, value, status, cot, timestamp);
                    let value: i64 = value.round() as i64;
                    let target = PointHlr { tx_id, name: name.to_owned(), value, status, cot, timestamp };
                    assert!(result.to_int() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::String(_) => {
                    // let result = PointHlr::new(tx_id, &name, value, status, cot, timestamp);
                    // let value: i64 = value.parse().unwrap();
                    // let target = PointHlr { tx_id, name: name.to_owned(), value: value, status, cot, timestamp };
                    // assert!(result.to_int() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
            };
        }
        test_duration.exit();
    }
    ///
    /// Testing PointHlr::to_real
    #[test]
    fn to_real() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "to_real";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point1", Value::Bool(false), Status::Invalid, Cot::default(), chrono::Utc::now()),
            (02, 02, "/App/Service/Point2", Value::Bool(true), Status::Invalid, Cot::default(), chrono::Utc::now()),
            (03, 03, "/App/Service/Point3", Value::Int(100i64), Status::Invalid, Cot::default(), chrono::Utc::now()),
            (04, 04, "/App/Service/Point4", Value::Int(200i64), Status::Obsolete, Cot::default(), chrono::Utc::now()),
            (05, 05, "/App/Service/Point5", Value::Real(300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (06, 06, "/App/Service/Point6", Value::Double(300.2f64), Status::TimeInvalid, Cot::default(), chrono::Utc::now()),
            (07, 07, "/App/Service/Point7", Value::String("101.1".to_owned()), Status::Ok, Cot::default(), chrono::Utc::now()),
        ];
        for (step, tx_id, name, value, status, cot, timestamp) in test_data {
            match value {
                Value::Bool(value) => {
                    let result = PointHlr::new(tx_id, &name, Bool(value), status, cot, timestamp);
                    let value: f32 = if value {1.0} else {0.0};
                    let target = PointHlr { tx_id, name: name.to_owned(), value, status, cot, timestamp };
                    assert!(result.to_real() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::Int(value) => {
                    let result = PointHlr::new(tx_id, &name, value, status, cot, timestamp);
                    let value: f32 = value as f32;
                    let target = PointHlr { tx_id, name: name.to_owned(), value, status, cot, timestamp };
                    assert!(result.to_real() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::Real(value) => {
                    let result = PointHlr::new(tx_id, &name, value, status, cot, timestamp);
                    let target = PointHlr { tx_id, name: name.to_owned(), value, status, cot, timestamp };
                    assert!(result.to_real() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::Double(value) => {
                    let result = PointHlr::new(tx_id, &name, value, status, cot, timestamp);
                    let value: f32 = value as f32;
                    let target = PointHlr { tx_id, name: name.to_owned(), value, status, cot, timestamp };
                    assert!(result.to_real() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                Value::String(_) => {
                    // let result = PointHlr::new(tx_id, &name, value, status, cot, timestamp);
                    // let value: f32 = value.parse().unwrap();
                    // let target = PointHlr { tx_id, name: name.to_owned(), value: value, status, cot, timestamp };
                    // assert!(result.to_real() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
            };
        }
        test_duration.exit();
    }
}

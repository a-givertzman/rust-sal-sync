#[cfg(test)]

mod point {
    use std::{sync::Once, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use log::debug;
    use testing::{entities::test_value::Value, stuff::max_test_duration::TestDuration};
    use crate::services::{
        entity::{Cot, Point, PointConfigType, PointHlr, Status},
        subscription::SubscriptionCriteria, types::Bool,
    };
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
    fn serialize_json() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "serialize_json";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (r#"{"cot":"Inf","name":"/App/path/Point.Name.0","status":0,"timestamp":"2024-04-08T09:44:43.950510784+00:00","type":"Bool","value":1}"#,
                Point::Bool(PointHlr::new(
                    0,
                    &format!("/App/path/Point.Name.0"),
                    Bool(true),
                    Status::Ok,
                    Cot::Inf,
                    "2024-04-08T09:44:43.950510784+00:00".parse().unwrap(),
                ))
            ),
            (r#"{"cot":"Inf","name":"/App/path/Point.Name.1","status":0,"timestamp":"2024-04-08T09:44:44.450961534+00:00","type":"Int","value":1234567}"#,
                Point::Int(PointHlr::new(
                    0,
                    &format!("/App/path/Point.Name.1"),
                    1234567,
                    Status::Ok,
                    Cot::Inf,
                    "2024-04-08T09:44:44.450961534+00:00".parse().unwrap(),
                ))
            ),
            // (r#"{"cot":"Inf","name":"/App/path/Point.Name.2","status":0,"timestamp":"2024-04-08T09:44:43.550386216+00:00","type":"Real","value":123.12345}"#,
            //     Point::Real(PointHlr::new(
            //         0,
            //         &format!("/App/path/Point.Name.2"),
            //         123.12345,
            //         Status::Ok,
            //         Cot::Inf,
            //         "2024-04-08T09:44:43.550386216+00:00".parse().unwrap(),
            //     ))
            // ),
            (r#"{"cot":"Inf","name":"/App/path/Point.Name.3","status":0,"timestamp":"2024-04-08T09:44:43.550386216+00:00","type":"Double","value":123.12345}"#,
                Point::Double(PointHlr::new(
                    0,
                    &format!("/App/path/Point.Name.3"),
                    123.12345,
                    Status::Ok,
                    Cot::Inf,
                    "2024-04-08T09:44:43.550386216+00:00".parse().unwrap(),
                ))
            ),
        ];
        debug!("{} | Serialized Point: {:?}", self_id, lexical::parse::<f32, _>("1234.12345"));

        for (target, point) in test_data {
            let target: serde_json::Value = serde_json::from_str(target).unwrap();
            let result = serde_json::to_value(point).unwrap();
            debug!("{} | Serialized Point: {:#?}", self_id, result);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        test_duration.exit();
    }
    ///
    ///
    #[test]
    fn deserialize_json() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "deserialize_json";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (r#"{"cot":"Inf","name":"/App/path/Point.Name.0","status":0,"timestamp":"2024-04-08T09:44:43.950510784+00:00","type":"Bool","value":1}"#,
                Point::Bool(PointHlr::new(
                    0,
                    &format!("/App/path/Point.Name.0"),
                    Bool(true),
                    Status::Ok,
                    Cot::Inf,
                    "2024-04-08T09:44:43.950510784+00:00".parse().unwrap(),
                ))
            ),
            (r#"{"cot":"Inf","name":"/App/path/Point.Name.1","status":0,"timestamp":"2024-04-08T09:44:44.450961534+00:00","type":"Int","value":1234567}"#,
                Point::Int(PointHlr::new(
                    0,
                    &format!("/App/path/Point.Name.1"),
                    1234567,
                    Status::Ok,
                    Cot::Inf,
                    "2024-04-08T09:44:44.450961534+00:00".parse().unwrap(),
                ))
            ),
            (r#"{"cot":"Inf","name":"/App/path/Point.Name.2","status":0,"timestamp":"2024-04-08T09:44:43.550386216+00:00","type":"Real","value":123.12345}"#,
                Point::Real(PointHlr::new(
                    0,
                    &format!("/App/path/Point.Name.2"),
                    123.12345,
                    Status::Ok,
                    Cot::Inf,
                    "2024-04-08T09:44:43.550386216+00:00".parse().unwrap(),
                ))
            ),
        ];
        for (point_json, target) in test_data {
            debug!("{} | input: {:#?}", self_id, point_json);
            let result: Point = serde_json::from_str(point_json).unwrap();
            // let target: serde_json::Value = json!(target)&target.name, &conf).unwrap();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        test_duration.exit();
    }
    ///
    /// Testing Point::tx_id
    #[test]
    fn tx_id() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "tx_id";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point01", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (02, 02, "/App/Service/Point02", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (03, 03, "/App/Service/Point03", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (04, 04, "/App/Service/Point04", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (05, 05, "/App/Service/Point05", Value::Int(100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (06, 06, "/App/Service/Point06", Value::Int(-100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (07, 07, "/App/Service/Point07", Value::Int(200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (08, 08, "/App/Service/Point08", Value::Int(-200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (09, 09, "/App/Service/Point09", Value::Real(300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (10, 10, "/App/Service/Point10", Value::Real(-300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (11, 11, "/App/Service/Point11", Value::Double(300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (12, 12, "/App/Service/Point12", Value::Double(-300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (13, 13, "/App/Service/Point13", Value::String("101.1".into()), Status::Ok, Cot::default(), chrono::Utc::now()),
        ];
        for (step, tx_id, name, value, status, cot, timestamp) in test_data {
            match value {
                Value::Bool(value) => {
                    let result = Point::Bool(PointHlr::new(tx_id, &name, Bool(value), status, cot, timestamp));
                    assert!(result.tx_id() == tx_id, "step {} \nresult: {:?}\ntarget: {:?}", step, result.tx_id(), tx_id);
                }
                Value::Int(value) => {
                    let result = Point::Int(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.tx_id() == tx_id, "step {} \nresult: {:?}\ntarget: {:?}", step, result.tx_id(), tx_id);
                }
                Value::Real(value) => {
                    let result = Point::Real(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.tx_id() == tx_id, "step {} \nresult: {:?}\ntarget: {:?}", step, result.tx_id(), tx_id);
                }
                Value::Double(value) => {
                    let result = Point::Double(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.tx_id() == tx_id, "step {} \nresult: {:?}\ntarget: {:?}", step, result.tx_id(), tx_id);
                }
                Value::String(value) => {
                    let result = Point::String(PointHlr::new(tx_id, &name, value.clone(), status, cot, timestamp));
                    assert!(result.tx_id() == tx_id, "step {} \nresult: {:?}\ntarget: {:?}", step, result.tx_id(), tx_id);
                }
            };
        }
        test_duration.exit();
    }
    ///
    /// Testing Point::type_
    #[test]
    fn type_() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "type_";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point01", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (02, 02, "/App/Service/Point02", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (03, 03, "/App/Service/Point03", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (04, 04, "/App/Service/Point04", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (05, 05, "/App/Service/Point05", Value::Int(100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (06, 06, "/App/Service/Point06", Value::Int(-100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (07, 07, "/App/Service/Point07", Value::Int(200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (08, 08, "/App/Service/Point08", Value::Int(-200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (09, 09, "/App/Service/Point09", Value::Real(300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (10, 10, "/App/Service/Point10", Value::Real(-300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (11, 11, "/App/Service/Point11", Value::Double(300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (12, 12, "/App/Service/Point12", Value::Double(-300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (13, 13, "/App/Service/Point13", Value::String("101.1".into()), Status::Ok, Cot::default(), chrono::Utc::now()),
        ];
        for (step, tx_id, name, value, status, cot, timestamp) in test_data {
            match value {
                Value::Bool(value) => {
                    let result = Point::Bool(PointHlr::new(tx_id, &name, Bool(value), status, cot, timestamp));
                    assert!(result.type_() == PointConfigType::Bool, "step {} \nresult: {:?}\ntarget: {:?}", step, result.type_(), PointConfigType::Bool);
                }
                Value::Int(value) => {
                    let result = Point::Int(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.type_() == PointConfigType::Int, "step {} \nresult: {:?}\ntarget: {:?}", step, result.type_(), PointConfigType::Int);
                }
                Value::Real(value) => {
                    let result = Point::Real(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.type_() == PointConfigType::Real, "step {} \nresult: {:?}\ntarget: {:?}", step, result.type_(), PointConfigType::Real);
                }
                Value::Double(value) => {
                    let result = Point::Double(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.type_() == PointConfigType::Double, "step {} \nresult: {:?}\ntarget: {:?}", step, result.type_(), PointConfigType::Double);
                }
                Value::String(value) => {
                    let result = Point::String(PointHlr::new(tx_id, &name, value.clone(), status, cot, timestamp));
                    assert!(result.type_() == PointConfigType::String, "step {} \nresult: {:?}\ntarget: {:?}", step, result.type_(), PointConfigType::String);
                }
            };
        }
        test_duration.exit();
    }
    ///
    /// Testing Point::name
    #[test]
    fn name() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "name";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point01", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (02, 02, "/App/Service/Point02", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (03, 03, "/App/Service/Point03", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (04, 04, "/App/Service/Point04", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (05, 05, "/App/Service/Point05", Value::Int(100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (06, 06, "/App/Service/Point06", Value::Int(-100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (07, 07, "/App/Service/Point07", Value::Int(200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (08, 08, "/App/Service/Point08", Value::Int(-200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (09, 09, "/App/Service/Point09", Value::Real(300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (10, 10, "/App/Service/Point10", Value::Real(-300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (11, 11, "/App/Service/Point11", Value::Double(300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (12, 12, "/App/Service/Point12", Value::Double(-300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (13, 13, "/App/Service/Point13", Value::String("101.1".into()), Status::Ok, Cot::default(), chrono::Utc::now()),
        ];
        for (step, tx_id, name, value, status, cot, timestamp) in test_data {
            match value {
                Value::Bool(value) => {
                    let result = Point::Bool(PointHlr::new(tx_id, &name, Bool(value), status, cot, timestamp));
                    assert!(result.name() == name, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name(), name);
                }
                Value::Int(value) => {
                    let result = Point::Int(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.name() == name, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name(), name);
                }
                Value::Real(value) => {
                    let result = Point::Real(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.name() == name, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name(), name);
                }
                Value::Double(value) => {
                    let result = Point::Double(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.name() == name, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name(), name);
                }
                Value::String(value) => {
                    let result = Point::String(PointHlr::new(tx_id, &name, value.clone(), status, cot, timestamp));
                    assert!(result.name() == name, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name(), name);
                }
            };
        }
        test_duration.exit();
    }
    ///
    /// Testing Point::dest
    #[test]
    fn dest() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "dest";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point01", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (02, 02, "/App/Service/Point02", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (03, 03, "/App/Service/Point03", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (04, 04, "/App/Service/Point04", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (05, 05, "/App/Service/Point05", Value::Int(100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (06, 06, "/App/Service/Point06", Value::Int(-100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (07, 07, "/App/Service/Point07", Value::Int(200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (08, 08, "/App/Service/Point08", Value::Int(-200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (09, 09, "/App/Service/Point09", Value::Real(300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (10, 10, "/App/Service/Point10", Value::Real(-300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (11, 11, "/App/Service/Point11", Value::Double(300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (12, 12, "/App/Service/Point12", Value::Double(-300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (13, 13, "/App/Service/Point13", Value::String("101.1".into()), Status::Ok, Cot::default(), chrono::Utc::now()),
        ];
        for (step, tx_id, name, value, status, cot, timestamp) in test_data {
            match value {
                Value::Bool(value) => {
                    let result = Point::Bool(PointHlr::new(tx_id, &name, Bool(value), status, cot, timestamp));
                    let target = SubscriptionCriteria::new(name, cot).destination();
                    assert!(result.dest() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name(), target);
                }
                Value::Int(value) => {
                    let result = Point::Int(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    let target = SubscriptionCriteria::new(name, cot).destination();
                    assert!(result.dest() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name(), target);
                }
                Value::Real(value) => {
                    let result = Point::Real(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    let target = SubscriptionCriteria::new(name, cot).destination();
                    assert!(result.dest() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name(), target);
                }
                Value::Double(value) => {
                    let result = Point::Double(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    let target = SubscriptionCriteria::new(name, cot).destination();
                    assert!(result.dest() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name(), target);
                }
                Value::String(value) => {
                    let result = Point::String(PointHlr::new(tx_id, &name, value.clone(), status, cot, timestamp));
                    let target = SubscriptionCriteria::new(name, cot).destination();
                    assert!(result.dest() == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name(), target);
                }
            };
        }
        test_duration.exit();
    }
    ///
    /// Testing Point::value
    #[test]
    fn value() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "value";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point01", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (02, 02, "/App/Service/Point02", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (03, 03, "/App/Service/Point03", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (04, 04, "/App/Service/Point04", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (05, 05, "/App/Service/Point05", Value::Int(100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (06, 06, "/App/Service/Point06", Value::Int(-100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (07, 07, "/App/Service/Point07", Value::Int(200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (08, 08, "/App/Service/Point08", Value::Int(-200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (09, 09, "/App/Service/Point09", Value::Real(300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (10, 10, "/App/Service/Point10", Value::Real(-300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (11, 11, "/App/Service/Point11", Value::Double(300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (12, 12, "/App/Service/Point12", Value::Double(-300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (13, 13, "/App/Service/Point13", Value::String("101.1".into()), Status::Ok, Cot::default(), chrono::Utc::now()),
        ];
        for (step, tx_id, name, value_, status, cot, timestamp) in test_data {
            match value_.clone() {
                Value::Bool(value) => {
                    let result = Point::Bool(PointHlr::new(tx_id, &name, Bool(value), status, cot, timestamp));
                    assert!(result.value() == value_, "step {} \nresult: {:?}\ntarget: {:?}", step, result.value(), value_);
                }
                Value::Int(value) => {
                    let result = Point::Int(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.value() == value_, "step {} \nresult: {:?}\ntarget: {:?}", step, result.value(), value_);
                }
                Value::Real(value) => {
                    let result = Point::Real(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.value() == value_, "step {} \nresult: {:?}\ntarget: {:?}", step, result.value(), value_);
                }
                Value::Double(value) => {
                    let result = Point::Double(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.value() == value_, "step {} \nresult: {:?}\ntarget: {:?}", step, result.value(), value_);
                }
                Value::String(value) => {
                    let result = Point::String(PointHlr::new(tx_id, &name, value.clone(), status, cot, timestamp));
                    assert!(result.value() == value_, "step {} \nresult: {:?}\ntarget: {:?}", step, result.value(), value_);
                }
            };
        }
        test_duration.exit();
    }
    ///
    /// Testing Point::status
    #[test]
    fn status() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "status";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point01", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (02, 02, "/App/Service/Point02", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (03, 03, "/App/Service/Point03", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (04, 04, "/App/Service/Point04", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (05, 05, "/App/Service/Point05", Value::Int(100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (06, 06, "/App/Service/Point06", Value::Int(-100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (07, 07, "/App/Service/Point07", Value::Int(200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (08, 08, "/App/Service/Point08", Value::Int(-200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (09, 09, "/App/Service/Point09", Value::Real(300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (10, 10, "/App/Service/Point10", Value::Real(-300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (11, 11, "/App/Service/Point11", Value::Double(300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (12, 12, "/App/Service/Point12", Value::Double(-300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (13, 13, "/App/Service/Point13", Value::String("101.1".into()), Status::Ok, Cot::default(), chrono::Utc::now()),
        ];
        for (step, tx_id, name, value, status, cot, timestamp) in test_data {
            match value {
                Value::Bool(value) => {
                    let result = Point::Bool(PointHlr::new(tx_id, &name, Bool(value), status, cot, timestamp));
                    assert!(result.status() == status, "step {} \nresult: {:?}\ntarget: {:?}", step, result.status(), status);
                }
                Value::Int(value) => {
                    let result = Point::Int(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.status() == status, "step {} \nresult: {:?}\ntarget: {:?}", step, result.status(), status);
                }
                Value::Real(value) => {
                    let result = Point::Real(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.status() == status, "step {} \nresult: {:?}\ntarget: {:?}", step, result.status(), status);
                }
                Value::Double(value) => {
                    let result = Point::Double(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.status() == status, "step {} \nresult: {:?}\ntarget: {:?}", step, result.status(), status);
                }
                Value::String(value) => {
                    let result = Point::String(PointHlr::new(tx_id, &name, value.clone(), status, cot, timestamp));
                    assert!(result.status() == status, "step {} \nresult: {:?}\ntarget: {:?}", step, result.status(), status);
                }
            };
        }
        test_duration.exit();
    }
    ///
    /// Testing Point::cot
    #[test]
    fn cot() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "cot";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point01", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (02, 02, "/App/Service/Point02", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (03, 03, "/App/Service/Point03", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (04, 04, "/App/Service/Point04", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (05, 05, "/App/Service/Point05", Value::Int(100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (06, 06, "/App/Service/Point06", Value::Int(-100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (07, 07, "/App/Service/Point07", Value::Int(200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (08, 08, "/App/Service/Point08", Value::Int(-200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (09, 09, "/App/Service/Point09", Value::Real(300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (10, 10, "/App/Service/Point10", Value::Real(-300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (11, 11, "/App/Service/Point11", Value::Double(300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (12, 12, "/App/Service/Point12", Value::Double(-300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (13, 13, "/App/Service/Point13", Value::String("101.1".into()), Status::Ok, Cot::default(), chrono::Utc::now()),
        ];
        for (step, tx_id, name, value, status, cot, timestamp) in test_data {
            match value {
                Value::Bool(value) => {
                    let result = Point::Bool(PointHlr::new(tx_id, &name, Bool(value), status, cot, timestamp));
                    assert!(result.cot() == cot, "step {} \nresult: {:?}\ntarget: {:?}", step, result.cot(), cot);
                }
                Value::Int(value) => {
                    let result = Point::Int(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.cot() == cot, "step {} \nresult: {:?}\ntarget: {:?}", step, result.cot(), cot);
                }
                Value::Real(value) => {
                    let result = Point::Real(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.cot() == cot, "step {} \nresult: {:?}\ntarget: {:?}", step, result.cot(), cot);
                }
                Value::Double(value) => {
                    let result = Point::Double(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.cot() == cot, "step {} \nresult: {:?}\ntarget: {:?}", step, result.cot(), cot);
                }
                Value::String(value) => {
                    let result = Point::String(PointHlr::new(tx_id, &name, value.clone(), status, cot, timestamp));
                    assert!(result.cot() == cot, "step {} \nresult: {:?}\ntarget: {:?}", step, result.cot(), cot);
                }
            };
        }
        test_duration.exit();
    }
    ///
    /// Testing Point::timestamp
    #[test]
    fn timestamp() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "timestamp";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, 01, "/App/Service/Point01", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (02, 02, "/App/Service/Point02", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (03, 03, "/App/Service/Point03", Value::Bool(true), Status::Ok, Cot::default(), chrono::Utc::now()),
            (04, 04, "/App/Service/Point04", Value::Bool(false), Status::Ok, Cot::default(), chrono::Utc::now()),
            (05, 05, "/App/Service/Point05", Value::Int(100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (06, 06, "/App/Service/Point06", Value::Int(-100i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (07, 07, "/App/Service/Point07", Value::Int(200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (08, 08, "/App/Service/Point08", Value::Int(-200i64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (09, 09, "/App/Service/Point09", Value::Real(300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (10, 10, "/App/Service/Point10", Value::Real(-300.1f32), Status::Ok, Cot::default(), chrono::Utc::now()),
            (11, 11, "/App/Service/Point11", Value::Double(300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (12, 12, "/App/Service/Point12", Value::Double(-300.2f64), Status::Ok, Cot::default(), chrono::Utc::now()),
            (13, 13, "/App/Service/Point13", Value::String("101.1".into()), Status::Ok, Cot::default(), chrono::Utc::now()),
        ];
        for (step, tx_id, name, value, status, cot, timestamp) in test_data {
            match value {
                Value::Bool(value) => {
                    let result = Point::Bool(PointHlr::new(tx_id, &name, Bool(value), status, cot, timestamp));
                    assert!(result.timestamp() == timestamp, "step {} \nresult: {:?}\ntarget: {:?}", step, result.timestamp(), timestamp);
                }
                Value::Int(value) => {
                    let result = Point::Int(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.timestamp() == timestamp, "step {} \nresult: {:?}\ntarget: {:?}", step, result.timestamp(), timestamp);
                }
                Value::Real(value) => {
                    let result = Point::Real(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.timestamp() == timestamp, "step {} \nresult: {:?}\ntarget: {:?}", step, result.timestamp(), timestamp);
                }
                Value::Double(value) => {
                    let result = Point::Double(PointHlr::new(tx_id, &name, value, status, cot, timestamp));
                    assert!(result.timestamp() == timestamp, "step {} \nresult: {:?}\ntarget: {:?}", step, result.timestamp(), timestamp);
                }
                Value::String(value) => {
                    let result = Point::String(PointHlr::new(tx_id, &name, value.clone(), status, cot, timestamp));
                    assert!(result.timestamp() == timestamp, "step {} \nresult: {:?}\ntarget: {:?}", step, result.timestamp(), timestamp);
                }
            };
        }
        test_duration.exit();
    }
}

#[cfg(test)]

use std::{sync::Once, time::Duration};
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
use log::debug;
use testing::stuff::max_test_duration::TestDuration;
use crate::services::entity::PointType;
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
/// Testing PointType::serialize
#[test]
fn serialize() {
    DebugSession::new().filter(LogLevel::Debug).init();
    init_once();
    init_each();
    println!();
    let self_id = "serialize";
    println!("\n{}", self_id);
    let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
    test_duration.run().unwrap();
    let test_data = [
        (01, r#"Bool"#, PointType::Bool),
        (03, r#"Int"#, PointType::Int),
        (05, r#"Real"#, PointType::Real),
        (07, r#"Double"#, PointType::Double),
        (09, r#"String"#, PointType::String),
        (11, r#"Json"#, PointType::Json),
    ];
    for (step, target, value) in test_data {
        let result = serde_yaml::to_value(&value).unwrap();
        let result = result.as_str().unwrap();
        debug!("Step: {}  |  Serialized PointType: {:?}", step, result);
        assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        let result = serde_json::to_value(&value).unwrap();
        let result = result.as_str().unwrap();
        debug!("Step: {}  |  Serialized PointType: {:?}", step, result);
        assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    }
    test_duration.exit();
}
///
/// Testing PointType::deserialize_yaml
#[test]
fn deserialize_yaml() {
    DebugSession::new().filter(LogLevel::Debug).init();
    init_once();
    init_each();
    println!();
    let self_id = "deserialize_yaml";
    println!("\n{}", self_id);
    let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
    test_duration.run().unwrap();
    let test_data = [
        (01, r#"bool"#, PointType::Bool),
        (02, r#"Bool"#, PointType::Bool),
        (03, r#"int"#, PointType::Int),
        (04, r#"Int"#, PointType::Int),
        (05, r#"real"#, PointType::Real),
        (06, r#"Real"#, PointType::Real),
        (07, r#"double"#, PointType::Double),
        (08, r#"Double"#, PointType::Double),
        (09, r#"string"#, PointType::String),
        (10, r#"String"#, PointType::String),
        (11, r#"json"#, PointType::Json),
        (12, r#"Json"#, PointType::Json),
    ];
    for (step, value, target) in test_data {
        let result: PointType = serde_yaml::from_str(value).unwrap();
        debug!("Step: {}  |  Deserialized yaml PointType: {:?}", step, result);
        assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    }
    test_duration.exit();
}
///
/// Testing PointType::deserialize_json
#[test]
fn deserialize_json() {
    DebugSession::new().filter(LogLevel::Debug).init();
    init_once();
    init_each();
    println!();
    let self_id = "deserialize_json";
    println!("\n{}", self_id);
    let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
    test_duration.run().unwrap();
    let test_data = [
        (01, r#""bool""#, PointType::Bool),
        (02, r#""Bool""#, PointType::Bool),
        (03, r#""int""#, PointType::Int),
        (04, r#""Int""#, PointType::Int),
        (05, r#""real""#, PointType::Real),
        (06, r#""Real""#, PointType::Real),
        (07, r#""double""#, PointType::Double),
        (08, r#""Double""#, PointType::Double),
        (09, r#""string""#, PointType::String),
        (10, r#""String""#, PointType::String),
        (11, r#""json""#, PointType::Json),
        (12, r#""Json""#, PointType::Json),
    ];
    for (step, value, target) in test_data {
        let result: PointType = serde_json::from_str(value).unwrap();
        debug!("Step: {}  |  Deserialized json PointType: {:?}", step, result);
        assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    }
    test_duration.exit();
}
///
/// Testing PointType::to_string
#[test]
fn to_string() {
    DebugSession::new().filter(LogLevel::Debug).init();
    init_once();
    init_each();
    println!();
    let self_id = "to_string";
    println!("\n{}", self_id);
    let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
    test_duration.run().unwrap();
    let test_data = [
        (02, "Bool", PointType::Bool),
        (04, "Int", PointType::Int),
        (06, "Real", PointType::Real),
        (08, "Double", PointType::Double),
        (10, "String", PointType::String),
        (12, "Json", PointType::Json),
    ];
    for (step, target, value) in test_data {
        let result = value.to_string();
        debug!("Step: {}  |  Deserialized json PointType: {:?}", step, result);
        assert!(&result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    }
    test_duration.exit();
}

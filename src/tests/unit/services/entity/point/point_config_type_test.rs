#[cfg(test)]

use std::{sync::Once, time::Duration};
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
use log::debug;
use testing::stuff::max_test_duration::TestDuration;
use crate::services::entity::PointConfType;
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
/// Testing PointConfType::serialize
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
        (01, r#"Bool"#, PointConfType::Bool),
        (03, r#"Int"#, PointConfType::Int),
        (05, r#"Real"#, PointConfType::Real),
        (07, r#"Double"#, PointConfType::Double),
        (09, r#"String"#, PointConfType::String),
        (11, r#"Json"#, PointConfType::Json),
    ];
    for (step, target, value) in test_data {
        let result = serde_yaml::to_value(&value).unwrap();
        let result = result.as_str().unwrap();
        debug!("Step: {}  |  Serialized PointConfType: {:?}", step, result);
        assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        let result = serde_json::to_value(&value).unwrap();
        let result = result.as_str().unwrap();
        debug!("Step: {}  |  Serialized PointConfType: {:?}", step, result);
        assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    }
    test_duration.exit();
}
///
/// Testing PointConfType::deserialize_yaml
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
        (01, r#"bool"#, PointConfType::Bool),
        (02, r#"Bool"#, PointConfType::Bool),
        (03, r#"int"#, PointConfType::Int),
        (04, r#"Int"#, PointConfType::Int),
        (05, r#"real"#, PointConfType::Real),
        (06, r#"Real"#, PointConfType::Real),
        (07, r#"double"#, PointConfType::Double),
        (08, r#"Double"#, PointConfType::Double),
        (09, r#"string"#, PointConfType::String),
        (10, r#"String"#, PointConfType::String),
        (11, r#"json"#, PointConfType::Json),
        (12, r#"Json"#, PointConfType::Json),
    ];
    for (step, value, target) in test_data {
        let result: PointConfType = serde_yaml::from_str(value).unwrap();
        debug!("Step: {}  |  Deserialized yaml PointConfType: {:?}", step, result);
        assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    }
    test_duration.exit();
}
///
/// Testing PointConfType::deserialize_json
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
        (01, r#""bool""#, PointConfType::Bool),
        (02, r#""Bool""#, PointConfType::Bool),
        (03, r#""int""#, PointConfType::Int),
        (04, r#""Int""#, PointConfType::Int),
        (05, r#""real""#, PointConfType::Real),
        (06, r#""Real""#, PointConfType::Real),
        (07, r#""double""#, PointConfType::Double),
        (08, r#""Double""#, PointConfType::Double),
        (09, r#""string""#, PointConfType::String),
        (10, r#""String""#, PointConfType::String),
        (11, r#""json""#, PointConfType::Json),
        (12, r#""Json""#, PointConfType::Json),
    ];
    for (step, value, target) in test_data {
        let result: PointConfType = serde_json::from_str(value).unwrap();
        debug!("Step: {}  |  Deserialized json PointConfType: {:?}", step, result);
        assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    }
    test_duration.exit();
}
///
/// Testing PointConfType::to_string
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
        (02, "Bool", PointConfType::Bool),
        (04, "Int", PointConfType::Int),
        (06, "Real", PointConfType::Real),
        (08, "Double", PointConfType::Double),
        (10, "String", PointConfType::String),
        (12, "Json", PointConfType::Json),
    ];
    for (step, target, value) in test_data {
        let result = value.to_string();
        debug!("Step: {}  |  Deserialized json PointConfType: {:?}", step, result);
        assert!(&result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    }
    test_duration.exit();
}

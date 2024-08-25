#[cfg(test)]

mod point_config_type {
    use std::{sync::Once, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use log::debug;
    use testing::stuff::max_test_duration::TestDuration;
    use crate::services::entity::point::point_config_type::PointConfigType;
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
    /// Testing PointConfigType::serialize
    #[test]
    fn serialize() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "serialize";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, r#"Bool"#, PointConfigType::Bool),
            (03, r#"Int"#, PointConfigType::Int),
            (05, r#"Real"#, PointConfigType::Real),
            (07, r#"Double"#, PointConfigType::Double),
            (09, r#"String"#, PointConfigType::String),
            (11, r#"Json"#, PointConfigType::Json),
        ];
        for (step, target, value) in test_data {
            let result = serde_yaml::to_value(&value).unwrap();
            let result = result.as_str().unwrap();
            debug!("Step: {}  |  Serialized PointConfigType: {:?}", step, result);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
            let result = serde_json::to_value(&value).unwrap();
            let result = result.as_str().unwrap();
            debug!("Step: {}  |  Serialized PointConfigType: {:?}", step, result);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
    ///
    /// Testing PointConfigType::deserialize_yaml
    #[test]
    fn deserialize_yaml() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "deserialize_yaml";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, r#"bool"#, PointConfigType::Bool),
            (02, r#"Bool"#, PointConfigType::Bool),
            (03, r#"int"#, PointConfigType::Int),
            (04, r#"Int"#, PointConfigType::Int),
            (05, r#"real"#, PointConfigType::Real),
            (06, r#"Real"#, PointConfigType::Real),
            (07, r#"double"#, PointConfigType::Double),
            (08, r#"Double"#, PointConfigType::Double),
            (09, r#"string"#, PointConfigType::String),
            (10, r#"String"#, PointConfigType::String),
            (11, r#"json"#, PointConfigType::Json),
            (12, r#"Json"#, PointConfigType::Json),
        ];
        for (step, value, target) in test_data {
            let result: PointConfigType = serde_yaml::from_str(value).unwrap();
            debug!("Step: {}  |  Deserialized yaml PointConfigType: {:?}", step, result);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
    ///
    /// Testing PointConfigType::deserialize_json
    #[test]
    fn deserialize_json() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "deserialize_json";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, r#""bool""#, PointConfigType::Bool),
            (02, r#""Bool""#, PointConfigType::Bool),
            (03, r#""int""#, PointConfigType::Int),
            (04, r#""Int""#, PointConfigType::Int),
            (05, r#""real""#, PointConfigType::Real),
            (06, r#""Real""#, PointConfigType::Real),
            (07, r#""double""#, PointConfigType::Double),
            (08, r#""Double""#, PointConfigType::Double),
            (09, r#""string""#, PointConfigType::String),
            (10, r#""String""#, PointConfigType::String),
            (11, r#""json""#, PointConfigType::Json),
            (12, r#""Json""#, PointConfigType::Json),
        ];
        for (step, value, target) in test_data {
            let result: PointConfigType = serde_json::from_str(value).unwrap();
            debug!("Step: {}  |  Deserialized json PointConfigType: {:?}", step, result);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
    ///
    /// Testing PointConfigType::to_string
    #[test]
    fn to_string() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "to_string";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (02, "Bool", PointConfigType::Bool),
            (04, "Int", PointConfigType::Int),
            (06, "Real", PointConfigType::Real),
            (08, "Double", PointConfigType::Double),
            (10, "String", PointConfigType::String),
            (12, "Json", PointConfigType::Json),
        ];
        for (step, target, value) in test_data {
            let result = value.to_string();
            debug!("Step: {}  |  Deserialized json PointConfigType: {:?}", step, result);
            assert!(&result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }

}

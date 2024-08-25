#[cfg(test)]

mod point_config_address {
    use std::{sync::Once, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use log::debug;
    use testing::stuff::max_test_duration::TestDuration;
    use crate::services::entity::point::point_config_address::PointConfigAddress;
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
    /// Testing PointConfigAddress::empty
    #[test]
    fn empty() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "serialize";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        let result = PointConfigAddress::empty();
        assert!(result.offset == None, "\nresult: {:?}\ntarget: {:?}", result.offset, None::<u32>);
        assert!(result.bit == None, "\nresult: {:?}\ntarget: {:?}", result.bit, None::<u8>);
        test_duration.exit();
    }
    ///
    /// Testing PointConfigType::serialize yaml
    #[test]
    fn serialize_yaml() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "serialize yaml";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, r#"
                offset: 111
            "#, 
            PointConfigAddress { offset: Some(111), bit: None }),
            (02, r#"
                offset: 111
                bit: 3
            "#, 
            PointConfigAddress { offset: Some(111), bit: Some(3) }),
        ];
        for (step, target, value) in test_data {
            let result = serde_yaml::to_value(&value).unwrap();
            let target: serde_yaml::Value = serde_yaml::from_str(&target).unwrap();
            debug!("Step: {}  |  Serialized yaml PointConfigAddress: {:?}", step, result);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
    ///
    /// Testing PointConfigType::serialize json
    #[test]
    fn serialize_json() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "serialize json";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, r#"{
                "offset": 111
            }"#, 
            PointConfigAddress { offset: Some(111), bit: None }),
            (02, r#"{
                "offset": 111,
                "bit": 3
            }"#, 
            PointConfigAddress { offset: Some(111), bit: Some(3) }),
        ];
        for (step, target, value) in test_data {
            let result = serde_json::to_value(&value).unwrap();
            let target: serde_json::Value = serde_json::from_str(&target).unwrap();
            debug!("Step: {}  |  Serialized json PointConfigAddress: {:?}", step, result);
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
            (01, r#"
                offset: 111
            "#, 
            PointConfigAddress { offset: Some(111), bit: None }),
            (02, r#"
                offset: 111
                bit: 3
            "#, 
            PointConfigAddress { offset: Some(111), bit: Some(3) }),
        ];
        for (step, value, target) in test_data {
            let result: PointConfigAddress = serde_yaml::from_str(value).unwrap();
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
            (01, r#"{
                "offset": 111
            }"#, 
            PointConfigAddress { offset: Some(111), bit: None }),
            (02, r#"{
                "offset": 111,
                "bit": 3
            }"#, 
            PointConfigAddress { offset: Some(111), bit: Some(3) }),
        ];
        for (step, value, target) in test_data {
            let result: PointConfigAddress = serde_json::from_str(value).unwrap();
            debug!("Step: {}  |  Deserialized json PointConfigType: {:?}", step, result);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}

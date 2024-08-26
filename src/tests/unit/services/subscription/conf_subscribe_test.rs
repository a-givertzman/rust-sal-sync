#[cfg(test)]

mod conf_subscribe {
    use std::{collections::HashMap, sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::services::{
        entity::{cot::Cot, name::Name, point::point_config::PointConfig},
        subscription::{
            subscription_criteria::SubscriptionCriteria,
            conf_subscribe::ConfSubscribe,
        },
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
    fn new() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "conf_subscribe_test";
        let self_name = Name::new("", self_id);
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let points = [
            r#"point Drive.Speed:
                type: 'Real'
                address:
                    offset: 0"#,
            r#"point Drive.OutputVoltage:
                type: 'Real'
                address:
                    offset: 4"#,
            r#"point Drive.DCVoltage:
                type: 'Real'
                address:
                    offset: 8"#,
            r#"point Drive.Current:
                type: 'Real'
                address:
                    offset: 12
                history: r"#,
            r#"point Drive.Torque:
                type: 'Real'
                address:
                    offset: 16"#,
            r#"point Drive.Torque1:
                type: 'Real'
                address:
                    offset: 16
                alarm: 1"#,
        ];
        let points = points.map(|conf| {
            let conf = serde_yaml::from_str(conf).unwrap();
            PointConfig::from_yaml(&self_name, &conf)
        });
        let test_data = [
            (
                r#"
                    subscribe: MultiQueue
                "#,
                HashMap::from([("MultiQueue".to_owned(), Some(vec![]))])
            ),
            (
                r#"
                    subscribe:
                        MultiQueue_01: {}
                        MultiQueue_02: {}
                "#,
                HashMap::from([
                    ("MultiQueue_01".to_owned(), Some(vec![])),
                    ("MultiQueue_02".to_owned(), Some(vec![])),
                ])
            ),
            (
                r#"
                    subscribe:
                        App/MultiQueue:
                            Inf: []
                "#,
                HashMap::from([("App/MultiQueue".to_owned(), Some(vec![
                    SubscriptionCriteria::new(Name::new(self_id, "Drive.Speed").join(), Cot::Inf),
                    SubscriptionCriteria::new(Name::new(self_id, "Drive.OutputVoltage").join(), Cot::Inf),
                    SubscriptionCriteria::new(Name::new(self_id, "Drive.DCVoltage").join(), Cot::Inf),
                    SubscriptionCriteria::new(Name::new(self_id, "Drive.Current").join(), Cot::Inf),
                    SubscriptionCriteria::new(Name::new(self_id, "Drive.Torque").join(), Cot::Inf),
                    SubscriptionCriteria::new(Name::new(self_id, "Drive.Torque1").join(), Cot::Inf),
                ]))])
            ),
            (
                r#"
                    subscribe:
                        App/MultiQueue:
                            {history: rw}: []
                "#,
                HashMap::from([("App/MultiQueue".to_owned(), Some(vec![
                    SubscriptionCriteria::new(Name::new(self_id, "Drive.Current").join(), Cot::All),
                ]))])
            ),
            (
                r#"
                    subscribe:
                        App/MultiQueue:
                            {alarm: 1}: []
                "#,
                HashMap::from([("App/MultiQueue".to_owned(), Some(vec![
                    SubscriptionCriteria::new(Name::new(self_id, "Drive.Torque1").join(), Cot::All),
                ]))])
            ),
            (
                r#"
                    subscribe:
                        MultiQueue:
                            {cot: Inf, history: r}: []
                "#,
                HashMap::from([("MultiQueue".to_owned(), Some(vec![
                    SubscriptionCriteria::new(Name::new(self_id, "Drive.Current").join(), Cot::Inf),
                ]))])
            ),
            (
                r#"
                    subscribe:
                        MultiQueue:
                            {cot: Inf, history: r}:
                                - /App/Service/Point.Name.01
                                - /App/Service/Point.Name.02
                "#,
                HashMap::from([("MultiQueue".to_owned(), None)])
            ),
        ];
        for (conf, target) in test_data {
            match serde_yaml::from_str(conf) {
                Ok(conf) => {
                    let conf: serde_yaml::Value = conf;
                    let (_key, conf) = conf.as_mapping().unwrap().into_iter().next().unwrap();
                    let conf = ConfSubscribe::new(conf.clone());
                    println!("\nconf     : {:#?}", conf);
                    let result = conf.with(&points.to_vec());
                    println!("subscribe: {:#?}", result);
                    assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
                }
                Err(err) => {
                    panic!("Deserialize error: {:#?}", err);
                }
            };
        }
        test_duration.exit();
    }
}

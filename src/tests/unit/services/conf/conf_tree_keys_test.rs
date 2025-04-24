#[cfg(test)]
mod conf_tree {
    use std::sync::Once;
    use debugging ::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use crate::services::conf::ConfTree;
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
    /// Testing [ConfTree].keys() method
    #[test]
    fn keys() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        log::info!("test_config_tree_keys");
        // let (initial, switches) = init_each();
        let test_data: Vec<(&[&str], &str, &[&str])> = vec![
            (
                &[],
                r#"
                    let newVar2:
                        input: const 2.2
                    let newVar3:
                        input: const 3.3
                    let newVar1:
                        input: const 1.1
                "#,
                &["let newVar2", "let newVar3", "let newVar1"],
            ),
            (
                &["cycle", "subscribe"],
                r#"
                    cycle: 200
                    subscribe: 
                        /App/MultiQueue:
                            {cot: Inf, history: rw}: []

                    fn fnOr:
                        input1: const someValue
                        input2: point '/path/Point.Name/'
                        input:
                    fn fnDebug:
                        input: point '/path/Point.Name/'
                "#,
                &["fn fnOr", "fn fnDebug"],
            ),
        ];
        for (exclude, value, target) in test_data {
            // log::debug!("test value: {:?}", value);
            let conf: serde_yaml::Value = serde_yaml::from_str(value).unwrap();
            // log::debug!("test conf: {:?}", conf);
            // let conf = test_data.get("/").unwrap();
            let conf_tree = ConfTree::new_root(conf);
            // log::debug!("confTree: {:?}", conf_tree);
            let result = conf_tree.keys(exclude);
            log::debug!("result: {:?}", result);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            // let mut targets = target.as_map().iter();
            // for (_name, result) in result.as_map() {
            //     let (_, target) = targets.next().unwrap();
            //     assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            // }
        }
    }
}

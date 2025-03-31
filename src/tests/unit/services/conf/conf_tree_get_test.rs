#[cfg(test)]
mod config_tree_get {
    use std::sync::Once;
    use indexmap::IndexMap;
    use testing::entities::test_value::Value;
    use debugging ::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use crate::services::conf::conf_tree::{ConfTree, ConfTreeGet};
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
    /// For storing different types in the same collection
    #[derive(Debug)]
    enum Kind {
        Val(Value),
        Map(IndexMap<String, Value>),
        Vec(Vec<Value>),
        Node(ConfTree),
    }
    impl Kind {
        fn as_val(&self) -> Value {
            match self {
                Kind::Val(value) => value.to_owned(),
                _ => panic!("Kind {:?} - is not Value", self)
            }
        }
        fn as_map(&self) -> IndexMap<String, Value> {
            match self {
                Kind::Map(map) => map.to_owned(),
                _ => panic!("Kind {:?} - is not Map", self)
            }
        }
        fn as_vec(&self) -> Vec<Value> {
            match self {
                Kind::Vec(value) => value.to_owned(),
                _ => panic!("Kind {:?} - is not Vec", self)
            }
        }
        fn as_node(&self) -> ConfTree {
            match self {
                Kind::Node(value) => value.to_owned(),
                _ => panic!("Kind {:?} - is not ConfTree node", self)
            }
        }
    }
    ///
    ///
    #[test]
    fn valid() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        log::info!("conf_tree_get");
        // let (initial, switches) = init_each();
        let test_data: Vec<(&str, IndexMap<&str, Kind>)> = vec![
            (
                r#"
                    bool: true
                    f64: 64.64
                    u64: 64
                    i64: -64
                    str: str
                    map:
                        val1: 1
                        val2: 2
                        val3: 3
                    vec:
                        - 1
                        - 2
                        - 3
                    node: 
                        val4: 4
                        val5: 5
                        val6: 6
                "#,
                IndexMap::from([
                    ("bool", Kind::Val(Value::Bool(true))),
                    ("f64", Kind::Val(Value::Double(64.64))),
                    ("u64", Kind::Val(Value::Int(64))),
                    ("i64", Kind::Val(Value::Int(-64))),
                    ("str", Kind::Val(Value::String("str".to_owned()))),
                    ("map", Kind::Map(IndexMap::from([
                        ("val1".to_owned(), Value::Int(1)),
                        ("val2".to_owned(), Value::Int(2)),
                        ("val3".to_owned(), Value::Int(3)),
                    ]))),
                    ("vec", Kind::Vec(vec![
                        Value::Int(1),
                        Value::Int(2),
                        Value::Int(3),
                    ])),
                    ("node", Kind::Node(ConfTree::new(
                        "node",
                        serde_yaml::from_str(r#"
                            val4: 4
                            val5: 5
                            val6: 6
                        "#).unwrap(),
                    ))),
                ])
            ),
        ];
        for (value, targets) in test_data {
            // log::debug!("value: {:?}", value);
            let conf: serde_yaml::Value = serde_yaml::from_str(value).unwrap();
            log::trace!("conf: {:?}", conf);
            // let conf = test_data.get("/").unwrap();
            let conf = ConfTree::new_root(conf);
            log::trace!("confTree: {:?}", conf);
            let key = "bool";
            let result: bool = conf.get(key).unwrap();
            let target = targets.get(key).unwrap().as_val().as_bool();
            assert!(result == target, "key: {key} \nresult: {:?}\ntarget: {:?}", result, target);
            let key = "f64";
            let result: f64 = conf.get(key).unwrap();
            let target = targets.get(key).unwrap().as_val().as_double();
            assert!(result == target, "key: {key} \nresult: {:?}\ntarget: {:?}", result, target);
            let key = "i64";
            let result: i64 = conf.get(key).unwrap();
            let target = targets.get(key).unwrap().as_val().as_int();
            assert!(result == target, "key: {key} \nresult: {:?}\ntarget: {:?}", result, target);
            let key = "map";
            let result: serde_yaml::Mapping = conf.get(key).unwrap();
            let result: IndexMap<String, Value> = result
                .into_iter()
                .map(|(key, val)| (
                    key.as_str().unwrap().to_owned(),
                    Value::Int(val.as_i64().unwrap()),
                ))
                .collect();
            let target = targets.get(key).unwrap().as_map();
            assert!(result == target, "key: {key} \nresult: {:?}\ntarget: {:?}", result, target);
            let key = "vec";
            let result: Vec<serde_yaml::Value> = conf.get(key).unwrap();
            let result: Vec<Value> = result.into_iter().map(|item| Value::Int(item.as_i64().unwrap())).collect();
            let target = targets.get(key).unwrap().as_vec();
            assert!(result == target, "key: {key} \nresult: {:?}\ntarget: {:?}", result, target);
            let key = "str";
            let result: String = conf.get(key).unwrap();
            let target = targets.get(key).unwrap().as_val().as_string();
            assert!(result == target, "key: {key} \nresult: {:?}\ntarget: {:?}", result, target);
            let key = "u64";
            let result: u64 = conf.get(key).unwrap();
            let target = targets.get(key).unwrap().as_val().as_int() as u64;
            assert!(result == target, "key: {key} \nresult: {:?}\ntarget: {:?}", result, target);
            let key = "node";
            if let Some(result) = ConfTreeGet::<ConfTree>::get(&conf, key) {
                let target = targets.get(key).unwrap().as_node();
                assert!(result == target, "key: {key} \nresult: {:?}\ntarget: {:?}", result, target);
            } else {
                panic!("key: {key} - not found");
            };
        }
    }
}

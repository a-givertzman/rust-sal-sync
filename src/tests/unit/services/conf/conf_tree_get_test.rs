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
    ///
    #[derive(Clone, Debug, PartialEq, Eq)]
    enum Node {
        Map(IndexMap<String, Node>),
        End(ConfTree),
    }
    impl Node {
        fn as_map(&self) ->&IndexMap<String, Node> {
            match self {
                Node::Map(map) => map,
                Node::End(_) => panic!("is not a map"),
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
        let test_data: Vec<(&str, Node)> = vec![
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
                "#,
                Node::Map(IndexMap::from([
                    (format!("let newVar2"), Node::Map(IndexMap::from([
                        (format!("input"), Node::End(ConfTree { key: format!("input"), conf: serde_yaml::from_str("const 2.2").unwrap() })),
                    ]))),
                    (format!("let newVar3"), Node::Map(IndexMap::from([
                        (format!("input"), Node::End(ConfTree { key: format!("input"), conf: serde_yaml::from_str("const 3.3").unwrap() })),
                    ]))),
                    (format!("let newVar1"), Node::Map(IndexMap::from([
                        (format!("input"), Node::End(ConfTree { key: format!("input"), conf: serde_yaml::from_str("const 1.1").unwrap() })),
                    ]))),
                ]))
            ),
        ];
        for (value, target) in test_data {
            // log::debug!("test value: {:?}", value);
            let conf: serde_yaml::Value = serde_yaml::from_str(value).unwrap();
            log::debug!("test conf: {:?}", conf);
            // let conf = test_data.get("/").unwrap();
            let conf = ConfTree::new_root(conf);
            log::debug!("confTree: {:?}", conf);
            let result = inputs(&conf);
            log::debug!("result: {:?}", result);

            let val = ConfTreeGet::<bool>::get(&conf, "bool");
            let val = ConfTreeGet::<f64>::get(&conf, "f64");
            let val = ConfTreeGet::<i64>::get(&conf, "i64");
            let val = ConfTreeGet::<serde_yaml::Mapping>::get(&conf, "map");
            let val = ConfTreeGet::<Vec<serde_yaml::Value>>::get(&conf, "vec");
            let val = ConfTreeGet::<String>::get(&conf, "str");
            let val = ConfTreeGet::<u64>::get(&conf, "u64");

            // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            // let mut targets = target.as_map().iter();
            // for (_name, result) in result.as_map() {
            //     let (_, target) = targets.next().unwrap();
            //     assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            // }
        }
    }

    fn inputs(conf_tree: &ConfTree) -> Node {
        match conf_tree.sub_nodes() {
            Some(nodes) => {
                let mut res: IndexMap<String, Node> = IndexMap::new();
                for node in nodes {
                    log::debug!("key: {:?}\t|\tnode: {:?}", &node.key, &node.conf);
                    let sub_res = inputs(&node);
                    res.insert(node.key.clone(), sub_res);
                }
                return Node::Map(res)
            }
            None => {
                return Node::End(conf_tree.clone());
            }
        };
    }
    ///
    /// 
    // #[test]
    fn as_type() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::info!("test_config_tree_valid");
        // let (initial, switches) = init_each();
        let test_data: Vec<(&str, IndexMap<&str, Value>)> = vec![
            (
                r#"
                    boolTrue: true
                    boolFalse: false
                    int1: 177
                    int2: -177
                    real1: 177.3
                    real2: -177.3
                    double1: 177.3
                    double2: -177.3
                    string1: /Path/Point.Name/
                    string2: '/Path/Point.Name/'
                    string3: "/Path/Point.Name/"
                "#,
                IndexMap::from([
                    ("boolTrue", Value::Bool(true)),
                    ("boolFalse", Value::Bool(false)),
                    ("int1", Value::Int(177)),
                    ("int2", Value::Int(-177)),
                    ("real1", Value::Real(177.3)),
                    ("real2", Value::Real(-177.3)),
                    ("double1", Value::Double(177.3)),
                    ("double2", Value::Double(-177.3)),
                    ("string1", Value::String("/Path/Point.Name/".to_string())),
                    ("string2", Value::String("/Path/Point.Name/".to_string())),
                    ("string3", Value::String("/Path/Point.Name/".to_string())),
                ])
            ),

        ];
        for (value, targets) in test_data {
            // log::debug!("test value: {:?}", value);
            let conf: serde_yaml::Value = serde_yaml::from_str(value).unwrap();
            log::debug!("test conf: {:?}", conf);
            let conf_tree = ConfTree::new_root(conf);
            for (key, target) in targets {
                match target {
                    Value::Bool(target_value) => assert_eq!(conf_tree.as_bool(key).unwrap(), target_value),
                    Value::Int(target_value) => assert_eq!(conf_tree.as_i64(key).unwrap(), target_value),
                    Value::Real(target_value) => assert_eq!(conf_tree.as_f32(key).unwrap(), target_value),
                    Value::Double(target_value) => assert_eq!(conf_tree.as_f64(key).unwrap(), target_value),
                    Value::String(target_value) => assert_eq!(conf_tree.as_str(key).unwrap(), target_value),
                }
            }
        }
    }
}

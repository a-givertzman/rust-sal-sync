use std::{hash::BuildHasherDefault, str::FromStr, time::Duration};
use hashers::fx_hash::FxHasher;
use indexmap::IndexMap;
use crate::{
    collections::map::FxIndexMap,
    services::{
        conf::conf_tree::ConfTree, entity::{name::Name, point::point_config::PointConfig}, subscription::conf_subscribe::ConfSubscribe,
        task::functions::conf::fn_conf_keywd::{FnConfKeywd, FnConfKindName},
    },
};
use super::{
    conf_duration::ConfDuration, conf_keywd::ConfKeywd, conf_kind::ConfKind, conf_tree::ConfTreeGet, diag_keywd::DiagKeywd
};
///
/// Result getting parameter
pub enum ConfParam<T, E> {
    Ok(T),
    None,
    Err(E)
}
///
/// Common configuration used in the custom service config 
#[deprecated(note = "Use sal_core::services::conf::ConfTree instead")]
#[derive(Debug, PartialEq, Clone)]
pub struct ServiceConfig {
    id: String,
    pub key: String,
    conf: ConfTree,
    pub keys: Vec<String>,
}
//
//
impl ServiceConfig {
    ///
    /// Creates new instance of ServiceConfig
    pub fn new(parent: &str, conf: ConfTree) -> Self {
        let keys = conf.sub_nodes().unwrap().map(|conf| conf.key).collect();
        Self {
            id: format!("{}/ServiceConfig", parent),
            key: conf.key.clone(),
            conf,
            keys,
        }
    }
    ///
    /// returns first sub node
    pub fn first(&self) -> Option<ConfTree> {
        self.conf.next()
    }
    ///
    /// Returns ConfTree by key if found or None
    pub fn get(&self, key: &str) -> Option<ConfTree> {
        self.conf.get(key)
    }
    ///
    /// Removes key from self.keys
    fn remove_key(&mut self, name: &str) -> Result<(), String> {
        match self.keys.iter().position(|x| *x == name) {
            Some(index) => {
                self.keys.remove(index);
                Ok(())
            }
            None => Err(format!("{}.remove_key | '{}' - not found in: {:?}", self.id, name, self.conf)),
        }
    }
    ///
    /// Returns serde_yaml::Value by key and removes key
    pub fn get_param_value(&mut self, name: &str) -> Result<serde_yaml::Value, String> {
        match self.remove_key(name) {
            Ok(_) => {
                match ConfTreeGet::<serde_yaml::Value>::get( &self.conf, name) {
                    Some(val) => Ok(val),
                    None => Err(format!("{}.get_param_value | '{}' - not found in: {:?}", self.id, name, self.conf)),
                }
            }
            Err(err) => Err(err),
        }
    }
    ///
    /// Returns ConfTree by key and removes key
    pub fn get_param_conf(&mut self, name: &str) -> Result<ConfTree, String> {
        match self.remove_key(name) {
            Ok(_) => {
                match self.conf.get(name) {
                    Some(conf_tree) => Ok(conf_tree),
                    None => Err(format!("{}.get_param_conf | '{}' - not found in: {:?}", self.id, name, self.conf)),
                }
            }
            Err(err) => Err(err),
        }
    }
    ///
    /// Retuirns duration conf by key or None
    pub fn get_duration(&mut self, name: &str) -> Option<Duration> {
        match self.get_param_value(name) {
            Ok(value) => {
                let value = if value.is_u64() {
                    value.as_u64().unwrap().to_string()
                } else if value.is_string() {
                    value.as_str().unwrap().to_string()
                } else {
                    panic!("{}.get_duration | Invalid {} duration format: {:?} \n\tin: {:?}", self.id, &name, &value, self.conf)
                };
                match ConfDuration::from_str(&value) {
                    Ok(conf_duration) => {
                        Some(conf_duration.to_duration())
                    }
                    Err(err) => panic!("{}.get_duration | Parse {} duration '{}' error: {:?}", self.id, &name, &value, err),
                }
            }
            Err(_) => None,
        }
    }
    ///
    /// Returns general parameter by keyword's `prefix` and `kind`
    /// - `kind` - a kind of cofiguration entity
    ///
    /// Where keyword looks loke
    /// ```markdown
    /// | opt        | requir   |  requir     |  opt      |
    /// | ---------- | -------- | ----------- | --------- |
    /// | **prefix** | **kind** | Name        | Sufix     |
    /// |            | task     | Task        | Task1     |
    /// |            | service  | ApiClient   | ApiClient |
    /// | in         | queue    | in-queue    |           |
    /// | out        | queue    | out-queue   |           |
    /// ```
    pub fn get_param_by_keyword(&mut self, prefix: &str, kind: impl Into<String>) -> Result<(ConfKeywd, ConfTree), String> {
        let self_conf = self.conf.clone();
        let kind = kind.into();
        for node in self_conf.sub_nodes().unwrap() {
            if let Ok(keyword) = ConfKeywd::from_str(&node.key) {
                if keyword.kind() == kind && keyword.prefix() == prefix {
                    match self.remove_key(&node.key) {
                        Ok(_) => return Ok((keyword, node)),
                        Err(err) => return Err(err),
                    };
                }
            };
        };
        Err(format!("{}.get_param_by_keyword | keyword '{} {:?}' - not found", self.id, prefix, kind))
    }
    ///
    /// Returns ConfSubscribe by 'subscribe' key
    pub fn subscribe(&mut self) -> Result<ConfSubscribe, String> {
        match self.get_param_value("subscribe") {
            Ok(conf) => {
                Ok(ConfSubscribe::new(conf))
            }
            Err(err) => Err(err),
        }
    }
    ///
    /// Returns in queue name
    pub fn get_in_queue(&mut self) -> Result<(String, i64), String> {
        let prefix = "in";
        let sub_param = "max-length";
        match self.get_param_by_keyword(prefix, ConfKind::Queue) {
            Ok((keyword, self_recv_queue)) => {
                let name = format!("{} {} {}", keyword.prefix(), keyword.kind().to_string(), keyword.name());
                log::debug!("{}.get_in_queue | self in-queue params {}: {:?}", self.id, name, self_recv_queue);
                match ConfTreeGet::<serde_yaml::Value>::get(&self_recv_queue, sub_param) {
                    Some(val) => match val.as_i64() {
                        Some(max_length) => Ok((keyword.name(), max_length)),
                        None => Err(format!("{}.get_in_queue | '{}': '{:?}' - must be an integer, in conf: {:?}", self.id, name, val, self.conf)),
                    }
                    None => Err(format!("{}.get_in_queue | '{}' - not found in: {:?}", self.id, name, self.conf)),
                }
            }
            Err(err) => Err(format!("{}.get_in_queue | {} queue - not found in: {:#?}\n\terror: {:?}", self.id, prefix, self.conf, err)),
        }
    }
    ///
    /// Returns out queue name
    pub fn get_out_queue(&mut self) -> Result<String, String> {
        let prefix = "out";
        match self.get_param_by_keyword(prefix, ConfKind::Queue) {
            Ok((keyword, tx_name)) => {
                let name = format!("{} {} {}", keyword.prefix(), keyword.kind().to_string(), keyword.name());
                log::debug!("{}.get_out_queue | self out-queue params {}: {:?}", self.id, name, tx_name);
                Ok(tx_name.conf.as_str().unwrap().to_string())
            }
            Err(err) => Err(format!("{}.get_out_queue | {} queue - not found in: {:#?}\n\terror: {:?}", self.id, prefix, self.conf, err)),
        }
    }
    ///
    /// Returns name of the 'send-to' queue
    pub fn get_send_to(&mut self) -> Result<String, String> {
        match self.get_param_value("send-to") {
            Ok(conf) => {
                Ok(conf.as_str().unwrap().to_string())
            }
            Err(err) => Err(format!("{}.get_send_to | 'send-to' - not found in: {:#?}\n\terror: {:#?}", self.id, self.conf, err)),
        }
    }
    ///
    /// Returns vec of names of the 'send-to' queue
    pub fn get_send_to_many(&mut self) -> ConfParam<Vec<String>, String> {
        match self.get_param_value("send-to") {
            Ok(conf) => {
                match conf {
                    serde_yaml::Value::Null => {
                        log::warn!("{}.get_send_to_many | Parameter 'send-to' - is empty", self.id);
                        ConfParam::Ok(vec![])
                    }
                    serde_yaml::Value::Sequence(conf) => {
                        let mut items = vec![];
                        for item in conf.iter() {
                            match item.as_str() {
                                Some(item) => items.push(item.to_owned()),
                                None => return ConfParam::Err(format!("{}.get_send_to_many | Array<String> expected in 'send-to', but found: {:#?}", self.id, conf)),
                            }
                        }
                        ConfParam::Ok(items)
                    }
                    _ => ConfParam::Err(format!("{}.get_send_to_many | Array<String> expected in 'send-to', but found: {:#?}", self.id, conf)),
                }
            }
            Err(_) => ConfParam::None,
        }
    }
    ///
    /// Returns diagnosis point configs
    pub fn get_diagnosis(&mut self, parent: &Name) -> FxIndexMap<DiagKeywd, PointConfig> {
        let mut points = IndexMap::with_hasher(BuildHasherDefault::<FxHasher>::default());
        match self.get_param_conf("diagnosis") {
            Ok(conf) => {
                let diag_node_conf = ServiceConfig::new(&self.id, conf);
                for key in &diag_node_conf.keys {
                    let keyword = FnConfKeywd::from_str(key).unwrap();
                    if keyword.kind() == FnConfKindName::Point {
                        let point_name = Name::new(parent, keyword.data()).join();
                        let point_conf = diag_node_conf.get(key).unwrap();
                        log::trace!("{}.get_diagnosis | Point '{}'", self.id, point_name);
                        let point = PointConfig::new(parent, &point_conf);
                        let point_name_keywd = DiagKeywd::new(&point.name);
                        points.insert(point_name_keywd, point);
                    } else {
                        log::warn!("{}.get_diagnosis | point conf expected, but found: {:?}", self.id, keyword);
                    }
                }

            }
            Err(err) => {
                log::warn!("{}.get_diagnosis | diagnosis - not found in {:#?},\n\terror: {:#?}", self.id, self.conf, err);
            }
        };
        points
    }
}

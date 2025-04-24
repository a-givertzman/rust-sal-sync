use std::{str::FromStr, time::Duration};
use sal_core::error::Error;
use serde::de::DeserializeOwned;
use crate::{
    collections::FxIndexMap,
    services::{
        entity::{Name, PointConfig},
        task::functions::conf::fn_conf_keywd::{FnConfKeywd, FnConfKindName},
    },
};
use super::{conf_duration::ConfDuration, conf_keywd::ConfKeywd, conf_kind::ConfKind, diag_keywd::DiagKeywd};
///
/// ConfTree holds sede_yaml::Value and it key
/// for root key = ""
/// Allow to iterate across all yaml config nodes
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfTree {
    id: String,
    pub key: String,
    pub conf: serde_yaml::Value,
}
//
// 
impl ConfTree {
    ///
    /// creates iterotor on the serde_yaml::Value mapping
    pub fn new_root(conf: serde_yaml::Value) -> Self {
        Self {
            id: String::from("ConfTree"),
            key: String::new(),
            conf,
        }
    }
    ///
    /// creates ConfTree instance holding the key and serde_yaml::Value
    pub fn new(key: &str, conf: serde_yaml::Value) -> Self {
        Self {
            id: String::from("ConfTree"),
            key: key.into(),
            conf,
        }
    }
    ///
    /// returns true if holding mapping 
    pub fn is_mapping(&self) -> bool {
        self.conf.is_mapping()
    }
    ///
    /// iterates across all sub nodes 
    pub fn next(&self) -> Option<ConfTree> {
        match self.sub_nodes() {
            Some(mut sub_nodes) => sub_nodes.next(),
            None => None,
        }
    }
    ///
    /// returns count of sub nodes
    pub fn count(&self) -> usize {
        match self.sub_nodes() {
            Some(sub_nodes) => sub_nodes.count(),
            None => 0,
        }
    }
    ///
    /// iterate across all sub nodes
    pub fn sub_nodes(&self) -> Option<impl Iterator<Item = ConfTree> + '_> {
        if self.conf.is_mapping() {
            let iter = self.conf.as_mapping().unwrap().into_iter().map( |(key, value)| {
                ConfTree::new(
                    key.as_str().unwrap(),
                    value.clone(),
                )
            });
            Some(iter)
        } else {
            None
        }
    }
    ///
    /// Returns keys, excluding specified
    /// - `exclude` - list of keys to be filtered
    pub fn keys(&self, exclude: &[impl AsRef<str>]) -> Vec<String> {
        let exclude: Vec<&str> = exclude.iter().map(|key| key.as_ref()).collect();
        self.conf.as_mapping().map(|nodes| {
            nodes.keys().filter_map(|key| {
                match key.as_str() {
                    Some(key) => if exclude.contains(&key) {
                        None
                    } else {
                        Some(key.to_owned())
                    }
                    None => None,
                }
            }).collect()
        })
        .unwrap_or(vec![])
    }
    ///
    /// Returns prefix field
    /// ```markdown
    /// | opt        | requir   |  requir     |  opt      |
    /// | ---------- | -------- | ----------- | --------- |
    /// | **prefix** | kind     | Name        | Sufix     |
    /// ```
    /// 
    /// Will parsed from self `key` as [ConfKeywd]
    pub fn prefix(&self) -> Result<String, Error> {
        let error = Error::new(&self.id, "prefix");
        match ConfKeywd::from_str(&self.key) {
            Ok(keywd) => {
                log::trace!("ConfTree.prefix | Keyword: {:?}", keywd);
                Ok(keywd.prefix())
            }
            Err(err) => Err(error.err(format!("Error in {:?}: \n\t{:?}", self.key, err))),
        }
    }
    ///
    /// Returns `kind` field
    /// ```markdown
    /// | opt        | requir   |  requir     |  opt      |
    /// | ---------- | -------- | ----------- | --------- |
    /// | prefix     | **kind** | Name        | Sufix     |
    /// ```
    /// 
    /// Will parsed from self `key` as [ConfKeywd]
    pub fn kind(&self) -> Result<String, Error> {
        let error = Error::new(&self.id, "kind");
        match ConfKeywd::from_str(&self.key) {
            Ok(keywd) => {
                log::trace!("ConfTree.kind | Keyword: {:?}", keywd);
                Ok(keywd.kind())
            }
            Err(err) => Err(error.err(format!("Error in {:?}: \n\t{:?}", self.key, err))),
        }
    }
    ///
    /// Returns `name` field 
    /// ```markdown
    /// | opt        | requir   |  requir     |  opt      |
    /// | ---------- | -------- | ----------- | --------- |
    /// | prefix     | kind     | **Name**    | Sufix     |
    /// ```
    /// 
    /// Will parsed from self `key` as [ConfKeywd]
    pub fn name(&self) -> Result<String, Error> {
        let error = Error::new(&self.id, "name");
        match ConfKeywd::from_str(&self.key) {
            Ok(keywd) => {
                log::trace!("ConfTree.name | Keyword: {:?}", keywd);
                Ok(keywd.name())
            }
            Err(err) => Err(error.err(format!("Error in {:?}: \n\t{:?}", self.key, err))),
        }
    }
    ///
    /// Returns `sufix` field
    /// ```markdown
    /// | opt        | requir   |  requir     |  opt      |
    /// | ---------- | -------- | ----------- | --------- |
    /// | prefix     | kind     | Name        | **Sufix** |
    /// ```
    /// 
    /// Will parsed from self `key` as [ConfKeywd]
    pub fn sufix(&self) -> Result<String, Error> {
        let error = Error::new(&self.id, "sufix");
        match ConfKeywd::from_str(&self.key) {
            Ok(keywd) => {
                log::trace!("ConfTree.sufix | Keyword: {:?}", keywd);
                Ok(keywd.sufix())
            }
            Err(err) => Err(error.err(format!("Error in {:?}: \n\t{:?}", self.key, err))),
        }
    }
    ///
    /// Returns `sufix` field if exists and not empty or default
    /// ```markdown
    /// | opt        | requir   |  requir     |  opt      |
    /// | ---------- | -------- | ----------- | --------- |
    /// | prefix     | kind     | Name        | **Sufix** |
    /// ```
    /// 
    /// Will parsed from self `key` as [ConfKeywd]
    pub fn sufix_or(&self, default: impl Into<String>) -> String {
        match self.sufix() {
            Ok(sufix) => if sufix.is_empty() {
                default.into()
            } else {
                sufix
            }
            Err(_) => default.into(),
        }
    }
    ///
    /// returns tree node value as bool by it's key if exists
    pub fn as_bool(&self, key: &str) -> Result<bool, Error> {
        let error = Error::new(&self.id, "as_bool");
        if self.conf.is_mapping() {
            match self.conf.as_mapping().unwrap().get(key) {
                Some(value) => {
                    match value.as_bool() {
                        Some(value) => Ok(value),
                        None => Err(error.err(format!("error getting BOOL by key '{:?}' from node '{:?}'", &key, value))),
                    }
                }
                None => Err(error.err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf))),
            }
        } else {
            Err(error.err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)))
        }
    }
    ///
    /// returns tree node value as bool by it's key if exists
    pub fn as_i64(&self, key: &str) -> Result<i64, Error> {
        let error = Error::new(&self.id, "as_i64");
        if self.conf.is_mapping() {
            match self.conf.as_mapping().unwrap().get(key) {
                Some(value) => {
                    match value.as_i64() {
                        Some(value) => Ok(value),
                        None => Err(error.err(format!("error getting INT by key '{:?}' from node '{:?}'", &key, value))),
                    }
                }
                None => Err(error.err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf))),
            }
        } else {
            Err(error.err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)))
        }
    }
    ///
    /// returns tree node value as f32 by it's key if exists
    pub fn as_f32(&self, key: &str) -> Result<f32, Error> {
        let error = Error::new(&self.id, "as_f32");
        if self.conf.is_mapping() {
            match self.conf.as_mapping().unwrap().get(key) {
                Some(value) => {
                    match value.as_f64() {
                        Some(value) => Ok(value as f32),
                        None => Err(error.err(format!("error getting REAL by key '{:?}' from node '{:?}'", &key, value))),
                    }
                }
                None => Err(error.err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf))),
            }
        } else {
            Err(error.err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)))
        }
    }
    ///
    /// returns tree node value as f64 by it's key if exists
    pub fn as_f64(&self, key: &str) -> Result<f64, Error> {
        let error = Error::new(&self.id, "as_f64");
        if self.conf.is_mapping() {
            match self.conf.as_mapping().unwrap().get(key) {
                Some(value) => {
                    match value.as_f64() {
                        Some(value) => Ok(value),
                        None => Err(error.err(format!("error getting DOUBLE by key '{:?}' from node '{:?}'", &key, value))),
                    }
                }
                None => Err(error.err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf))),
            }
        } else {
            Err(error.err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)))
        }
    }
    ///
    /// returns tree node value as str by it's key if exists
    pub fn as_str(&self, key: &str) -> Result<&str, Error> {
        let error = Error::new(&self.id, "as_str");
        if self.conf.is_mapping() {
            match self.conf.as_mapping().unwrap().get(key) {
                Some(value) => {
                    match value.as_str() {
                        Some(value) => Ok(value),
                        None => Err(error.err(format!("Error getting STRING by key '{:?}' from node '{:?}'", &key, value))),
                    }
                }
                None => Err(error.err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf))),
            }
        } else {
            Err(error.err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)))
        }
    }
    ///
    /// removes node by it's key if exists
    /// returns Result<&Self>
    pub fn remove(&mut self, key: &str) -> Result<serde_yaml::Value, Error> {
        let error = Error::new(&self.id, "remove");
        if self.conf.is_mapping() {
            match self.conf.as_mapping_mut().unwrap().remove(key) {
                Some(value) => Ok(value),
                None => Err(error.err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf))),
            }
        } else {
            Err(error.err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)))
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
    pub fn get_by_keywd(&self, prefix: &str, kind: impl Into<String>) -> Result<(ConfKeywd, ConfTree), Error> {
        let self_conf = self.clone();
        let kind = kind.into();
        let error = Error::new(&self.id, "get_by_keywd");
        for node in self_conf.sub_nodes().unwrap() {
            if let Ok(keyword) = ConfKeywd::from_str(&node.key) {
                if keyword.kind() == kind && keyword.prefix() == prefix {
                    return Ok((keyword, node))
                }
            }
        }
        Err(error.err(format!("keyword '{} {:?}' - not found", prefix, kind)))
    }
    ///
    /// Returns `in queue` name
    pub fn get_in_queue(&self) -> Result<(String, i64), Error> {
        let prefix = "in";
        let sub_param = "max-length";
        let error = Error::new(&self.id, "get_in_queue");
        match self.get_by_keywd(prefix, ConfKind::Queue) {
            Ok((keyword, self_recv_queue)) => {
                let name = format!("{} {} {}", keyword.prefix(), keyword.kind().to_string(), keyword.name());
                log::trace!("{}.get_in_queue | self in-queue params {}: {:?}", self.id, name, self_recv_queue);
                match ConfTreeGet::<serde_yaml::Value>::get(&self_recv_queue, sub_param) {
                    Some(val) => match val.as_i64() {
                        Some(max_length) => Ok((keyword.name(), max_length)),
                        None => Err(error.err(format!("'{}': '{:?}' - must be an integer, in conf: {:?}", name, val, self.conf))),
                    }
                    None => Err(error.err(format!("'{}' - not found in: {:?}", name, self.conf))),
                }
            }
            Err(err) => Err(error.pass_with(format!("{} queue - not found in: {:#?}\n\terror: {:?}", prefix, self.conf, err), err)),
        }
    }
    ///
    /// Returns out queue name
    #[deprecated(note = "Use ConfTree::get_send_to instead")]
    pub fn get_out_queue(&self) -> Result<String, Error> {
        let prefix = "out";
        let error = Error::new(&self.id, "get_out_queue");
        match self.get_by_keywd(prefix, ConfKind::Queue) {
            Ok((keyword, tx_name)) => {
                let name = format!("{} {} {}", keyword.prefix(), keyword.kind().to_string(), keyword.name());
                log::trace!("{}.get_out_queue | self out-queue params {}: {:?}", self.id, name, tx_name);
                Ok(tx_name.conf.as_str().unwrap().to_string())
            }
            Err(err) => Err(error.err(format!("{} queue - not found in: {:#?}\n\terror: {:?}", prefix, self.conf, err))),
        }
    }
    ///
    /// Returns vec of names of the 'send-to' queue
    pub fn get_send_to_many(&self) -> Result<Vec<String>, Error> {
        let error = Error::new(&self.id, "get_send_to_many");
        match ConfTreeGet::<serde_yaml::Value>::get(self, "send-to") {
            Some(conf) => {
                match conf {
                    serde_yaml::Value::Null => {
                        Err(error.err("Parameter 'send-to' - is empty"))
                    }
                    serde_yaml::Value::Sequence(conf) => {
                        let mut items = vec![];
                        for item in conf.iter() {
                            match item.as_str() {
                                Some(item) => items.push(item.to_owned()),
                                None => log::warn!("{}.get_send_to_many | In parameter 'send-to' String's expected , but found: {:#?}", self.id, item),
                            }
                        }
                        Ok(items)
                    }
                    _ => {
                        Err(error.err(format!("In parameter 'send-to' expected Array of String, but found: {:#?}", conf)))
                    }
                }
            }
            None => Err(error.err("Parameter 'send-to' - is not found")),
        }
    }
    ///
    /// Returns Type by `key`, parsed from serde_yaml
    pub fn parse<T: DeserializeOwned + std::fmt::Debug>(&self, key: impl AsRef<str>) -> Result<T, Error> {
        let error = Error::new(&self.id, "parse");
        let val = self.conf
            .get(key.as_ref())
            .ok_or(error.err(format!("key '{}' - not found in: {:#?}", key.as_ref(), self.conf)))?;
        let val = serde_yaml::from_value::<T>(val.to_owned())
        .map_err(|err| error.err(format!("key '{}' - parse error: {:?} in: {:#?}", key.as_ref(), err, self.conf)));
        log::trace!("ConfTree.parse | {}: {:#?}", key.as_ref(), val);
        val
    }
    ///
    /// Retuirns duration conf by key or Error
    /// 
    /// ```yaml
    /// duration: 10ms      # 10 milliseconds
    /// interval: 100us     # 100 microseconds
    /// timeout: 3s         # 3 seconds
    /// ```
    pub fn get_duration(&self, key: impl AsRef<str>) -> Result<Duration, Error> {
        let error = Error::new(&self.id, "get_duration");
        match ConfTreeGet::<serde_yaml::Value>::get(self, key.as_ref()) {
            Some(value) => {
                let value = if value.is_u64() {
                    value.as_u64().unwrap().to_string()
                } else if value.is_string() {
                    value.as_str().unwrap().to_string()
                } else {
                    return Err(error.err(format!("Invalid {} duration format: {:?} \n\tin: {:?}", key.as_ref(), &value, self.conf)));
                };
                match ConfDuration::from_str(&value) {
                    Ok(conf_duration) => {
                        Ok(conf_duration.to_duration())
                    }
                    Err(err) => Err(error.err(format!("Parse {} duration '{}' error: {:?}", key.as_ref(), &value, err))),
                }
            }
            None => Err(error.err(format!("Key {} - not found in: {:#?}", key.as_ref(), self.conf))),
        }
    }
    ///
    /// Returns diagnosis point config's
    /// 
    /// ```yaml
    /// diagnosis:                          # internal diagnosis
    ///     point Status:                   # Ok(0) / Invalid(10)
    ///         type: 'Int'
    ///         # history: r                # r / rw - activates history
    ///     point Connection:               # Ok(0) / Invalid(10)
    ///         type: 'Int'
    ///         # history: r                # r / rw - activates history
    /// ```
    pub fn get_diagnosis(&self, parent: impl Into<String>) -> FxIndexMap<DiagKeywd, PointConfig> {
        let mut points = FxIndexMap::default();
        let parent = parent.into();
        match ConfTreeGet::<ConfTree>::get(self, "diagnosis") {
            Some(conf) => {
                for key in conf.keys(&[] as &[&str]) {
                    let keyword = FnConfKeywd::from_str(&key).unwrap();
                    if keyword.kind() == FnConfKindName::Point {
                        let point_name = Name::new(&parent, keyword.data()).join();
                        let point_conf = conf.get(key).unwrap();
                        log::trace!("{}.get_diagnosis | Point '{}'", self.id, point_name);
                        let point = PointConfig::new(&parent, &point_conf);
                        let point_name_keywd = DiagKeywd::new(&point.name);
                        points.insert(point_name_keywd, point);
                    } else {
                        log::warn!("{}.get_diagnosis | point conf expected, but found: {:?}", self.id, keyword);
                    }
                }

            }
            None => {
                log::warn!("{}.get_diagnosis | diagnosis - not found in {:#?}", self.id, self.conf);
            }
        };
        points
    }
}

///
/// Provides generic access to the containing values by a key
pub trait ConfTreeGet<T> {
    ///
    /// Returns a value by it key
    /// 
    /// # Panics
    /// Function Will panic 
    /// - if requested key does not exists
    /// - if the type of value is not matched to the requested
    fn get(&self, key: impl AsRef<str>) -> Option<T>;
}
//
//
impl ConfTreeGet<ConfTree> for ConfTree {
    ///
    /// Returns a sub-node by it's key if exists, else None
    fn get(&self, key: impl AsRef<str>) -> Option<ConfTree> {
        if self.conf.is_mapping() {
            self.conf.as_mapping().unwrap().get(key.as_ref()).map(|value| ConfTree {
                id: String::from("ConfTree"),
                key: key.as_ref().to_owned(),
                conf: value.clone(),
            })
        } else {
            None
        }
    }
}
//
//
impl ConfTreeGet<serde_yaml::Value> for ConfTree {
    fn get(&self, key: impl AsRef<str>) -> Option<serde_yaml::Value> {
        let val = self.conf.get(key.as_ref()).map(|val| val.to_owned());
        log::trace!("ConfTree.get | {}: {:#?}", key.as_ref(), val);
        val
    }
}
//
//
impl ConfTreeGet<bool> for ConfTree {
    fn get(&self, key: impl AsRef<str>) -> Option<bool> {
        let val = match self.conf.get(key.as_ref()) {
            Some(val) => val.as_bool(),
            None => None,
        };
        log::trace!("ConfTree.get | {}: {:?}", key.as_ref(), val);
        val
    }
}
//
//
impl ConfTreeGet<f64> for ConfTree {
    fn get(&self, key: impl AsRef<str>) -> Option<f64> {
        let val = match self.conf.get(key.as_ref()) {
            Some(val) => val.as_f64(),
            None => None,
        };
        log::trace!("ConfTree.get | {}: {:?}", key.as_ref(), val);
        val
    }
}
//
//
impl ConfTreeGet<i64> for ConfTree {
    fn get(&self, key: impl AsRef<str>) -> Option<i64> {
        let val = match self.conf.get(key.as_ref()) {
            Some(val) => val.as_i64(),
            None => None,
        };
        log::trace!("ConfTree.get | {}: {:?}", key.as_ref(), val);
        val
    }
}
//
//
impl ConfTreeGet<serde_yaml::Mapping> for ConfTree {
    fn get(&self, key: impl AsRef<str>) -> Option<serde_yaml::Mapping> {
        let val = match self.conf.get(key.as_ref()) {
            Some(val) => val.as_mapping().map(|val| val.to_owned()),
            None => None,
        };
        log::trace!("ConfTree.get | {}: {:#?}", key.as_ref(), val);
        val
    }
}
//
//
impl ConfTreeGet<Vec<serde_yaml::Value>> for ConfTree {
    fn get(&self, key: impl AsRef<str>) -> Option<Vec<serde_yaml::Value>> {
        let val = match self.conf.get(key.as_ref()) {
            Some(val) => val.as_sequence().map(|val| val.to_owned()),
            None => None,
        };
        log::trace!("ConfTree.get | {}: {:#?}", key.as_ref(), val);
        val
    }
}
//
//
impl ConfTreeGet<String> for ConfTree {
    fn get(&self, key: impl AsRef<str>) -> Option<String> {
        let val = match self.conf.get(key.as_ref()) {
            Some(val) => val.as_str().map(|val| val.to_owned()),
            None => None,
        };
        log::trace!("ConfTree.get | {}: {:?}", key.as_ref(), val);
        val
    }
}
//
//
impl ConfTreeGet<u64> for ConfTree {
    fn get(&self, key: impl AsRef<str>) -> Option<u64> {
        let val = match self.conf.get(key.as_ref()) {
            Some(val) => val.as_u64(),
            None => None,
        };
        log::trace!("ConfTree.get | {}: {:?}", key.as_ref(), val);
        val
    }
}

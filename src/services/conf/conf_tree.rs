use std::{collections::HashSet, str::FromStr};
use sal_core::error::Error;
use super::{conf_keywd::ConfKeywd, conf_kind::ConfKind};
///
/// ConfTree holds sede_yaml::Value and it key
/// for root key = ""
/// Allow to iterate across all yaml config nodes
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfTree {
    id: String,
    pub key: String,
    pub conf: serde_yaml::Value,
    /// keys of nodes of current conf, which was olready requested
    requested: HashSet<String>,
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
            requested: HashSet::new()
        }
    }
    ///
    /// creates ConfTree instance holding the key and serde_yaml::Value
    pub fn new(key: &str, conf: serde_yaml::Value) -> Self {
        Self {
            id: String::from("ConfTree"),
            key: key.into(),
            conf,
            requested: HashSet::new()
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
    /// Returns keys, which has not requested yet
    pub fn keys(&self) -> Vec<String> {
        self.conf.as_mapping().map(|nodes| {
            nodes.keys().filter_map(|key| {
                match key.as_str() {
                    Some(key) => if self.requested.contains(key) {
                        Some(key.to_owned())
                    } else {
                        None
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
        match ConfKeywd::from_str(&self.key) {
            Ok(keywd) => {
                log::trace!("ConfTree.prefix | Keyword: {:?}", keywd);
                Ok(keywd.prefix())
            }
            Err(err) => Err(Error::new("ConfTree", "prefix").err(format!("Error in {:?}: \n\t{:?}", self.key, err))),
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
        match ConfKeywd::from_str(&self.key) {
            Ok(keywd) => {
                log::trace!("ConfTree.kind | Keyword: {:?}", keywd);
                Ok(keywd.kind())
            }
            Err(err) => Err(Error::new("ConfTree", "prefix").err(format!("Error in {:?}: \n\t{:?}", self.key, err))),
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
        match ConfKeywd::from_str(&self.key) {
            Ok(keywd) => {
                log::trace!("ConfTree.name | Keyword: {:?}", keywd);
                Ok(keywd.name())
            }
            Err(err) => Err(Error::new("ConfTree", "prefix").err(format!("Error in {:?}: \n\t{:?}", self.key, err))),
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
        match ConfKeywd::from_str(&self.key) {
            Ok(keywd) => {
                log::trace!("ConfTree.sufix | Keyword: {:?}", keywd);
                Ok(keywd.sufix())
            }
            Err(err) => Err(Error::new("ConfTree", "prefix").err(format!("Error in {:?}: \n\t{:?}", self.key, err))),
        }
    }
    ///
    /// returns tree node value as bool by it's key if exists
    pub fn as_bool(&self, key: &str) -> Result<bool, String> {
        if self.conf.is_mapping() {
            match self.conf.as_mapping().unwrap().get(key) {
                Some(value) => {
                    match value.as_bool() {
                        Some(value) => Ok(value),
                        None => Err(format!("error getting BOOL by key '{:?}' from node '{:?}'", &key, value)),
                    }
                }
                None => Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)),
            }
        } else {
            Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf))
        }
    }
    ///
    /// returns tree node value as bool by it's key if exists
    pub fn as_i64(&self, key: &str) -> Result<i64, String> {
        if self.conf.is_mapping() {
            match self.conf.as_mapping().unwrap().get(key) {
                Some(value) => {
                    match value.as_i64() {
                        Some(value) => Ok(value),
                        None => Err(format!("error getting INT by key '{:?}' from node '{:?}'", &key, value)),
                    }
                }
                None => Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)),
            }
        } else {
            Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf))
        }
    }
    ///
    /// returns tree node value as f32 by it's key if exists
    pub fn as_f32(&self, key: &str) -> Result<f32, String> {
        if self.conf.is_mapping() {
            match self.conf.as_mapping().unwrap().get(key) {
                Some(value) => {
                    match value.as_f64() {
                        Some(value) => Ok(value as f32),
                        None => Err(format!("error getting REAL by key '{:?}' from node '{:?}'", &key, value)),
                    }
                }
                None => Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)),
            }
        } else {
            Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf))
        }
    }
    ///
    /// returns tree node value as f64 by it's key if exists
    pub fn as_f64(&self, key: &str) -> Result<f64, String> {
        if self.conf.is_mapping() {
            match self.conf.as_mapping().unwrap().get(key) {
                Some(value) => {
                    match value.as_f64() {
                        Some(value) => Ok(value),
                        None => Err(format!("error getting DOUBLE by key '{:?}' from node '{:?}'", &key, value)),
                    }
                }
                None => Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)),
            }
        } else {
            Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf))
        }
    }
    ///
    /// returns tree node value as str by it's key if exists
    pub fn as_str(&self, key: &str) -> Result<&str, String> {
        if self.conf.is_mapping() {
            match self.conf.as_mapping().unwrap().get(key) {
                Some(value) => {
                    match value.as_str() {
                        Some(value) => Ok(value),
                        None => Err(format!("Error getting STRING by key '{:?}' from node '{:?}'", &key, value)),
                    }
                }
                None => Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)),
            }
        } else {
            Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf))
        }
    }
    ///
    /// removes node by it's key if exists
    /// returns Result<&Self>
    pub fn remove(&mut self, key: &str) -> Result<serde_yaml::Value, String> {
        if self.conf.is_mapping() {
            match self.conf.as_mapping_mut().unwrap().remove(key) {
                Some(value) => Ok(value),
                None => Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)),
            }
        } else {
            Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf))
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
    pub fn get_by_keyword(&mut self, prefix: &str, kind: impl Into<String>) -> Result<(ConfKeywd, ConfTree), Error> {
        let self_conf = self.clone();
        let kind = kind.into();
        for node in self_conf.sub_nodes().unwrap() {
            if let Ok(keyword) = ConfKeywd::from_str(&node.key) {
                if keyword.kind() == kind && keyword.prefix() == prefix {
                    self.requested.insert(node.key.clone());
                    return Ok((keyword, node))
                }
            }
        }
        Err(Error::new(&self.id, "get_by_keyword").err(format!("keyword '{} {:?}' - not found", prefix, kind)))
    }
    ///
    /// Returns `in queue` name
    pub fn get_in_queue(&mut self) -> Result<(String, i64), String> {
        let prefix = "in";
        let sub_param = "max-length";
        match self.get_by_keyword(prefix, ConfKind::Queue) {
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
    /// Returns `value` by 'send-to' key
    pub fn get_send_to(&mut self) -> Result<String, Error> {
        match ConfTreeGet::<serde_yaml::Value>::get(self, "send-to") {
            Some(conf) => {
                self.requested.insert("send-to".into());
                Ok(conf.as_str().unwrap().to_string())
            }
            None => Err(Error::new(&self.id, "get_send_to").err(format!("'send-to' - not found in: {:#?}", self.conf))),
        }
    }
    ///
    /// Returns vec of names of the 'send-to' queue
    pub fn get_send_to_many(&mut self) -> Option<Vec<String>> {
        match ConfTreeGet::<serde_yaml::Value>::get(self, "send-to") {
            Some(conf) => {
                match conf {
                    serde_yaml::Value::Null => {
                        log::warn!("{}.get_send_to_many | Parameter 'send-to' - is empty", self.id);
                        None
                    }
                    serde_yaml::Value::Sequence(conf) => {
                        let mut items = vec![];
                        for item in conf.iter() {
                            match item.as_str() {
                                Some(item) => items.push(item.to_owned()),
                                None => log::warn!("{}.get_send_to_many | Array<String> expected in 'send-to', but found: {:#?}", self.id, conf),
                            }
                        }
                        Some(items)
                    }
                    _ => {
                        log::warn!("{}.get_send_to_many | Array<String> expected in 'send-to', but found: {:#?}", self.id, conf);
                        None
                    }
                }
            }
            None => None,
        }
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
                requested: HashSet::new(),
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
        log::debug!("ConfTree.get | {}: {:#?}", key.as_ref(), val);
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
        log::debug!("ConfTree.get | {}: {:?}", key.as_ref(), val);
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
        log::debug!("ConfTree.get | {}: {:?}", key.as_ref(), val);
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
        log::debug!("ConfTree.get | {}: {:?}", key.as_ref(), val);
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
        log::debug!("ConfTree.get | {}: {:#?}", key.as_ref(), val);
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
        log::debug!("ConfTree.get | {}: {:#?}", key.as_ref(), val);
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
        log::debug!("ConfTree.get | {}: {:?}", key.as_ref(), val);
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
        log::debug!("ConfTree.get | {}: {:?}", key.as_ref(), val);
        val
    }
}

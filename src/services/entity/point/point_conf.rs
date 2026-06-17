use std::{collections::HashMap, str::FromStr};
use log::trace;
use serde::{Serialize, Deserialize};
use crate::services::{
    // conf::ConfTree,
    // fn_::fn_conf_keywd::FnConfKeywd,
    conf::ConfTree, entity::{
        Name, PointType, point::{
            point_conf_address::PointConfAddress, 
            point_conf_filters::PointConfFilter, 
        }
    }, task::functions::FnConfKeywd
};
use super::point_conf_history::PointConfHistory;
///
/// The configuration of the Point
///  - id - unique identificator for database;
///  - name - unique /path/name for exchanging with clients and between services;
///  - _type - the type of the holding value, suporting: Bool, Int, Real, Double, String;
///  - history - flag, meaning if the point has to be stored into the historian database, 
///     - r - read direction, points hawing Cot::Inf, Cot::ActCon, Cot::ActErr, Cot::ReqCon, Cot::ReqErr
///     - w - write direction, points hawing Cot::Req, Cot::Act
///     - rw - both directions
///  - alarm - flag, meaning if point have alarm class 0..15
///     - 0 - or ommited, alarm class is none, normal information point
///     - >0 - point contains alarm information of the corresponding alarm class
///  - address - protocol specific addres
///  - filters - threshold filters
///  - comment - description text
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointConf {
    #[serde(skip)]
    pub id: usize,
    #[serde(skip)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(alias = "type", alias = "Type")]
    pub type_: PointType,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_none")]
    pub history: PointConfHistory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PointConfAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<PointConfFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}
///
/// 
fn is_none<T: Default + PartialEq>(t: &T) -> bool {
    t == &Default::default()
}
//
// 
impl PointConf {
    ///
    /// creates PointConf from serde_yaml::Value of following format:
    /// ```yaml
    /// PointName:
    ///     id: usize               # unique identificator for database
    ///     type: bool              # bool / int / real / string / json
    ///     alarm: 0                # 0..15
    ///     history: r              # ommit - None / r - Read / w - Write / rw - ReadWrite (Optional)
    ///     address:                # Protocol-specific address in the source device (Optional)
    ///         offset: 0..65535    #   0..65535
    ///         bit: 0..255         #   0..255 (Optional)
    ///     filter:                 # Filter conf, using such filter, point can be filtered immediately after input's parser
    ///         threshold: 0.5      #   absolute threshold delta
    ///         factor: 1.5         #   multiplier for absolute threshold delta - in this case the delta will be accumulated
    ///     comment: Test Point 
    /// ```
    pub fn new(parent: impl Into<String>, conf_tree: &ConfTree) -> Self {
        trace!("PointConf.new | confTree: {:?}", conf_tree);
        let mut pc: PointConf = serde_yaml::from_value(conf_tree.conf.clone()).unwrap();
        let keyword = FnConfKeywd::from_str(&conf_tree.key);
        let name = match keyword {
            Ok(keyword) => keyword.data(),
            Err(_) => conf_tree.key.clone(),
        };
        pc.name = Name::new(parent, name).join();
        if let Some(mut filter) = pc.filters.clone() {
            if let Some(factor) = filter.factor {
                if factor == 0.0 {
                    filter.factor = None
                }
            }
        }
        pc
    }    
    ///
    /// Creates config from serde_yaml::Value of following format:
    pub fn from_yaml(parent: impl Into<String>, value: &serde_yaml::Value) -> Self {
        trace!("PointConf.from_yaml | value: {:?}", value);
        Self::new(parent, &ConfTree::new_root(value.clone()).next().unwrap())
    }
    ///
    /// Returns yaml representation
    pub fn to_yaml(&self) -> serde_yaml::Value {
        let result: serde_yaml::Value = serde_yaml::to_value(self).unwrap();
        let mut wrap = HashMap::new();
        wrap.insert(self.name.clone(), result);
        serde_yaml::to_value(wrap).unwrap()
    }
    ///
    /// Converts json into PointConf
    pub fn from_json(name: &str, value: &serde_json::Value) -> Result<Self, String> {
        trace!("PointConf.from_json | value {:#?}", value);
        match serde_json::from_value(value.clone()) {
            Ok(map) => {
                let  mut map: Self = map;
                map.name = name.to_owned();
                Ok(map)
            }
            Err(err) => Err(format!("PointConf.from_json | Error: {:?}", err)),
        }
    }
    ///
    /// Returns json containing internally taggged PointConf
    pub fn to_json(&self) -> serde_json::Value {
        let result: serde_json::Value = serde_json::to_value(self).unwrap();
        let mut wrap = HashMap::new();
        wrap.insert(self.name.clone(), result);
        serde_json::to_value(wrap).unwrap()
    }
}

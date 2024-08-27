use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use super::retain_point_conf::RetainPointConf;
///
/// Retain configuration parameters and tools
/// - `path` - location of the retained values, something like assets/retain/
/// - `point` - store / load Point Id
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetainConf {
    pub path: Option<PathBuf>,
    pub point: Option<RetainPointConf>,
}
//
//
impl Default for RetainConf {
    fn default() -> Self {
        Self {
            path: Some(PathBuf::from("assets/retain/")),
            point: Some(RetainPointConf::default())
        }
    }
}
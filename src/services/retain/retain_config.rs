use serde::{Deserialize, Serialize};
use super::retain_point_conf::RetainPointConf;
///
/// Retain configuration parameters and tools
/// - 'point' - store / load Point Id
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetainConfig {
    point: Option<RetainPointConf>,
}
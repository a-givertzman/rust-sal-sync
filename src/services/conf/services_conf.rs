use sal_core::Error;
use serde::{Deserialize, Serialize};
use crate::services::retain::retain_conf::RetainConf;
///
/// Configuration parameters for [Services](src/services/services.rs)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesConf {
    pub retain: RetainConf,
}
//
//
impl TryFrom<serde_yaml::Value> for ServicesConf {
    type Error = Error;
    ///
    /// Returns ServicesConf parsed from serde_yaml::Value
    fn try_from(value: serde_yaml::Value) -> Result<Self, Self::Error> {
        serde_yaml::from_value(value).map_err(|err| Error::new("ServicesConf", "try_from").err(err.to_string()))
    }
}

use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use super::{PointRegistryApiConf, PointRegistryConf};

#[deprecated(note = "Use RegistryConf instead")]
pub type RetainConf = RegistryConf;

///
/// Retain configuration parameters and tools
/// - `path` - location of the retained values, something like assets/retain/
/// - `point` - store / load Point Id
/// 
/// **Example**
/// ```yaml
/// retain:
///     path: assets/retain/
///     point:
///         path: point/id.json
///         api:
///             table: public.tags
///             address: 0.0.0.0:8080
///             auth_token: 123!@#
///             database: crane_data_server
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegistryConf {
    pub path: Option<PathBuf>,
    pub point: Option<PointRegistryConf>,
}
//
//
impl RegistryConf {
    ///
    /// Creates new instance of `RegistryConf`
    pub fn new(path: Option<impl AsRef<Path>>, point: Option<PointRegistryConf>) -> Self {
        Self {
            path: path.map(|v| PathBuf::from(v.as_ref())),
            point
        }
    }
    ///
    /// Returns `point.path` if specified or 
    pub fn point_path(&self) -> Result<PathBuf, String> {
        match (self.path.clone(), self.point.clone()) {
            (Some(mut path), Some(point_path)) => {
                path.push(point_path.path);
                Ok(path)
            }
            (None, None) => Err(format!("RegistryConf.point_path | Retain path is not specified in the application config: {:#?}, \
                \n\t it have to be something like 'assets/testing/retain/', \
                \n\t or for testing purposes 'assets/testing/retain/'", 
            self)),
            (None, Some(_)) => Err(format!("RegistryConf.point_path | Retain path is not specified in the application config: {:#?}, \
                \n\t it have to be something like 'assets/testing/retain/', \
                \n\t or for testing purposes 'assets/testing/retain/'", 
            self)),
            (Some(_), None) => Err(format!("RegistryConf.point_path | Retain path is not specified in the application config: {:#?}, \
                \n\t it have to be something like 'assets/testing/retain/', \
                \n\t or for testing purposes 'assets/testing/retain/'", 
            self)),
        }
    }
    ///
    /// Returns `point.api` if specified or 
    pub fn point_api(&self) -> Result<PointRegistryApiConf, String> {
        match self.point.clone() {
            Some(point) => {
                match point.api {
                    Some(api) => Ok(api),
                    None => Err(format!("RegistryConf.point_api | Retain `point.api` is not specified in the application config: {:#?}", self)),
                }
            }
            None => Err(format!("RegistryConf.point_api | Retain `point` is not specified in the application config: {:#?}", self)),
        }
    }

}
//
//
impl Default for RegistryConf {
    fn default() -> Self {
        Self {
            path: Some(PathBuf::from("assets/retain/")),
            point: Some(PointRegistryConf::default())
        }
    }
}
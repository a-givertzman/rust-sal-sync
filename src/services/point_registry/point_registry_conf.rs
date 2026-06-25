use serde::{Deserialize, Serialize};
use super::PointRegistryApiConf;

#[deprecated(note = "Use PointRegistryConf instead")]
pub type RetainPointConf = PointRegistryConf;
///
/// Conf parameters to store/load Point's Id's on the disk
/// - `path` - where to store Point's Id's, something like `"assets/retain/point/id.json"`
/// - `api` - database parameters to store Point's Id's in the database 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointRegistryConf {
    pub path: String,
    pub api: Option<PointRegistryApiConf>,
}
//
//
impl PointRegistryConf {
    ///
    /// Creates conf parameters to store/load Point's Id's on the disk (and database if api specified)
    /// - `path` - where to store Point's Id's, something like `"point/id.json"`, with will be inside assets/retain/ - specified in the RetainConf
    /// - `api` - database parameters to store Point's Id's in the database 
    pub fn new(path: impl Into<String>, api: Option<PointRegistryApiConf>) -> Self {
        Self { 
            path: path.into(),
            api,
        }
    }
}
//
//
impl Default for PointRegistryConf {
    ///
    /// **Returns `RetainPointConf` with the default walues**
    /// 
    /// ```ignore
    /// RetainPointConf {
    ///     path: "id.json",    // file name withing standart path coming from retain_config: assets/retain/point/
    ///     api: None,
    /// }
    /// ```
    fn default() -> Self {
        Self {
            path: "id.json".to_owned(), //file name withing standart path coming from retain_config: assets/retain/point/
            api: None,
        }
    }
}

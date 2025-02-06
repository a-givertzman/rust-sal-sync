use crate::services::{
    entity::name::Name, retain::retain_conf::RetainConf,
    // conf::service_conf::ServiceConfig,
};
use super::conf_tree::ConfTree;
///
/// Configuration parameters for [Services](https://github.com/a-givertzman/rust-sal-sync/blob/master/src/services/services.rs)
#[derive(Debug, Clone, PartialEq)]
pub struct ServicesConf {
    pub name: Name,
    pub retain: RetainConf,
}
//
//
impl ServicesConf {
    ///
    /// 
    pub fn new(parent: impl Into<String>, conf: &ConfTree) -> Self {
        // trace!("ServicesConf.new | confTree: {:?}", conf_tree);
        // let self_id = format!("ServicesConf({})", conf_tree.key);
        // let mut self_conf = ServiceConfig::new(&self_id, conf_tree.to_owned());
        // trace!("{}.new | selfConf: {:?}", self_id, self_conf);
        // let self_name = Name::new(parent, self_conf.sufix());
        // debug!("{}.new | name: {:?}", self_id, self_name);
        // let description = self_conf.get_param_value("description").unwrap().as_str().unwrap().to_owned();
        // debug!("{}.new | description: {:?}", self_id, description);
        Self { name: Name::new("parent", "ServicesConf"), retain: RetainConf::default() }
    }
}
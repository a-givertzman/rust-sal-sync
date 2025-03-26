use crate::services::{
    entity::{dbg_id::DbgId, name::Name}, retain::retain_conf::RetainConf,
};
use super::conf_tree::{ConfTree, ConfTreeGet};
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
        // log::trace!("ServicesConf.new | confTree: {:?}", conf_tree);
        let parent = parent.into();
        let dbg = DbgId::with_parent(&parent, format!("ServicesConf({})", conf.key));
        let me = conf.sufix()
            .map(|s| if s.is_empty() {conf.name().unwrap()} else {s})
            .unwrap_or(conf.name().unwrap());
        let name = Name::new(parent, me);
        log::debug!("{}.new | name: {:?}", dbg, name);
        let retain = ConfTreeGet::<serde_yaml::Value>::get(conf, "retain");
        log::debug!("{}.new | retain: {:?}", dbg, retain);
        Self {
            name,
            retain: RetainConf::default(),
        }
    }
}
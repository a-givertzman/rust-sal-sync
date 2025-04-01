use sal_core::dbg::Dbg;
use crate::services::{
    entity::name::Name, retain::retain_conf::RetainConf,
};
use super::conf_tree::ConfTree;
///
/// Configuration parameters for [Services](https://github.com/a-givertzman/rust-sal-sync/blob/master/src/services/services.rs)
/// 
/// **Example**
/// ```yaml
/// services:
///     retain:
///         path: assets/retain/
///         point:
///             path: point/id.json
///             api:
///                 table: public.tags
///                 address: 0.0.0.0:8080
///                 auth_token: 123!@#
///                 database: crane_data_server
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
    pub fn new(parent: impl Into<String>, mut conf: ConfTree) -> Self {
        // log::trace!("ServicesConf.new | confTree: {:?}", conf_tree);
        let parent = parent.into();
        let dbg = Dbg::new(&parent, format!("ServicesConf({})", conf.key));
        let me = conf.sufix()
            .map(|s| if s.is_empty() {conf.name().unwrap()} else {s})
            .unwrap_or(conf.name().unwrap_or(format!("ServicesConf")));
        let name = Name::new(parent, me);
        log::debug!("{}.new | name: {:?}", dbg, name);
        let retain: RetainConf = match conf.parse("retain") {
            Ok(retain) => {
                log::debug!("{}.new | retain: {:?}", dbg, retain);
                retain
            },
            Err(err) => {
                log::warn!("{}.new | 'retain' parse error: {:?}", dbg, err);
                let retain = RetainConf::default();
                log::debug!("{}.new | Default retain: {:?}", dbg, retain);
                retain
            },
        };
        Self {
            name,
            retain,
        }
    }
}

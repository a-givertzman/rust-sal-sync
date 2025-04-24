use sal_core::dbg::Dbg;
use crate::services::{
    entity::Name, retain::RetainConf,
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
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = conf.sufix_or(conf.name().unwrap_or("ServicesConf".to_owned()));
        let dbg = Dbg::new(&parent, &me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
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

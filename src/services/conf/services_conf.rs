use crate::services::retain::retain_conf::RetainConf;
///
/// Configuration parameters for [Services](src/serv)
#[derive(Debug, PartialEq, Clone)]
pub struct ServicesConf {
    pub retain: RetainConf,
}
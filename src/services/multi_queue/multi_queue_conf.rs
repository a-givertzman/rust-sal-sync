use std::{fs, str::FromStr};
use crate::services::{conf::{conf_kind::ConfKind, conf_tree::ConfTree}, entity::name::Name, service::link_name::LinkName};
///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// service MultiQueue:
///     cycle: 1 ms
///     reconnect: 1 s  # default 3 s
///     address: 127.0.0.1:8080
///     in queue link:
///         max-length: 10000
///     send-to:                  # optional
///         - MultiQueue.queue
///                         ...
#[derive(Debug, PartialEq, Clone)]
pub struct MultiQueueConf {
    pub(crate) name: Name,
    pub(crate) rx: String,
    pub(crate) rx_max_length: i64,
    pub(crate) send_to: Vec<LinkName>,
}
//
// 
impl MultiQueueConf {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// service MultiQueue:
    ///     in queue in-queue:
    ///         max-length: 10000
    ///     send-to:                    # optional
    ///         - Service0.in-queue
    ///         - Service1.in-queue
    ///         ...
    ///         - ServiceN.in-queue
    ///                     ...
    pub fn new(parent: impl Into<String>, mut conf: ConfTree) -> MultiQueueConf {
        log::trace!("MultiQueueConfig.new | confTree: {:?}", conf);
        let dbg = format!("MultiQueueConf({})", conf.key);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let me = conf.sufix()
            .map(|s| if s.is_empty() {conf.name().unwrap()} else {s})
            .unwrap_or(conf.name().unwrap());
        let self_name = Name::new(parent, me);
        log::debug!("{}.new | self_name: {:?}", dbg, self_name);
        let (rx, rx_max_length) = conf.get_in_queue().unwrap();
        log::debug!("{}.new | 'in queue': {},\tmax-length: {}", dbg, rx, rx_max_length);
        let send_to = match conf.get_send_to_many() {
            Some(send_to) => send_to.into_iter().map(|send_to|LinkName::from_str(&send_to).unwrap()).collect(),
            None => {
                log::warn!("{}.new | 'send-to' - not found, empty or wrong values, Array<String> expected in config: {:#?}", dbg, conf);
                vec![]
            }
        };
        log::debug!("{}.new | 'send-to': {:?}", dbg, send_to);
        if let Ok((_, _)) = conf.get_by_keywd("out", ConfKind::Queue) {
            log::error!("{}.new | Parameter 'out queue' - deprecated, use 'send-to' instead in conf: {:#?}", dbg, conf)
        }
        MultiQueueConf {
            name: self_name,
            rx,
            rx_max_length,
            send_to,
        }
    }
    ///
    /// Creates config from serde_yaml::Value of following format:
    pub fn from_yaml(parent: impl Into<String>, value: &serde_yaml::Value) -> MultiQueueConf {
        match value.as_mapping().unwrap().into_iter().next() {
            Some((key, value)) => {
                Self::new(parent, ConfTree::new(key.as_str().unwrap(), value.clone()))
            }
            None => {
                panic!("MultiQueueConfig.from_yaml | Format error or empty conf: {:#?}", value)
            }
        }
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(parent: impl Into<String>, path: &str) -> MultiQueueConf {
        match fs::read_to_string(path) {
            Ok(yaml_string) => {
                match serde_yaml::from_str(&yaml_string) {
                    Ok(config) => {
                        MultiQueueConf::from_yaml(parent, &config)
                    }
                    Err(err) => {
                        panic!("MultiQueueConfig.read | Error in config: {:?}\n\terror: {:?}", yaml_string, err)
                    }
                }
            }
            Err(err) => {
                panic!("MultiQueueConfig.read | File {} reading error: {:?}", path, err)
            }
        }
    }
}

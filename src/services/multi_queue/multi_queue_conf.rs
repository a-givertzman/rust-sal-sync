use std::{fs, str::FromStr, time::Duration};
use crate::services::{conf::{ConfKind, ConfTree}, entity::Name, service::LinkName};
///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// service MultiQueue:
///     wait-started: 10 ms         # optional, next service will wait until current completely started plus specified time
///     wait-finished: 10 ms        # optional, parent service will wait until current completely finished plus specified time
///     in queue in-queue:
///         max-length: 10000
///     send-to:                    # optional
///         - Service0.in-queue
///         - Service1.in-queue
///         ...
///         - ServiceN.in-queue
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct MultiQueueConf {
    pub(crate) name: Name,
    pub(crate) wait_started: Option<Duration>,
    pub(crate) wait_finished: Option<Duration>,
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
    ///     wait-started: 10 ms         # optional, next service will wait until current completely started plus specified time
    ///     wait-finished: 10 ms        # optional, parent service will wait until current completely finished plus specified time
    ///     in queue in-queue:
    ///         max-length: 10000
    ///     send-to:                    # optional
    ///         - Service0.in-queue
    ///         - Service1.in-queue
    ///         ...
    ///         - ServiceN.in-queue
    /// ```
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> MultiQueueConf {
        let me = conf.sufix_or(conf.name().unwrap());
        let dbg = format!("MultiQueueConf({})", me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, &me);
        let dbg = format!("MultiQueueConf '{}'", name);
        log::debug!("{}.new | self_name: {:?}", dbg, name);
        let wait_started: Option<Duration> = conf.get_duration("wait-started").ok();
        log::debug!("{}.new | wait-started: {:?}", dbg, wait_started);
        let wait_finished: Option<Duration> = conf.get_duration("wait-finished").ok();
        log::debug!("{}.new | wait-finished: {:?}", dbg, wait_finished);
        let (rx, rx_max_length) = conf.get_in_queue().unwrap();
        log::debug!("{}.new | 'in queue': {},\tmax-length: {}", dbg, rx, rx_max_length);
        let send_to = match conf.get_send_to_many() {
            Ok(send_to) => send_to.into_iter().map(|send_to|LinkName::from_str(&send_to).unwrap()).collect(),
            Err(err) => {
                log::info!("{}.new | {:#?}", dbg, err);
                vec![]
            }
        };
        log::debug!("{}.new | 'send-to': {:?}", dbg, send_to);
        if let Ok((_, _)) = conf.get_by_keywd("out", ConfKind::Queue) {
            log::error!("{}.new | Parameter 'out queue' - deprecated, use 'send-to' instead", dbg)
        }
        MultiQueueConf {
            name,
            wait_started,
            wait_finished,
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
                panic!("MultiQueueConf.from_yaml | Format error or empty conf: {:#?}", value)
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
                        panic!("MultiQueueConf.read | Error in config: {:?}\n\terror: {:?}", yaml_string, err)
                    }
                }
            }
            Err(err) => {
                panic!("MultiQueueConf.read | File {} reading error: {:?}", path, err)
            }
        }
    }
}

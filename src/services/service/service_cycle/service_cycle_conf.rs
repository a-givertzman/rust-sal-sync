use std::{str::FromStr, time::Duration};

use debugging::session::debug_session::LogLevel;
use sal_core::{dbg::Dbg, error::Error};

use crate::services::conf::{ConfDuration, ConfTree, ConfTreeGet};

///
/// ### ServiceCicle config
/// 
/// - Default config (log-level by default is Error)
/// ```yaml
///     cycle: 10 ms
/// ```
/// 
/// - Config with log level
///     -`log-level`: 
///         - `None` - Silent mode. No logging for any exceeded intervals.
///         - `Error` - Logs critical exceedances ≥ 25 % of the cycle interval.
///         - `Warn` - Logs exceedances ≥ 10 % of the cycle interval.
///         - `Debug` - Logs all exceedances
/// ```yaml
///     cycle:
///         interval: 10 ms
///         log-level: None,    # None / Error / Warn / Debug
/// ```
pub struct ServiceCycleConf {
    interval: Duration,
    log_level: LogLevel,
}
//
impl ServiceCycleConf {
    ///
    /// Returns ServiceCycleConf from `ConfTree`
    pub fn new(parent: impl Into<String>, conf: &ConfTree) -> Self {
        let dbg = Dbg::new(parent, "ServiceCycleConf");
        let (interval, log_level) = match conf.is_mapping() {
            true => {
                let interval = conf.get_duration("interval").expect(&format!("{dbg}.new | 'interval' - not found or wrong config"));
                let log_level: String = conf.get("log-level").unwrap_or("Error".into());
                let log_level = match log_level.to_lowercase().as_str() {
                    "none" => LogLevel::Off,
                    "error" => LogLevel::Error,
                    "warn" => LogLevel::Warn,
                    "debug" => LogLevel::Debug,
                    _ => {
                        log::error!("{dbg}.new | Unknown log-level '{log_level}', expected None / Error / Warn / Debug");
                        panic!("{dbg}.new | Unknown log-level '{log_level}', expected None / Error / Warn / Debug");
                    }
                };
                (interval, log_level)
            }
            false => {
                let interval = conf.conf.as_str().expect(&format!("{dbg}.new | '{:?}' - wrong config", conf.conf));
                let interval = ConfDuration::from_str(interval)
                    .map_err(|err| Error::new(&dbg, "new").pass(err))
                    .expect(&format!("{dbg}.new | wrong config"))
                    .to_duration();
                (interval, LogLevel::Error)
            }
        };
        Self {
            interval,
            log_level,
        }
    }
}
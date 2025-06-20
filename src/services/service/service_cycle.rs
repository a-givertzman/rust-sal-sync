use std::{time::{Duration, Instant}, thread};

use sal_core::dbg::Dbg;
///
/// ServiceCycle - provides exact time interval in ms / us (future posible implementation)
///  - creates with Duration of interval
///  - method start() - begins countdown
///  - method wait() - awaiting remainder of the specified interval if not elapsed
/// 
/// [How to sleep for a few microseconds](https://stackoverflow.com/questions/4986818/how-to-sleep-for-a-few-microseconds)
pub struct ServiceCycle {
    dbg: Dbg,
    instant: Instant,
    interval: Duration,
    warn_exceed: Duration,
    err_exceed: Duration,
}
//
// 
impl ServiceCycle {
    ///
    /// Creates ServiceCycle with Duration of interval
    pub fn new(parent: impl Into<String>, interval: Duration) -> Self {
        Self {
            dbg: Dbg::new(parent.into(), "ServiceCycle"),
            instant: Instant::now(),
            interval,
            warn_exceed: interval / 10,
            err_exceed: interval / 4,
        }
    }
    ///
    /// Returns the specified cycle interval
    #[allow(unused)]
    pub fn interval(&self) -> Duration {
        self.interval
    }
    ///
    /// Starts new timer
    pub fn start(&mut self) {
        self.instant = Instant::now();
    }
    ///
    /// Waits for the remaining time,
    /// If the time elapsed since the start
    /// less then the specified cycle interval
    pub fn wait(&self) {
        let elapsed = self.instant.elapsed();
        if elapsed <= self.interval {
            let remainder = self.interval - elapsed;
            log::trace!("{}.wait | waiting: {:?}", self.dbg, remainder);
            thread::sleep(remainder);
        } else {
            let exceed = elapsed - self.interval;
            match exceed {
                e if e >= self.err_exceed => {
                    log::error!("{}.wait | exceeded {:?} by {:?}, elapsed {:?}", self.dbg, self.interval, elapsed - self.interval, elapsed);
                }
                e if e >= self.warn_exceed => {
                    log::warn!("{}.wait | exceeded {:?} by {:?}, elapsed {:?}", self.dbg, self.interval, elapsed - self.interval, elapsed);
                }
                _ => {
                    log::debug!("{}.wait | exceeded {:?} by {:?}, elapsed {:?}", self.dbg, self.interval, elapsed - self.interval, elapsed);
                }
            }
        }
    }
    ///
    /// Returns current elapsed time
    pub fn elapsed(&mut self) -> Duration {
        self.instant.elapsed()
    }
}
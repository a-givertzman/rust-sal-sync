use std::time::{Duration, Instant};

use sal_core::dbg::Dbg;

use crate::{services::Signal, sync::channel::{self, Receiver, Sender}};
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
    exit: Receiver<Signal>,
    s: Option<Sender<Signal>>,
}
//
// 
impl ServiceCycle {
    ///
    /// Creates ServiceCycle with Duration of interval
    pub fn new(parent: impl Into<String>, interval: Duration) -> Self {
        let (s, recv) = channel::unbounded();
        Self {
            dbg: Dbg::new(parent.into(), "ServiceCycle"),
            instant: Instant::now(),
            interval,
            warn_exceed: interval / 10,
            err_exceed: interval / 4,
            exit: recv,
            s: Some(s),
        }
    }
    ///
    /// Creates ServiceCycle with Duration of interval and depends on the system signal
    /// - imediatelly exits when system `Signal::Exit` received
    pub fn with_exit(parent: impl Into<String>, interval: Duration, exit: Receiver<Signal>) -> Self {
        Self {
            dbg: Dbg::new(parent.into(), "ServiceCycle"),
            instant: Instant::now(),
            interval,
            warn_exceed: interval / 10,
            err_exceed: interval / 4,
            exit,
            s: None,
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
    /// Waits for the remaining time, if the time elapsed since the start less then the specified cycle interval
    /// 
    /// If created from `ServiceExit`, then imediatelly returns if halt signal received
    pub fn wait(&self) {
        let elapsed = self.instant.elapsed();
        if elapsed <= self.interval {
            let remainder = self.interval.saturating_sub(elapsed);
            log::trace!("{}.wait | waiting: {:?}", self.dbg, remainder);
            _ = self.exit.recv_timeout(remainder);
        } else {
            let exceed = elapsed - self.interval;
            if exceed >= self.err_exceed {
                log::error!("{}.wait | exceeded {:?} by {:?}, elapsed {:?}", self.dbg, self.interval, elapsed - self.interval, elapsed);
            } else if exceed >= self.warn_exceed {
                log::warn!("{}.wait | exceeded {:?} by {:?}, elapsed {:?}", self.dbg, self.interval, elapsed - self.interval, elapsed);
            } else {
                log::debug!("{}.wait | exceeded {:?} by {:?}, elapsed {:?}", self.dbg, self.interval, elapsed - self.interval, elapsed);
            }
        }
    }
    ///
    /// Returns current elapsed time
    pub fn elapsed(&mut self) -> Duration {
        self.instant.elapsed()
    }
}
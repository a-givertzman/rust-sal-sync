use std::time::Duration;

use sal_core::dbg::Dbg;

use crate::{services::{ServiceCycle, Signal}, sync::channel::{self, Receiver}};
///
/// ServiceExit | Client of Graceful Shutdown System
/// 
/// Обеспечение централизованного механизма «мягкого завершения» для всех сервисов и потоков приложения.
/// 
///  - Provides Graceful Shutdown for all sevices of the application
///     - On close application [Signal::Exit] will sent and channel closed imediately
///  - Additionaly provides:
///     - sleep(Duration) - like thread::sleep, but returns Signal::Exit immediatelly if aplication halted
///     - interval() - [ServiceCycle] depends on system [Signal::Exit]
/// 
/// [How to sleep for a few microseconds](https://stackoverflow.com/questions/4986818/how-to-sleep-for-a-few-microseconds)
#[derive(Debug, Clone)]
pub struct ServiceExit {
    pub(super) recv: Receiver<Signal>,
    #[allow(unused)]
    pub(super) dbg: Dbg,
}
//
// 
impl ServiceExit {
    ///
    /// Returns the specified cycle interval
    #[allow(unused)]
    pub fn interval(&self, parent: impl Into<String>, interval: Duration) -> ServiceCycle {
        ServiceCycle::with_exit(
            parent,
            interval,
            self.recv.clone(),
        )
    }
    ///
    /// Waits for the duration, or [Signal::Exit]
    /// - Works like thread::sleep
    pub fn sleep(&self, dur: Duration) -> Signal {
        match self.recv.recv_timeout(dur) {
            Ok(Signal::Continue) | Err(channel::RecvTimeoutError::Timeout) => Signal::Continue,
            _ => Signal::Exit,
        }
    }
    ///
    /// Returns true if exit signal received
    pub fn is_exiting(&self) -> bool {
        self.recv.is_closed()
    }
}
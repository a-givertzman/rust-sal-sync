use std::{thread::{self}, time::{Duration, Instant}};
use sal_core::error::Error;
use crate::thread_pool::scheduler::Scheduler;
///
/// If maximum test turation will be exceeded - the panics throwed
pub struct LockTimer {
    id: String,
    type_: String,
    duration: Duration,
    scheduler: Option<Scheduler>,
    exit_s: kanal::Sender<()>,
    exit_r: kanal::Receiver<()>,
}
//
// 
impl LockTimer {
    ///
    /// If maximum lock duration exceeded - error message printed
    /// - parent: String - name of the parent entity
    /// - duration - maximum lock duration
    pub fn new(parent: impl Into<String>, type_: impl Into<String>, duration: Duration, scheduler: Option<Scheduler>) -> Self {
        let (send, recv) = kanal::bounded(0);
        Self {
            id: format!("{}/LockTimer", parent.into()),
            type_: type_.into(),
            duration,
            scheduler,
            exit_s: send,
            exit_r: recv,
        }
    }
    ///
    /// The countdown begins, exiting process with error code 70 / error message - if duration exceeded
    pub fn run(&self) -> Result<(), Error> {
        let dbg = self.id.clone();
        let type_ = self.type_.clone();
        let exit = self.exit_r.clone();
        let duration = self.duration;
        match &self.scheduler {
            Some(scheduler) => scheduler.spawn(move || {
                Self::wait(&dbg, &type_, duration, exit);
                Ok(())
            }),
            None => {
                thread::Builder::new().name(format!("{}.run", dbg)).spawn(move || {
                    Self::wait(&dbg, &type_, duration, exit);
                })
                .map_or_else(|err| Err(Error::new("me", "area").err(err.to_string())), |_| Ok(()))
            }
        }
    }
    ///
    /// The timer loop
    fn wait(dbg: &str, type_: &str, duration: Duration, exit: kanal::Receiver<()>) {
        let timer = Instant::now();
        match exit.recv_timeout(duration) {
            Ok(_) => {
            }
            Err(err) => match err {
                kanal::ReceiveErrorTimeout::Closed | kanal::ReceiveErrorTimeout::SendClosed => {
                    log::debug!("{}.run | Closed", dbg);
                }
                kanal::ReceiveErrorTimeout::Timeout => {
                    if timer.elapsed() > duration {
                        // std::process::exit(80);   // SOFTWARE: ExitCode = 80
                        log::error!("{}.run | Exceeded max lock wait ({:?}) for type: '{}'", dbg, duration, type_);
                    }
                }
            }
        }
    }
    ///
    /// Normal completion, must be called before the duration has expired
    pub fn exit(&self) {
        if let Err(err) = self.exit_s.send(()) {
            log::warn!("{}.exit | Send exit signal error: {:?}", self.id, err);
        }
    }
}
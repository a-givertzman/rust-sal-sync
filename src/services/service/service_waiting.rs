use std::time::Duration;
use sal_core::{dbg::Dbg, error::Error};
use crate::services::future::{Future, Sink};
///
/// 
pub struct ServiceWaiting<T> {
    dbg: Dbg,
    dur: Option<Duration>,
    sink: Sink<T>,
    future: Future<T>,
}
//
//
impl<T: Clone + Send + 'static> ServiceWaiting<T> {
    pub fn new(parent: impl Into<String>, dur: Option<Duration>,) -> Self {
        let (future, sink) = Future::new();
        Self {
            dbg: Dbg::new(parent, "ServiceWaiting"),
            dur,
            sink,
            future,
        }
    }
    ///
    /// Returns 
    pub fn release(&self) -> Sink<T> {
        self.sink.clone()
    }
    ///
    /// This method locks current thread until `future` received release event plus duration if specified
    pub fn wait(&self) -> T {
        let r = self.future.wait();
        self.dur.map(|dur| {
            std::thread::sleep(dur);
        });
        match r {
            Ok(r) => r,
            Err(err) => panic!("{:?}", Error::new(&self.dbg, "wait").pass(err)),
        }
    }
}
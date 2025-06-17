use log::error;
use sal_core::error::Error;
use crate::{services::types::TypeOf, sync::channel::{self, Receiver, Sender}, thread_pool::Scheduler};
///
/// Contains future callback
pub struct Future<T> {
    recv: Receiver<T>
    // exec: Box<dyn Fn() -> T>,
    // then: Box<dyn Fn(T) -> T>,
}
//
//
impl<T: Send + 'static> Future<T> {
    ///
    /// Returns Future new instance
    pub fn new() -> (Self, Sink<T>) {
        let (send, recv) = kanal::bounded(1);
        (
            Self { recv },
            Sink::new(send),
        )
    }
    /// 
    /// Returns value from future
    pub fn wait(&self) -> Result<T, Error> {
        match self.recv.recv() {
            Ok(event) => Ok(event),
            Err(err) => {
                log::warn!("Future.wait | Recv error: {:?}", err);
                Err(Error::new("Future", "wait").pass(err.to_string()))
            }
        }
    }
    /// 
    /// Returns future callback
    pub fn then(&self, on_done: impl Fn(T) -> T, on_err: impl Fn(String) -> T) -> T {
        match self.recv.recv() {
            Ok(event) => {
                (on_done)(event)
            }
            Err(err) => {
                (on_err)(format!("{}.then | Error: {:#?}", self.type_of(), err))
            }
        }
    }
    ///
    /// Spawning the closure using [Scheduler]
    pub fn spawn<F>(scheduler: Scheduler, f: F) -> Result<Future<T>, Error>
    where
        F: FnOnce() -> T + Send + 'static 
    {
        let (send, recv) = channel::bounded(1);
        let h = scheduler.spawn(move || {
            let result = f();
            if let Err(err) = send.send(result) {
                log::warn!("Future.spawn | Send error: {:?}", err);
            }
            Ok(())
        });
        match h {
            Ok(_) => Ok(Self { recv }),
            Err(err) => Err(Error::new("Future", "spawn").pass(err))
        }
    }
}
///
/// Contains `Sender<T>`
pub struct Sink<T> {
    send: Sender<T>,
}
//
//
impl<T> Sink<T> {
    ///
    /// Creates new instance of `Sink<T>`
    pub fn new(send: Sender<T>) -> Self {
        Self {
            send: send,
        }
    }
    ///
    /// Sends value to the corresponding `Future<T>`
    pub fn add(&self, value: T) {
        match self.send.send(value) {
            Ok(_) => {}
            Err(err) => error!("Sink.add | Send error: {:#?}", err),
        }
    }
}

use sal_core::error::Error;
use super::job::Job;
///
/// Provides schedule task to be executed on the [ThreadPool]
pub struct Scheduler {
    send: kanal::Sender<Job>,
    recv: kanal::Receiver<()>,
}
//
//
impl Scheduler {
    ///
    ///
    pub fn new(send: kanal::Sender<Job>, recv: kanal::Receiver<()>) -> Self {
        Self {
            send,
            recv,
        }
    }
    ///
    /// Spawns a new task to be scheduled on the [ThreadPool]
    pub fn spawn<F>(&self, f: F) -> Result<(), Error>
    where
        F: FnOnce() + Send + 'static {
        // Create a new Job::Task, wrapping a closure `f`
        let job = Box::new(f);
        match self.send.send(job) {
            Ok(_) => match self.recv.recv() {
                Ok(_) => Ok(()),
                Err(err) => Err(Error::new("Scheduler", "spawn").pass(err.to_string())),
            }
            Err(err) => Err(Error::new("Scheduler", "spawn").pass(err.to_string())),
        }
    }
}
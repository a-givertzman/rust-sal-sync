use sal_core::error::Error;
use super::job::Job;
///
/// Provides schedule task to be executed on the [ThreadPool]
pub struct Scheduler {
    send: kanal::Sender<Job>,
    // recv: kanal::Receiver<()>,
}
//
//
impl Scheduler {
    ///
    ///
    pub fn new(
        send: kanal::Sender<Job>,
        // recv: kanal::Receiver<()>,
    ) -> Self {
        Self {
            send,
            // recv,
        }
    }
    ///
    /// Spawns a new task to be scheduled on the [ThreadPool]
    pub fn spawn<F>(&self, f: F) -> Result<(), Error>
    where
        F: FnOnce() -> Result<(), Error> + Send + 'static {
        // Create a new Job::Task, wrapping a closure `f`
        match self.send.send(Job::Task(Box::new(f))) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new("Scheduler", "spawn").pass(err.to_string())),
        }
    }
}
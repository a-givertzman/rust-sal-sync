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
    F: FnOnce() + Send + 'static,
    // where
    //     F: FnOnce() -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        // We create a new Job::Task, wrapping our closure 'f'
        let job = Box::new(f);
        self.send.send(job);
        match self.recv.recv() {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new("Scheduler", "spawn").pass(err.to_string())),
        }
    }
}
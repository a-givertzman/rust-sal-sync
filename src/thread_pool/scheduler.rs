use sal_core::error::Error;
use super::{job::Job, JoinHandle};
///
/// Provides schedule task to be executed on the [ThreadPool]
#[derive(Clone)]
pub struct Scheduler {
    sender: kanal::Sender<Job>,
}
//
//
impl Scheduler {
    ///
    ///
    pub fn new(
        send: kanal::Sender<Job>,
    ) -> Self {
        Self {
            sender: send,
        }
    }
    ///
    /// Spawns a new task to be scheduled on the [ThreadPool]
    /// **Example**
    /// ```ignore
    /// let thread_pool = ThreadPool::new(&dbg, Some(1));
    /// let scheduler = thread_pool.scheduler();
    /// let result = scheduler.spawn(move || {
    ///     std::thread::sleep(Duration::from_millis(load));
    ///     result.fetch_add(1, Ordering::SeqCst);
    ///     Ok(())
    /// }).unwrap();
    /// assert!(result.join().unwrap() == ());
    /// thread_pool.join().unwrap();    
    /// ```
    pub fn spawn<F>(&self, f: F) -> Result<JoinHandle<()>, Error>
    where
        F: FnOnce() -> Result<(), Error> + Send + 'static {
        let (send, recv) = kanal::bounded(1);
        match self.sender.send(Job::Task((Box::new(f), send))) {
            Ok(_) => Ok(JoinHandle::new(recv)),
            Err(err) => Err(Error::new("Scheduler", "spawn").pass(err.to_string())),
        }
    }
}
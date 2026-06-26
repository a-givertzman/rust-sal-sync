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
    ///     result.fetch_add(1, Ordering::AcqRel);
    /// }).unwrap();
    /// assert!(result.join().unwrap() == ());
    /// thread_pool.join().unwrap();    
    /// ```
    pub fn spawn<F, T>(&self, f: F) -> Result<JoinHandle<T>, Error>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (send, recv) = kanal::bounded(1);
        let task = Box::new(move || {
            let result = f();
            let _ = send.send(result);
        });
        match self.sender.send(Job::Task(task)) {
            Ok(_) => Ok(JoinHandle::new("", "", recv)),
            Err(err) => Err(Error::new("Scheduler", "spawn").pass(err.to_string())),
        }
    }
    ///
    /// Spawns a named new task to be scheduled on the [ThreadPool]
    /// **Example**
    /// ```ignore
    /// let thread_pool = ThreadPool::new(&dbg, Some(1));
    /// let scheduler = thread_pool.scheduler();
    /// let result = scheduler.spawn("Task", move || {
    ///     std::thread::sleep(Duration::from_millis(load));
    ///     result.fetch_add(1, Ordering::AcqRel);
    /// }).unwrap();
    /// assert!(result.join().unwrap() == ());
    /// thread_pool.join().unwrap();    
    /// ```
    pub fn spawn_named<F, T>(&self, name: impl Into<String>, f: F) -> Result<JoinHandle<T>, Error>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (send, recv) = kanal::bounded(1);
        let task = Box::new(move || {
            let result = f();
            let _ = send.send(result);
        });
        match self.sender.send(Job::Task(task)) {
            Ok(_) => Ok(JoinHandle::new("", name, recv)),
            Err(err) => Err(Error::new("Scheduler", "spawn_named").pass(err.to_string())),
        }
    }
}
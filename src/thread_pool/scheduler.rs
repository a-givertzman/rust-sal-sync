use std::sync::Arc;
use sal_core::error::Error;
use crate::thread_pool::scaling::Scaling;
use super::{job::Job, JoinHandle};

///
/// ### Provides schedule task to be executed on the `ThreadPool`
#[derive(Clone)]
pub struct Scheduler {
    scaling: Arc<Scaling>,
    sender: kanal::Sender<Job>,
}
//
//
impl Scheduler {
    ///
    /// Returns `Scheduler` new instance
    pub fn new(scaling: Arc<Scaling>, send: kanal::Sender<Job>) -> Self {
        Self {
            scaling,
            sender: send,
        }
    }
    ///
    /// Maximum possible number of `Worker`'s
    pub fn capacity(&self) -> usize {
        self.scaling.capacity()
    }
    ///
    /// Current number of `Worker`'s
    pub fn size(&self) -> usize {
        self.scaling.size()
    }
    ///
    /// Current not a busy `Worker`'s
    pub fn free(&self) -> usize {
        self.scaling.free()
    }
    ///
    /// ### Spawns a new task to be scheduled on the `ThreadPool`
    /// 
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
        self.spawn_internal(None::<String>, f)
    }
    ///
    /// ### Spawns a named new task to be scheduled on the `ThreadPool`
    /// 
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
        self.spawn_internal(Some(name), f)
    }
    ///
    /// Spawns a new task to be scheduled on the `ThreadPool`
    #[inline]
    fn spawn_internal<F, T>(&self, name: Option<impl Into<String>>, f: F) -> Result<JoinHandle<T>, Error>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        if self.scaling.is_exiting() {
            return Err(Error::new("Scheduler", "spawn_named").pass("ThreadPool is shutting down"));
        }
        let (send, recv) = oneshot::channel();
        let task = Box::new(move || {
            let result = f();
            let _ = send.send(result);
        });
        match self.sender.send(Job::Task(task)) {
            Ok(_) => {
                self.scaling.extend();
                Ok(JoinHandle::new(None::<String>, name, recv))
            }
            Err(err) => Err(Error::new("Scheduler", "spawn_named").pass(err.to_string())),
        }
    }
}
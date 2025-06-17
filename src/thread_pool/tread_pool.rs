use std::sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex};
use coco::Stack;
use sal_core::{dbg::Dbg, error::Error};
use super::{job::Job, scheduler::Scheduler, worker::Worker, JoinHandle};
///
/// Provides ready to execute specified number of threads
/// - From start has only 1 or 2 prepared treads
/// - If all prepared threads are busy, new treds will be added to pool
/// - Number of threads limited by capacity, by default 64
pub struct ThreadPool {
    workers: Arc<Stack<Worker>>,
    sender: kanal::Sender<Job>,
    /// Maximum possible number of [Worker]'s
    capacity: Arc<AtomicUsize>,
    /// Current number of [Worker]'s
    size: Arc<AtomicUsize>,
    /// Not busy [Worker]'s
    free: Arc<AtomicUsize>,
}
//
//
impl ThreadPool {
    ///
    /// Returns [ThreadPool] new instance
    /// - `capacity` maximum number of threads, by default 64
    pub fn new(parent: impl Into<String>, capacity: Option<usize>) -> Self {
        let dbg = Dbg::new(parent, "ThreadPool");
        let default_capacity = 64;
        let capacity_ = match capacity {
            Some(capacity) => {
                if capacity == 0 {
                    log::warn!("{dbg} | Capacity of th ThreadPool cant be zero, used default {default_capacity}");
                    default_capacity
                } else {
                    capacity
                }
            }
            None => default_capacity,
        };
        let capacity = Arc::new(AtomicUsize::new(capacity_));
        let size = Arc::new(AtomicUsize::new(0));
        let free = Arc::new(AtomicUsize::new(0));
        let (sender, receiver) = kanal::unbounded();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = Arc::new(Stack::new());
        for _ in 0..if capacity.load(Ordering::SeqCst) > 1 { 2 } else { 1 } {
            workers.push(Worker::new(
                &dbg,
                receiver.clone(),
                capacity.clone(),
                size.clone(),
                free.clone(),
                workers.clone(),
            ));
        }
        ThreadPool {
            workers,
            sender,
            capacity,
            size,
            free,
        }
    }
    ///
    /// Maximum avalible number of [Worker]'s
    pub fn capacity(&self) -> usize {
        self.capacity.load(Ordering::SeqCst)
    }
    ///
    /// Current number of [Worker]'s
    pub fn size(&self) -> usize {
        self.size.load(Ordering::SeqCst)
    }
    ///
    /// Current not a busy [Worker]'s
    pub fn free(&self) -> usize {
        self.free.load(Ordering::SeqCst)
    }
    ///
    /// Returns [Scheduler] linked to the current [TreadPool]
    /// 
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
    pub fn scheduler(&self) -> Scheduler {
        Scheduler::new(self.sender.clone())
    }
    ///
    /// Spawns a new task to be scheduled on the [ThreadPool]
    pub fn spawn<F>(&self, f: F) -> Result<JoinHandle<()>, Error>
    where
        F: FnOnce() -> Result<(), Error> + Send + 'static
    {
        let (send, recv) = kanal::bounded(1);
        match self.sender.send(Job::Task((Box::new(f), send))) {
            Ok(_) => Ok(JoinHandle::new("", "", recv)),
            Err(err) => Err(Error::new("Scheduler", "spawn").pass(err.to_string())),
        }
    }
    ///
    /// Spawns a named new task to be scheduled on the [ThreadPool]
    pub fn spawn_named<F>(&self, name: impl Into<String> ,f: F) -> Result<JoinHandle<()>, Error>
    where
        F: FnOnce() -> Result<(), Error> + Send + 'static
    {
        let (send, recv) = kanal::bounded(1);
        match self.sender.send(Job::Task((Box::new(f), send))) {
            Ok(_) => Ok(JoinHandle::new("", name, recv)),
            Err(err) => Err(Error::new("Scheduler", "spawn").pass(err.to_string())),
        }
    }
    ///
    /// Returns all workers from self.workers
    fn send_exit_workers(&self) -> Vec<Worker> {
        let mut workers = vec![];
        while !self.workers.is_empty() {
            match self.workers.pop() {
                Some(worker) => {
                    if let Err(err) = self.sender.send(Job::Shutdown) {
                        log::warn!("ThreadPool.shutdown | Can't send 'Shutdown' signal to worker {}, error: {:?}", worker.id, err);
                    }
                    workers.push(worker);
                }
                None => break,
            }
        }
        workers
    }
    ///
    /// Sends `Shutdown` signal to all scheduled tasks and join them.
    /// This means all tasks will finish current jobs and then exit.
    pub fn join(&self) -> Result<(), Error> {
        self.shutdown()
    }
    ///
    /// Sends `Shutdown` signal to all scheduled tasks and join them.
    /// This means all tasks will finish current jobs and then exit.
    pub fn shutdown(&self) -> Result<(), Error> {
        let error = Error::new("ThreadPool", "shutdown");
        let mut errors = vec![];
        let mut remaining_workers = self.send_exit_workers();
        log::trace!("ThreadPool.shutdown | Shutdown signal sent to {} workers", remaining_workers.len());
        while !self.workers.is_empty() {
            match remaining_workers.pop() {
                Some(worker) => {
                    log::debug!("ThreadPool.shutdown | Wait for worker {} of {}...", worker.id, remaining_workers.len());
                    if let Err(err) = worker.join() {
                        let err = error.pass(format!("{:?}", err));
                        log::warn!("{}", err);
                        errors.push(err);
                    }
                }
                None => {}
            }
            let mut workers = self.send_exit_workers();
            log::trace!("ThreadPool.shutdown | Shutdown signal sent to {} workers", workers.len());
            remaining_workers.append(&mut workers);
        }
        if !errors.is_empty() {
            return Err(error.err(
                errors.iter().fold(String::new(), |acc, err| {
                    format!("{}\n{:?}", acc, err)
                })
            ));
        }
        Ok(())
    }
}
//
//
impl Drop for ThreadPool {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

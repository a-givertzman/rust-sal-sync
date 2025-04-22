use std::sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex};
use coco::Stack;
use sal_core::{dbg::Dbg, error::Error};
use super::{job::Job, scheduler::Scheduler, worker::Worker};
///
/// 
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
    pub fn new(parent: impl Into<String>, capacity: Option<usize>) -> Self {
        let dbg = Dbg::new(parent, "ThreadPool");
        let default_capacity = 64;
        let capacity = match capacity {
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
        let capacity = Arc::new(AtomicUsize::new(capacity));
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
    pub fn scheduler(&self) -> Scheduler {
        Scheduler::new(self.sender.clone())
    }
    ///
    /// Spawns a new task to be scheduled on the [ThreadPool]
    pub fn spawn<F>(&self, f: F) -> Result<(), Error>
    where
        F: FnOnce() -> Result<(), Error> + Send + 'static {
            match self.sender.send(Job::Task(Box::new(f))) {
                Ok(_) => Ok(()),
                Err(err) => Err(Error::new("Scheduler", "spawn").pass(err.to_string())),
            }
        }
    fn pop_workers(&self) -> Vec<Worker> {
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
    /// 
    pub fn shutdown(&self) -> Result<(), Error> {
        let error = Error::new("ThreadPool", "shutdown");
        let mut errors = vec![];
        let mut workers = self.pop_workers();
        log::debug!("ThreadPool.shutdown | Worker notified to 'Shutdown' {}", workers.len());
        while !workers.is_empty() {
            match workers.pop() {
                Some(worker) => {
                    log::debug!("ThreadPool.shutdown | Wait for worker {}", worker.id);
                    if let Err(err) = worker.join() {
                        let err = error.pass(format!("{:?}", err));
                        log::warn!("{}", err);
                        errors.push(err);
                    }
                }
                None => {}
            }
            workers.append(&mut self.pop_workers());
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
    ///
    /// 
    pub fn join(&self) -> Result<(), Error> {
        self.shutdown()
    }
}
//
//
impl Drop for ThreadPool {
    fn drop(&mut self) {
        if !self.workers.is_empty() {
            let _ = self.shutdown();
        }
    }
}

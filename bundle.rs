mod worker {
use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};
use crossbeam::queue::SegQueue;
use sal_core::{dbg::Dbg, error::Error};
use super::job::Job;
pub struct Worker {
    pub id: usize,
    thread: std::thread::JoinHandle<()>,
}
impl Worker {
    pub fn new(
        parent: impl Into<String>,
        receiver: kanal::Receiver<Job>,
        capacity: Arc<AtomicUsize>,
        size: Arc<AtomicUsize>,
        free: Arc<AtomicUsize>,
        workers: Arc<SegQueue<Worker>>,
    ) -> Worker {
        let parent = parent.into();
        let id = size.load(Ordering::Acquire);
        let dbg = Dbg::new(&parent, format!("Worker({id})"));
        size.fetch_add(1, Ordering::AcqRel);
        log::debug!("{dbg}.new | Created, capacity: {}, size: {}, free: {}", capacity.load(Ordering::Acquire), size.load(Ordering::Acquire), free.load(Ordering::Acquire));
        let thread = std::thread::spawn(move || loop {
            free.fetch_add(1, Ordering::Release);
            match receiver.recv() {
                Ok(Job::Task(job)) => {
                    if (free.load(Ordering::Acquire) < 2) && (size.load(Ordering::Acquire) < capacity.load(Ordering::Acquire)) {
                        Self::extend(&parent, &dbg, receiver.clone(), capacity.clone(), size.clone(), free.clone(), workers.clone());
                    }
                    log::debug!("{dbg}.new | Executing job...");
                    free.fetch_sub(1, Ordering::AcqRel);
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        job();
                    }));
                    let busy = size.load(Ordering::Acquire) - free.load(Ordering::Acquire) - 1;
                    log::debug!("{dbg}.new | Done job {id}, busy {busy}");
                }
                Ok(Job::Shutdown) => {
                    let busy = size.load(Ordering::Acquire) - free.load(Ordering::Acquire);
                    log::info!("{dbg}.new | Exit, busy {busy}");
                    break;
                }
                Err(err) => {
                    log::error!("{dbg}.new | Recv error, channel closed, details: \n\t{:?}", err);
                    break;
                }
            };
        });
        Worker { id, thread }
    }
    fn extend(
        parent: impl Into<String>,
        dbg: &Dbg,
        receiver: kanal::Receiver<Job>,
        capacity: Arc<AtomicUsize>,
        size: Arc<AtomicUsize>,
        free: Arc<AtomicUsize>,
        workers: Arc<SegQueue<Worker>>
    ) {
        let parent = parent.into();
        let new_workers = size.load(Ordering::SeqCst) * 2;
        log::debug!("{dbg}.extend | Trying to creating {new_workers} new workers...");
        for _ in 0..new_workers {
            if size.load(Ordering::SeqCst) < capacity.load(Ordering::SeqCst) {
                workers.push(Worker::new(
                    &parent,
                    receiver.clone(),
                    capacity.clone(),
                    size.clone(),
                    free.clone(),
                    workers.clone(),
                ));
            }
        }
    }
    pub fn join(self) -> Result<(), Error> {
        self.thread
            .join()
            .map_err(|err| Error::new(format!("Worker({})", self.id), "join").pass(format!("{:?}", err)))
    }
}}
pub(super) mod job {
pub enum Job {
    Task(Box<dyn FnOnce() + Send + 'static>),
    Shutdown,
}
}
mod join_handle {
use sal_core::error::Error;
///
/// Provides to join on a thread (block on its termination).
/// Returns `id` and `name` of associated thread
pub struct JoinHandle<T> {
    id: String,
    name: String,
    recv: kanal::Receiver<T>,
}
//
//
impl<T> JoinHandle<T> {
    ///
    /// Returns [JoinHandle] new instance
    pub fn new(id: impl Into<String>, name: impl Into<String>, recv: kanal::Receiver<T>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            recv,
        }
    }
    ///
    /// Gets the thread's unique identifier.
    pub fn id(&self) -> String {
        self.id.clone()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn join(self) -> Result<T, Error> {
        match self.recv.recv() {
            Ok(v) => Ok(v),
            Err(err) => Err(Error::new("JoinHandle", "join").err(err.to_string())),
        }
    }
}}
mod scheduler {
use sal_core::error::Error;
use super::{job::Job, JoinHandle};
#[derive(Clone)]
pub struct Scheduler {
    sender: kanal::Sender<Job>,
}
impl Scheduler {
    pub fn new(
        send: kanal::Sender<Job>,
    ) -> Self {
        Self {
            sender: send,
        }
    }
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
}}
mod tread_pool {
use std::sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex};
use crossbeam::queue::SegQueue;
use sal_core::{dbg::Dbg, error::Error};
use super::{job::Job, scheduler::Scheduler, worker::Worker, JoinHandle};
pub struct ThreadPool {
    workers: Arc<SegQueue<Worker>>,
    sender: kanal::Sender<Job>,
    capacity: Arc<AtomicUsize>,
    size: Arc<AtomicUsize>,
    free: Arc<AtomicUsize>,
}
impl ThreadPool {
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
        let workers = Arc::new(SegQueue::new());
        for _ in 0..if capacity.load(Ordering::Acquire) > 1 { 2 } else { 1 } {
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
    pub fn capacity(&self) -> usize {
        self.capacity.load(Ordering::Acquire)
    }
    pub fn size(&self) -> usize {
        self.size.load(Ordering::Acquire)
    }
    pub fn free(&self) -> usize {
        self.free.load(Ordering::Acquire)
    }
    pub fn scheduler(&self) -> Scheduler {
        Scheduler::new(self.sender.clone())
    }
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
    pub fn spawn_named<F, T>(&self, name: impl Into<String> ,f: F) -> Result<JoinHandle<T>, Error>
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
            Err(err) => Err(Error::new("Scheduler", "spawn").pass(err.to_string())),
        }
    }
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
    pub fn join(&self) -> Result<(), Error> {
        self.shutdown()
    }
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
impl Drop for ThreadPool {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}
}
pub use join_handle::*;
pub use scheduler::*;
pub use tread_pool::*;

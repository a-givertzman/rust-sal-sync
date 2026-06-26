mod worker {
use std::sync::Arc;
use sal_core::{dbg::Dbg, error::Error};
use crate::thread_pool::scaling::Scaling;
use super::job::Job;
pub struct Worker {
    pub id: usize,
    handle: std::thread::JoinHandle<()>,
}
impl Worker {
    pub fn new(
        parent: impl Into<String>,
        id: usize,
        receiver: kanal::Receiver<Job>,
        scaling: Arc<Scaling>,
    ) -> Worker {
        let parent = parent.into();
        let dbg = Dbg::new(&parent, format!("Worker({id})"));
        let handle = std::thread::spawn(move || loop {
            match receiver.recv() {
                Ok(Job::Task(job)) => {
                    log::debug!("{dbg}.new | Take Job...");
                    scaling.register_busy();
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        job();
                    }));
                    log::debug!("{dbg}.new | Job done");
                    scaling.register_idle();
                }
                Ok(Job::Shutdown) => {
                    log::info!("{dbg}.new | Exit");
                    break;
                }
                Err(err) => {
                    log::error!("{dbg}.new | Recv error, channel closed, details: \n\t{:?}", err);
                    break;
                }
            };
        });
        Worker { id, handle }
    }
    pub fn is_closed(&self) -> bool {
        self.handle.is_finished()
    }
    pub fn join(self) -> Result<(), Error> {
        self.handle
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
mod scaling {
use std::sync::{Arc, Weak, atomic::{AtomicBool, AtomicUsize, Ordering}};
use crossbeam::queue::SegQueue;
use sal_core::dbg::Dbg;
use crate::thread_pool::{job::Job, worker::Worker};
///
/// Manages lifecycle, capacity, and active metrics of ThreadPool workers.
pub struct Scaling {
    receiver: kanal::Receiver<Job>,
    workers: Arc<SegQueue<Worker>>,
    /// Maximum possible number of [Worker]'s
    capacity: Arc<AtomicUsize>,
    active: Arc<AtomicUsize>,
    is_extending: AtomicBool,
    me: Weak<Self>,
    exit: Arc<AtomicBool>,
    parent: String,
    dbg: Dbg,
}
impl Scaling {
    pub fn new(parent: impl Into<String>, capacity: Option<usize>, receiver: kanal::Receiver<Job>, workers: Arc<SegQueue<Worker>>, exit: Arc<AtomicBool>) -> Arc<Self> {
        let parent = parent.into();
        let dbg = Dbg::new(&parent, "Scaling");
        let default_capacity = 64;
        let capacity = match capacity {
            Some(capacity) => {
                if capacity == 0 {
                    log::warn!("{dbg} | Capacity of the ThreadPool cant be zero, used default {default_capacity}");
                    default_capacity
                } else {
                    capacity
                }
            }
            None => default_capacity,
        };
        Arc::new_cyclic(|me| {
            Self {
                receiver,
                workers,
                capacity: Arc::new(AtomicUsize::new(capacity)),
                active: Arc::new(AtomicUsize::new(0)),
                is_extending: AtomicBool::new(false),
                me: me.clone(),
                exit,
                parent,
                dbg
            }
        })
    }
    pub fn capacity(&self) -> usize {
        self.capacity.load(Ordering::Relaxed)
    }
    pub fn size(&self) -> usize {
        self.workers.len()
    }
    pub fn free(&self) -> usize {
        self.size().saturating_sub(self.busy())
    }
    pub fn busy(&self) -> usize {
        self.active.load(Ordering::Relaxed)
    }
    fn new_workers(&self) -> Option<usize> {
        let current_size = self.size().max(1);
        let max_capacity = self.capacity();
        if current_size >= max_capacity {
            return None;
        }
        if self.free() < 1 {
            let target_size = (current_size * 2).clamp(1, max_capacity);
            let new_size = target_size.saturating_sub(current_size);
            return (new_size > 0).then_some(new_size);
        }
        None
    }
    pub fn extend(&self) {
        if !self.exit.load(Ordering::Acquire) {
            if self.is_extending.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
                if let Some(scaling) = self.me.upgrade() {
                    if let Some(new_workers) = self.new_workers() {
                        log::debug!("{}.extend | Trying to create {new_workers} new workers...", self.dbg);
                        let max_capacity = self.capacity();
                        for _ in 0..new_workers {
                            let size = self.size();
                            if size < max_capacity {
                                self.workers.push(Worker::new(
                                    &self.parent,
                                    size + 1,
                                    self.receiver.clone(),
                                    scaling.clone(),
                                ));
                            }
                        }
                        log::debug!("{}.new | New workers created, size: {} capacity: {}", self.dbg, self.size(), self.capacity());
                    }
                }
                self.is_extending.store(false, Ordering::Release);
            }
        }
    }
    pub fn register_busy(&self) {
        self.active.fetch_add(1, Ordering::AcqRel);
        if log::max_level() >= log::LevelFilter::Debug {
            log::debug!("{}.new | Worker busy {} of {}, capacity: {}", self.dbg, self.busy(), self.size(), self.capacity());
        }
    }
    pub fn register_idle(&self) {
        let _ = self.active.fetch_update(Ordering::AcqRel, Ordering::Acquire, |x| {
            if x > 0 { Some(x - 1) } else { None }
        });
        if log::max_level() >= log::LevelFilter::Debug {
            log::debug!("{}.new | Worker busy {} of {}, capacity: {}", self.dbg, self.busy(), self.size(), self.capacity());
        }
    }
    pub fn unregister_worker(&self) {
        let mut tmp = Vec::with_capacity(self.workers.len());
        while !self.workers.is_empty() {
            if let Some(w) = self.workers.pop() {
                if !w.is_closed() {
                    tmp.push(w);
                }
            }
        }
        for w in tmp {
            self.workers.push(w);
        }
        if log::max_level() >= log::LevelFilter::Debug {
            log::debug!("{}.new | Worker closed, size: {}, capacity: {}", self.dbg, self.size(), self.capacity());
        }
    }
}}
mod join_handle {
use sal_core::error::Error;
pub struct JoinHandle<T> {
    id: String,
    name: String,
    recv: kanal::Receiver<T>,
}
impl<T> JoinHandle<T> {
    pub fn new(id: impl Into<String>, name: impl Into<String>, recv: kanal::Receiver<T>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            recv,
        }
    }
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
use std::sync::Arc;
use sal_core::error::Error;
use crate::thread_pool::scaling::Scaling;
use super::{job::Job, JoinHandle};
#[derive(Clone)]
pub struct Scheduler {
    scaling: Arc<Scaling>,
    sender: kanal::Sender<Job>,
}
impl Scheduler {
    pub fn new(scaling: Arc<Scaling>, send: kanal::Sender<Job>) -> Self {
        Self {
            scaling,
            sender: send,
        }
    }
    pub fn capacity(&self) -> usize {
        self.scaling.capacity()
    }
    pub fn size(&self) -> usize {
        self.scaling.size()
    }
    pub fn free(&self) -> usize {
        self.scaling.free()
    }
    pub fn spawn<F, T>(&self, f: F) -> Result<JoinHandle<T>, Error>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.scaling.extend();
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
        self.scaling.extend();
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
mod thread_pool {
use std::sync::{Arc, atomic::{AtomicBool, AtomicUsize, Ordering}};
use crossbeam::queue::SegQueue;
use sal_core::{dbg::Dbg, error::Error};
use super::{job::Job, scaling::Scaling, scheduler::Scheduler, worker::Worker, JoinHandle};
pub struct ThreadPool {
    workers: Arc<SegQueue<Worker>>,
    sender: kanal::Sender<Job>,
    scaling: Arc<Scaling>,
    exit: Arc<AtomicBool>,
}
impl ThreadPool {
    pub fn new(parent: impl Into<String>, capacity: Option<usize>) -> Self {
        let dbg = Dbg::new(parent, "ThreadPool");
        let exit = Arc::new(AtomicBool::new(false));
        let (sender, receiver) = kanal::unbounded();
        let workers = Arc::new(SegQueue::new());
        let scaling = Scaling::new(&dbg, capacity, receiver.clone(), workers.clone(), exit.clone());
        for _ in 0..(scaling.capacity() / 4) {
            let id = scaling.size() + 1;
            workers.push(Worker::new(
                &dbg,
                id,
                receiver.clone(),
                scaling.clone(),
            ));
        }
        ThreadPool {
            workers,
            sender,
            scaling,
            exit,
        }
    }
    pub fn capacity(&self) -> usize {
        self.scaling.capacity()
    }
    pub fn size(&self) -> usize {
        self.scaling.size()
    }
    pub fn free(&self) -> usize {
        self.scaling.free()
    }
    pub fn scheduler(&self) -> Scheduler {
        Scheduler::new(self.scaling.clone(), self.sender.clone())
    }
    pub fn spawn<F, T>(&self, f: F) -> Result<JoinHandle<T>, Error>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.scaling.extend();
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
        self.scaling.extend();
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
        if self.workers.is_empty() {
            return vec![];
        }
        let mut workers = Vec::with_capacity(self.workers.len());
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
        self.exit.store(true, Ordering::Release);
        let error = Error::new("ThreadPool", "shutdown");
        let mut errors = vec![];
        let mut remaining_workers = self.send_exit_workers();
        log::trace!("ThreadPool.shutdown | Shutdown signal sent to {} workers", remaining_workers.len());
        while !remaining_workers.is_empty() {
            if let Some(worker) = remaining_workers.pop() {
                log::debug!("ThreadPool.shutdown | Wait for worker {} of {}...", worker.id, remaining_workers.len() + 1);
                if let Err(err) = worker.join() {
                    let err = error.pass(format!("{:?}", err));
                    log::warn!("{}", err);
                    errors.push(err);
                }
            }
            if !self.workers.is_empty() {
                remaining_workers.extend(self.send_exit_workers());
            }
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
pub use thread_pool::*;

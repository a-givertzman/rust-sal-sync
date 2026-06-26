use std::sync::{Arc, Weak, atomic::{AtomicBool, AtomicUsize, Ordering}};
use crossbeam::queue::SegQueue;
use sal_core::dbg::Dbg;
use crate::thread_pool::{job::Job, worker::Worker};

///
/// Для автоматического восстановления флага AtomicBool в конце области видимости
struct ExtendGuard<'a>(&'a AtomicBool);
impl<'a> Drop for ExtendGuard<'a> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}
///
/// Manages lifecycle, capacity, and active metrics of ThreadPool workers.
pub struct Scaling {
    receiver: kanal::Receiver<Job>,
    workers: Arc<SegQueue<Worker>>,
    /// Maximum possible number of [Worker]'s
    capacity: Arc<AtomicUsize>,
    /// Number of workers currently executing a task
    active: Arc<AtomicUsize>,
    /// Extending at the moment
    is_extending: AtomicBool,
    me: Weak<Self>,
    exit: Arc<AtomicBool>,
    parent: String,
    dbg: Dbg,
}
//
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
    ///
    /// Returns maximum possible number of workers
    pub fn capacity(&self) -> usize {
        self.capacity.load(Ordering::Relaxed)
    }
    ///
    /// Returns current number of workers
    pub fn size(&self) -> usize {
        self.workers.len()
    }
    ///
    /// Returns maximum possible number of workers
    pub fn free(&self) -> usize {
        self.size().saturating_sub(self.busy() + self.receiver.len())
    }
    ///
    /// Returns nimber of workers busy for the moment
    pub fn busy(&self) -> usize {
        self.active.load(Ordering::Relaxed)
    }
    ///
    /// Checks if new workers are needed based on current workload and capacity
    fn new_workers(&self) -> Option<usize> {
        let current_size = self.size().max(1);
        let max_capacity = self.capacity();
        if current_size >= max_capacity {
            return None;
        }
        // Scale ONLY condition
        if self.free() < 3 {
            let target_size = (current_size * 2).clamp(1, max_capacity);
            let new_size = target_size.saturating_sub(current_size);
            return (new_size > 0).then_some(new_size);
        }
        None
    }
    ///
    /// Extending current number of [Worker]'s if required
    pub fn extend(&self) {
        if !self.exit.load(Ordering::Acquire) {
            if self.size() >= self.capacity() {
                return;
            }
            if self.is_extending.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
                let _guard = ExtendGuard(&self.is_extending);
                if let Some(scaling) = self.me.upgrade() {
                    if let Some(new_workers) = self.new_workers() {
                        log::debug!("{}.extend | Creating {new_workers} new workers...", self.dbg);
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
            }
        }
    }
    ///
    /// Registers worker is busy
    pub fn register_busy(&self) {
        self.active.fetch_add(1, Ordering::Relaxed);
        if log::max_level() >= log::LevelFilter::Debug {
            log::debug!("{}.new | Worker busy {} of {}, capacity: {}", self.dbg, self.busy(), self.size(), self.capacity());
        }
    }
    ///
    /// Registers worker is ready for new job
    pub fn register_idle(&self) {
        self.active.fetch_sub(1, Ordering::Relaxed);
        if log::max_level() >= log::LevelFilter::Debug {
            log::debug!("{}.new | Worker busy {} of {}, capacity: {}", self.dbg, self.busy(), self.size(), self.capacity());
        }
    }
    ///
    /// Returns `true` if `Scaling` is exiting
    pub fn is_exiting(&self) -> bool {
        self.exit.load(Ordering::Acquire)
    }
}
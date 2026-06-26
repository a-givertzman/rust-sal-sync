use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};
use crossbeam::queue::SegQueue;
use sal_core::{dbg::Dbg, error::Error};
use super::job::Job;
///
/// Picks up code to be executed in the [Worker]’s thread on the `ThreadPool`
pub struct Worker {
    pub id: usize,
    thread: std::thread::JoinHandle<()>,
}
//
//
impl Worker {
    ///
    /// Returns [Worker] new instance
    /// - `receiver` - channel of incomming jobs
    /// - `capacity` - maximum avalible number of [Worker]'s in the `ThreadPool`
    /// - `size` - current number of [Worker]'s in the `ThreadPool`
    /// - `free` - not busy [Worker]'s in the `ThreadPool`
    /// - `workers` - collection of [Worker]'s in the `ThreadPool`
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
    ///
    /// Extending current number of [Worker]'s if required
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
    ///
    /// Waits for the associated thread to finish.
    /// 
    /// This function will return immediately if the associated thread has already finished.
    /// 
    /// In terms of [atomic memory orderings], the completion of the associated thread synchronizes with this function returning. In other words, all operations performed by that thread happen before all operations that happen after join returns.
    /// 
    /// If the associated thread panics, [Err] is returned with the parameter given to panic (though see the Notes below).
    /// 
    /// ## Panics
    /// This function may panic on some platforms if a thread attempts to join itself or otherwise may create a deadlock with joining threads.
    pub fn join(self) -> Result<(), Error> {
        self.thread
            .join()
            .map_err(|err| Error::new(format!("Worker({})", self.id), "join").pass(format!("{:?}", err)))
    }
}
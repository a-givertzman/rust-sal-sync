use std::sync::Arc;
use sal_core::{dbg::Dbg, error::Error};
use crate::thread_pool::scaling::Scaling;

use super::job::Job;
///
/// Picks up code to be executed in the [Worker]’s thread on the `ThreadPool`
pub struct Worker {
    id: usize,
    handle: std::thread::JoinHandle<()>,
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
                Err(_) => {
                    log::info!("{dbg}.new | Job queue closed. Shutting down");
                    break;
                }
            };
        });
        Worker { id, handle}
    }
    ///
    /// Returns identifier of the Worker
    pub fn id(&self) -> usize {
        self.id
    }
    ///
    /// Returns true if `Worker` is exited
    pub fn is_closed(&self) -> bool {
        self.handle.is_finished()
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
        self.handle
            .join()
            .map_err(|err| Error::new(format!("Worker({})", self.id), "join").pass(format!("{:?}", err)))
    }
}
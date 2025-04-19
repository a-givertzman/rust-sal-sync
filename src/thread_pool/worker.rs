use std::sync::{Arc, Mutex};
use super::job::Job;
///
/// Picks up code to be executed in the Worker’s thread on the [ThreadPool]
pub struct Worker {
    id: usize,
    thread: std::thread::JoinHandle<()>,
}
//
//
impl Worker {
    ///
    /// Returns [Worker] new instance
    pub fn new(id: usize, receiver: Arc<Mutex<kanal::Receiver<Job>>>) -> Worker {
        let thread = std::thread::spawn(move || loop {
            // let error = Error::new("Worker", "new");
            match receiver.lock() {
                Ok(receiver) => match receiver.recv() {
                    Ok(job) => {
                        // let job = receiver.lock().unwrap().recv().unwrap();
                        log::debug!("Worker({id}).new | Got a job; executing...");
                        job();
                    }
                    Err(err) => {
                        log::trace!("Worker({id}).new | Recv error, channel closed, details: \n\t{:?}", err);
                    }
                }
                Err(err) => {
                    log::error!("Worker({id}).new | Lock error: {:?}", err);
                }
            }
        });
        Worker { id, thread }
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
    /// Panics
    /// This function may panic on some platforms if a thread attempts to join itself or otherwise may create a deadlock with joining threads.
    pub fn join(self) -> Result<(), Box<dyn std::any::Any + Send + 'static>> {
        self.thread.join()
    }
}
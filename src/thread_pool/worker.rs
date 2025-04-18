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
}
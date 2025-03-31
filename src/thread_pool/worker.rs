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
            let job = receiver.lock().unwrap().recv().unwrap();
            log::debug!("Worker({id}).new | Got a job; executing...");
            job();
        });
        Worker { id, thread }
    }
}
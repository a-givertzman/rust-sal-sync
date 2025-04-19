use std::sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex};
use coco::Stack;

use super::job::Job;
///
/// Picks up code to be executed in the [Worker]’s thread on the `ThreadPool`
pub struct Worker {
    id: usize,
    /// Current total number of [Worker]'s in the `ThreadPool`
    // size: Arc<AtomicUsize>,
    /// Not busy [Worker]'s in the `ThreadPool`
    // free: Arc<AtomicUsize>,
    // workers: Arc<Stack<Worker>>,
    thread: std::thread::JoinHandle<()>,
}
//
//
impl Worker {
    ///
    /// Returns [Worker] new instance
    pub fn new(id: usize, receiver: Arc<Mutex<kanal::Receiver<Job>>>, capacity: Arc<AtomicUsize>, size: Arc<AtomicUsize>, free: Arc<AtomicUsize>, workers: Arc<Stack<Worker>>) -> Worker {
        log::debug!("Worker({id}).new | New one created, catacity: {}, size: {}, free: {}", capacity.load(Ordering::SeqCst), size.load(Ordering::SeqCst), free.load(Ordering::SeqCst));
        let thread = std::thread::spawn(move || loop {
            // let error = Error::new("Worker", "new");
            if free.load(Ordering::SeqCst) < 2 {
                let new_workers = size.load(Ordering::SeqCst) * 2;
                log::debug!("Worker({id}).new | Creating {new_workers} new workers...");
                for _ in 0..new_workers {
                    if size.load(Ordering::SeqCst) < capacity.load(Ordering::SeqCst) {
                        let id = size.load(Ordering::SeqCst);
                        workers.push(Worker::new(id, Arc::clone(&receiver), capacity.clone(), size.clone(), free.clone(), workers.clone()));
                        size.fetch_add(1, Ordering::SeqCst);
                        free.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }
            let receiver_lock = receiver.lock();
            let job = match receiver_lock {
                Ok(receiver) => {
                    let job = receiver.recv();
                    match job {
                        Ok(job) => {
                            // let job = receiver.lock().unwrap().recv().unwrap();
                            Some(job)
                        }
                        Err(err) => {
                            log::error!("Worker({id}).new | Recv error, channel closed, details: \n\t{:?}", err);
                            break;
                        }
                    }
                }
                Err(err) => {
                    log::error!("Worker({id}).new | Lock error: {:?}", err);
                    None
                }
            };
            match job {
                Some(job) => {
                    log::debug!("Worker({id}).new | Executing job...");
                    free.fetch_sub(1, Ordering::SeqCst);
                    job();
                    free.fetch_add(1, Ordering::SeqCst);
                    log::debug!("Worker({id}).new | Done job...");
                }
                None => {}
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
use std::sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Mutex};
use coco::Stack;

use super::job::Job;
///
/// Picks up code to be executed in the [Worker]’s thread on the `ThreadPool`
pub struct Worker {
    pub id: usize,
    /// Current total number of [Worker]'s in the `ThreadPool`
    // size: Arc<AtomicUsize>,
    /// Not busy [Worker]'s in the `ThreadPool`
    // free: Arc<AtomicUsize>,
    // workers: Arc<Stack<Worker>>,
    thread: std::thread::JoinHandle<()>,
    exit: Arc<AtomicBool>,
}
//
//
impl Worker {
    ///
    /// Returns [Worker] new instance
    pub fn new(receiver: Arc<Mutex<kanal::Receiver<Job>>>, capacity: Arc<AtomicUsize>, size: Arc<AtomicUsize>, free: Arc<AtomicUsize>, workers: Arc<Stack<Worker>>) -> Worker {
        let id = size.load(Ordering::SeqCst);
        size.fetch_add(1, Ordering::SeqCst);
        let exit = Arc::new(AtomicBool::new(false));
        let exit_clone = exit.clone();
        log::debug!("Worker({id}).new | New one created, catacity: {}, size: {}, free: {}", capacity.load(Ordering::SeqCst), size.load(Ordering::SeqCst), 1 + free.load(Ordering::SeqCst));
        let thread = std::thread::spawn(move || loop {
            // let error = Error::new("Worker", "new");
            free.fetch_add(1, Ordering::SeqCst);
            if free.load(Ordering::SeqCst) < 2 {
                let new_workers = size.load(Ordering::SeqCst) * 2;
                log::debug!("Worker({id}).new | Creating {new_workers} new workers...");
                for _ in 0..new_workers {
                    if size.load(Ordering::SeqCst) < capacity.load(Ordering::SeqCst) {
                        workers.push(Worker::new(Arc::clone(&receiver), capacity.clone(), size.clone(), free.clone(), workers.clone()));
                    }
                }
            }
            let receiver_lock = receiver.lock();
            let job = match receiver_lock {
                Ok(receiver) => {
                    let job = receiver.recv();
                    match job {
                        Ok(job) => match job {
                            Job::Task(job) => {
                                // let job = receiver.lock().unwrap().recv().unwrap();
                                Some(job)
                            }
                            Job::Shutdown => break,
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
                    log::debug!("Worker({id}).new | Done job...");
                }
                None => {}
            }
            if exit.load(Ordering::SeqCst) {
                break;
            }
        });
        Worker { id, thread, exit: exit_clone }
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
        self.exit.store(true, Ordering::SeqCst);
        self.thread.join()
    }
}
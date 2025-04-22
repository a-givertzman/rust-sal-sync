use std::sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex};
use coco::Stack;
use sal_core::dbg::Dbg;

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
        receiver: Arc<Mutex<kanal::Receiver<Job>>>,
        capacity: Arc<AtomicUsize>,
        size: Arc<AtomicUsize>,
        free: Arc<AtomicUsize>,
        workers: Arc<Stack<Worker>>,
    ) -> Worker {
        let parent = parent.into();
        let id = size.load(Ordering::SeqCst);
        let dbg = Dbg::new(&parent, format!("Worker({id})"));
        size.fetch_add(1, Ordering::SeqCst);
        log::debug!("{dbg}.new | New one created, catacity: {}, size: {}, free: {}", capacity.load(Ordering::SeqCst), size.load(Ordering::SeqCst), 1 + free.load(Ordering::SeqCst));
        let thread = std::thread::spawn(move || loop {
            // let error = Error::new(&dbg, "new");
            free.fetch_add(1, Ordering::SeqCst);
            if free.load(Ordering::SeqCst) < 2 {
                Self::extend(&parent, &dbg, receiver.clone(), capacity.clone(), size.clone(), free.clone(), workers.clone());
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
                            log::error!("Worker({dbg}).new | Recv error, channel closed, details: \n\t{:?}", err);
                            break;
                        }
                    }
                }
                Err(err) => {
                    log::error!("Worker({dbg}).new | Lock error: {:?}", err);
                    None
                }
            };
            match job {
                Some(job) => {
                    log::debug!("Worker({dbg}).new | Executing job...");
                    free.fetch_sub(1, Ordering::SeqCst);
                    let _ = job();
                    log::debug!("Worker({dbg}).new | Done job...");
                }
                None => {}
            }
        });
        Worker { id, thread }
    }
    ///
    /// Extending current number of [Worker]'s if required
    fn extend(
        parent: impl Into<String>,
        dbg: &Dbg,
        receiver: Arc<Mutex<kanal::Receiver<Job>>>,
        capacity: Arc<AtomicUsize>,
        size: Arc<AtomicUsize>,
        free: Arc<AtomicUsize>,
        workers: Arc<Stack<Worker>>
    ) {
        let parent = parent.into();
        let new_workers = size.load(Ordering::SeqCst) * 2;
        log::debug!("{dbg}.extend | Creating {new_workers} new workers...");
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
    /// Panics
    /// This function may panic on some platforms if a thread attempts to join itself or otherwise may create a deadlock with joining threads.
    pub fn join(self) -> Result<(), Box<dyn std::any::Any + Send + 'static>> {
        self.thread.join()
    }
}
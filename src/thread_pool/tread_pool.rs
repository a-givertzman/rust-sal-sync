use std::sync::{atomic::AtomicUsize, Arc, Mutex};
use coco::Stack;
use sal_core::error::Error;
use super::{job::Job, scheduler::Scheduler, worker::Worker};
///
/// 
pub struct ThreadPool {
    workers: Arc<Stack<Worker>>,
    sender: kanal::Sender<Job>,
    // link: SkipMap<usize, (kanal::Sender<()>, kanal::Receiver<Job>)>,
    /// Maximum possible number of [Worker]'s
    capacity: usize,
    /// Current total number of [Worker]'s
    size: Arc<AtomicUsize>,
    /// not busy [Worker]'s
    free: Arc<AtomicUsize>,
}
//
//
impl ThreadPool {
    ///
    /// Returns [ThreadPool] new instance
    pub fn new(size: Option<usize>) -> Self {
        let default_capacity = 64;
        let capacity = match size {
            Some(capacity) => {
                if capacity == 0 {
                    log::warn!("ThreadPool.new | Size of th ThreadPool cant be zero, used default size {default_capacity}");
                    default_capacity
                } else {
                    capacity
                }
            }
            None => default_capacity,
        };
        let size = Arc::new(AtomicUsize::new(0));
        let free = Arc::new(AtomicUsize::new(0));
        let (sender, receiver) = kanal::unbounded();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = Arc::new(Stack::new());
        for id in 0..1 {
            workers.push(Worker::new(id, Arc::clone(&receiver), size.clone(), free.clone(), workers.clone()));

        }
        ThreadPool {
            workers,
            sender,
            capacity,
            size,
            free,
        }
    }
    ///
    /// Returns [Scheduler] linked to the current [TreadPool]
    pub fn scheduler(&self) -> Scheduler {
        Scheduler::new(self.sender.clone())
    }
    ///
    /// Spawns a new task to be scheduled on the [ThreadPool]
    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() -> Result<(), Error> + Send + 'static {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
    ///
    /// 
    pub fn join(&self) -> Result<(), Error> {
        let error = Error::new("ThreadPool", "join");
        let mut errors = vec![];
        while !self.workers.is_empty() {
            match self.workers.pop() {
                Some(th) => if let Err(err) = th.join() {
                    let err = error.pass(format!("{:?}", err));
                    log::warn!("{}", err);
                    errors.push(err);
                }
                None => break,
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
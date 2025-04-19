use std::sync::{Arc, Mutex};
use coco::Stack;
use sal_core::error::Error;
use super::{job::Job, scheduler::Scheduler, worker::Worker};
///
/// 
pub struct ThreadPool {
    workers: Stack<Worker>,
    sender: kanal::Sender<Job>,
    // link: SkipMap<usize, (kanal::Sender<()>, kanal::Receiver<Job>)>,
    capacity: usize,
    size: usize,
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
        let (sender, receiver) = kanal::unbounded();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = Stack::new();
        workers.push(Worker::new(0, Arc::clone(&receiver)));
        ThreadPool {
            workers,
            sender,
            capacity,
            size: 1,
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
use std::sync::{Arc, Mutex};
use coco::Stack;
use crossbeam_skiplist::SkipMap;
use sal_core::error::Error;
use super::{job::Job, scheduler::Scheduler, worker::Worker};
///
/// 
pub struct ThreadPool {
    workers: Stack<Worker>,
    sender: kanal::Sender<Job>,
    link: SkipMap<usize, (kanal::Sender<()>, kanal::Receiver<Job>)>,
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
            link: SkipMap::new(),
            capacity,
            size: 1,
        }
    }
    ///
    /// Returns [Scheduler] linked to the current [TreadPool]
    pub fn scheduler(&self) -> Scheduler {
        let (loc_send, rem_recv) = kanal::bounded(100);
        let (rem_send, loc_recv) = kanal::bounded(100);
        self.link.insert(self.link.len() + 1, (loc_send, loc_recv));
        Scheduler::new(rem_send, rem_recv)
    }
    ///
    /// Spawns a new task to be scheduled on the [ThreadPool]
    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() -> Result<(), Error> + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
use std::sync::{Arc, Mutex};
use crossbeam_skiplist::SkipMap;
use super::{job::Job, scheduler::Scheduler, worker::Worker};
///
/// 
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: kanal::Sender<Job>,
    link: SkipMap<usize, (kanal::Sender<()>, kanal::Receiver<Job>)>,
}
//
//
impl ThreadPool {
    ///
    /// Returns [ThreadPool] new instance
    pub fn new(size: Option<usize>) -> Self {
        let size = size.unwrap_or(64);
        assert!(size > 0, "ThreadPool.new | Size of th ThreadPool cant be Zero");
        let (sender, receiver) = kanal::unbounded();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender,
            link: SkipMap::new(),
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
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
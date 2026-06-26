use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use crossbeam::queue::SegQueue;
use sal_core::{dbg::Dbg, error::Error};
use super::{job::Job, scaling::Scaling, scheduler::Scheduler, worker::Worker, JoinHandle};
///
/// Provides ready to execute specified number of threads
/// - From start has only 1 or 2 prepared treads
/// - If all prepared threads are busy, new treds will be added to pool
/// - Number of threads limited by capacity, by default 64
pub struct ThreadPool {
    workers: Arc<SegQueue<Worker>>,
    scheduler: Scheduler,
    sender: kanal::Sender<Job>,
    scaling: Arc<Scaling>,
    /// Shutdoun requested
    exit: Arc<AtomicBool>,
}
//
impl ThreadPool {
    ///
    /// Returns [ThreadPool] new instance
    /// - `capacity` maximum number of threads, by default 64
    pub fn new(parent: impl Into<String>, capacity: Option<usize>) -> Self {
        let dbg = Dbg::new(parent, "ThreadPool");
        let exit = Arc::new(AtomicBool::new(false));
        let (sender, receiver) = kanal::bounded(16_000);
        let workers = Arc::new(SegQueue::new());
        let scaling = Scaling::new(&dbg, capacity, receiver.clone(), workers.clone(), exit.clone());
        let size = (scaling.capacity() / 4).max(1);
        log::debug!("{dbg}.new | Creating {size} new workers...");
        for _ in 0..size {
            let id = scaling.size() + 1;
            workers.push(Worker::new(
                &dbg,
                id,
                receiver.clone(),
                scaling.clone(),
            ));
        }
        ThreadPool {
            workers,
            scheduler: Scheduler::new(scaling.clone(), sender.clone()),
            sender,
            scaling,
            exit,
        }
    }
    ///
    /// Maximum possible number of [Worker]'s
    pub fn capacity(&self) -> usize {
        self.scaling.capacity()
    }
    ///
    /// Current number of [Worker]'s
    pub fn size(&self) -> usize {
        self.scaling.size()
    }
    ///
    /// Current not a busy [Worker]'s
    pub fn free(&self) -> usize {
        self.scaling.free()
    }
    ///
    /// Returns [Scheduler] linked to the current [TreadPool]
    /// 
    /// **Example**
    /// ```ignore
    /// let thread_pool = ThreadPool::new(&dbg, Some(1));
    /// let scheduler = thread_pool.scheduler();
    /// let result = scheduler.spawn(move || {
    ///     std::thread::sleep(Duration::from_millis(load));
    ///     result.fetch_add(1, Ordering::AcqRel);
    /// }).unwrap();
    /// assert!(result.join().unwrap() == ());
    /// thread_pool.join().unwrap();
    /// ```
    pub fn scheduler(&self) -> Scheduler {
        Scheduler::new(self.scaling.clone(), self.sender.clone())
    }
    ///
    /// Spawns a new task to be scheduled on the [ThreadPool]
    pub fn spawn<F, T>(&self, f: F) -> Result<JoinHandle<T>, Error>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.scheduler.spawn(f)
    }
    ///
    /// Spawns a named new task to be scheduled on the [ThreadPool]
    pub fn spawn_named<F, T>(&self, name: impl Into<String> ,f: F) -> Result<JoinHandle<T>, Error>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.scheduler.spawn_named(name, f)
    }
    ///
    /// Returns all workers from self.workers
    fn send_exit_workers(&self) -> Vec<Worker> {
        if self.workers.is_empty() {
            return vec![];
        }
        let mut workers = Vec::with_capacity(self.workers.len());
        while !self.workers.is_empty() {
            match self.workers.pop() {
                Some(worker) => {
                    workers.push(worker);
                }
                None => break,
            }
        }
        workers
    }
    ///
    /// Sends `Shutdown` signal to all scheduled tasks and join them.
    /// This means all tasks will finish current jobs and then exit.
    pub fn join(&self) -> Result<(), Error> {
        self.shutdown()
    }
    ///
    /// ### Placed a shutdown signal for all workers and gracefully waits for them to finish.
    /// 
    /// Existing sheduled tasks will be processed before workers exit.
    /// 
    /// > **Важно! Опасность дедлока при Graceful Shutdown:**
    /// В ThreadPool очередь задач инициализируется как `kanal::bounded(16_000)`.
    /// Если пользователь закинет 16 000 задач и сразу вызовет `shutdown()` (или объект выйдет из скоупа и сработает Drop),
    /// очередь будет заполнена под завязку. Добавление новой задачи и постановка ее в очередь это блокирующая операция.
    /// Главный поток зависнет ожидая, пока воркеры разгребут очередь и освободят место для сигнала `Shutdown`.
    /// 
    /// > **Dead Lock**: Если задачи в очереди завязаны на ожидании чего-либо от главного потока (который сейчас заблокирован),
    /// произойдет глухой дедлок.
    pub fn shutdown(&self) -> Result<(), Error> {
        let error = Error::new("ThreadPool", "shutdown");
        self.exit.store(true, Ordering::Release);
        // Даем воркерам время забрать все задачи из канала
        while !self.sender.is_empty() {
            std::thread::yield_now();
        }
        if let Err(err) = self.sender.close() {
            log::debug!("ThreadPool.shutdown | Channel close error: {:?}", err);
        }
        let mut errors = vec![];
        let mut remaining_workers = self.send_exit_workers();
        log::trace!("ThreadPool.shutdown | Shutdown signal sent to {} workers", remaining_workers.len());
        while !remaining_workers.is_empty() {
            if let Some(worker) = remaining_workers.pop() {
                log::debug!("ThreadPool.shutdown | Wait for worker {} of {}...", worker.id(), remaining_workers.len() + 1);
                if let Err(err) = worker.join() {
                    let err = error.pass(format!("{:?}", err));
                    log::warn!("{}", err);
                    errors.push(err);
                }
            }
            if !self.workers.is_empty() {
                remaining_workers.extend(self.send_exit_workers());
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
//
//
impl Drop for ThreadPool {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

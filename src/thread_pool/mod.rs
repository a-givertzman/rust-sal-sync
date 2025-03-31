//!
//! ThreadPool::new creates a fixed number of workers, if specified or by default 64
//! 
//! Use ThreadPool::spawn to execute task on the tread pool
//! 
//! Use Scheduler to send task to the tread pool late
//! 
//! ```ignore
//! let pool = TreadPool::new(None);
//! 
//! pool.spawn(|| {
//!     todo!("Do some thing on the tread executed on the tread pool")
//! });
//! 
//! fn service(scheduler: Scheduler) {
//!     scheduler.spawn(|| {
//!         todo!("Do some thing late on the tread executed on the tread pool")
//!     });
//! }
//!
//! let scheduler = pool.scheduler();
//! service(scheduler);
//! ```
//! 
//! 
pub mod job;
pub mod scheduler;
pub mod tread_pool;
pub mod worker;

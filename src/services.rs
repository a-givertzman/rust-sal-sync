//! ### Services implemented for the application
//! **Service**:
//! - executed in the separate thread, can be multi thread
//! - basicaly must be defined in the main configuration file like:
//! ```yaml
//! service ServiceName Id:
//!     in queue in-queue:
//!         max-length: 10000
//!     send-to: MultiQueue.in-queue
//! ```
///
pub mod conf;
pub mod entity;
pub mod future;
// pub mod multi_queue;
pub mod service;
// pub mod services;        <=  RetainPointId, SafeLock, QueueName
pub mod subscription;
pub mod task;
pub mod types;

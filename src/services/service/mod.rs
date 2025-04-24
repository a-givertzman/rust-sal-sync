//!
//! # Application service
//! 
//! ## Service
//! 
//! - Executed in the separate thread, can be multi thread
//! - Event-driven data exchanging
//! 
//! ## Configuration example
//! 
//! ```yaml
//! service ServiceName Id:
//!     in queue in-queue:
//!         max-length: 10000
//!     send-to: MultiQueue.in-queue
//! ```
//!
use std::time::Duration;
mod link_name;
mod service_cycle;
mod service_handles;
mod service;
pub const RECV_TIMEOUT: Duration = Duration::from_millis(100);

pub use link_name::*;
pub use service_cycle::*;
pub use service_handles::*;
pub use service::*;

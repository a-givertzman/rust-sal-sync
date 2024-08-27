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
pub mod link_name;
pub mod service_cycle;
pub mod service_handles;
pub mod service;
pub mod wait;

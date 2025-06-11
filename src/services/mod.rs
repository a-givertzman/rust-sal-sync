//! ### Application Services types, entities and tools
//! 
//! ## Number of tool
//! 
//! - Executing Service's in the separate threads
//! - Event-driven exchanging between the Service's
//! - Storing / loading configurations parameters
//! - Storing / loading runtime data
//! 
//! ## Configuration example
//! 
//! Following configuration contains main application parameters, ProfinetClient Service and Task Service
//! (Service's configuration sometimes can be incorrect, please check corresponding docs)
//! 
//! ```yaml
//! name: ApplicationName
//! description: Short explanation / purpose etc.
//! retain:
//!     api:
//!         table:      public.tags
//!         address:    0.0.0.0:8080
//!         auth_token: 123!@#
//!         database:   cma_data_server
//! 
//! service ProfinetClient Ied01:          # device will be executed in the independent thread, must have unique name
//!    in queue in-queue:
//!        max-length: 10000
//!    send-to: MultiQueue.in-queue
//!    cycle: 1 ms                     # operating cycle time of the device
//!    protocol: 'profinet'
//!    description: 'S7-IED-01.01'
//!    ip: '192.168.100.243'
//!    rack: 0
//!    slot: 1
//!    db db899:                       # multiple DB blocks are allowed, must have unique namewithing parent device
//!        description: 'db899 | Exhibit - drive data'
//!        number: 899
//!        offset: 0
//!        size: 34
//!        point Drive.Speed: 
//!            type: 'Real'
//!            offset: 0
//!                 ...
//! service Task Task1:
//!     cycle: 1 ms
//!     in queue recv-queue:
//!         max-length: 10000
//!     let var0: 
//!         input: const real 2.224
//!     
//!     fn ToMultiQueue:
//!         in1 point CraneMovement.BoomUp: 
//!             type: 'Int'
//!             comment: 'Some indication'
//!             input fn Add:
//!                 input1 fn Add:
//!                     input1: const real 0.2
//!                     input2: point real '/path/Point.Name'
//!     ...
//! ```
mod service;
mod services;

pub mod conf;
pub mod entity;
pub mod future;
mod multi_queue;
pub mod retain;
mod subscription;
pub mod task;
pub mod types;

pub use multi_queue::*;
pub use subscription::*;
pub use service::*;
pub use services::*;
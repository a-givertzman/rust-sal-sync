use sal_core::error::Error;
use crate::{services::{
    entity::{Object, Point, PointConf}, future::Future, subscription::SubscriptionCriteria
}, sync::channel::{Receiver, Sender}};
///
/// Interface for application service
/// - Running in the individual thread
pub trait Service: Object + std::fmt::Debug + Send + Sync {
    // ///
    // /// Returns service's ID
    // fn id(&self) -> &str;
    ///
    /// Returns copy of the Sender - service's incoming queue
    #[allow(unused_variables)]
    fn get_link(&self, name: &str) -> Sender<Point> {
        panic!("{}.get_link | Is not implemented", self.name())
    }
    ///
    /// Returns Receiver
    #[allow(unused_variables)]
    fn subscribe(&self, recvr_id: &str, points: &[SubscriptionCriteria]) -> (Sender<Point>, Receiver<Point>) {
        let err = Error::new(&self.name(), "subscribe").err(format!("Request from '{recvr_id}', But not implemented"));
        panic!("{err}")
    }
    ///
    /// Extends the sucessfully with additiuonal points
    #[allow(unused_variables)]
    fn extend_subscription(&self, recvr_id: &str, points: &[SubscriptionCriteria]) -> Result<(), Error> {
        let err = Error::new(&self.name(), "extend_subscription").err(format!("Request from '{recvr_id}', But not implemented"));
        panic!("{err}")
    }
    ///
    /// Canceling the subsciption
    #[allow(unused_variables)]
    fn unsubscribe(&self, recvr_id: &str, points: &[SubscriptionCriteria]) -> Result<(), Error> {
        let err = Error::new(&self.name(), "unsubscribe").err(format!("Request from '{recvr_id}', But not implemented"));
        panic!("{err}")
    }
    ///
    /// Starts service's main loop in the individual thread
    fn run(&self) -> Result<(), Error>;
    ///
    /// Returns list of configurations of the defined points
    fn points(&self) -> Vec<PointConf> {
        vec![]
    }
    ///
    /// Returns `Future<Point>`, where will be pushed all points by subscription
    fn gi(&self, recvr_id: &str, points: &[SubscriptionCriteria]) -> Future<Vec<Point>> {
        let _ = points;
        let err = Error::new(&self.name(), "gi").err(format!("Request from '{recvr_id}', But not implemented"));
        panic!("{err}")
    }
    ///
    /// Waits for the [Service] to finish.
    ///
    /// Returns immediately if the [Service] has already finished.
    /// 
    /// ## Panics
    /// - If not implemented for associated [Service]
    /// - If specific implementation may panics internally,
    ///   like `std::thread::JoinHandle` - may panic on some platforms 
    ///   if a thread attempts to join itself or otherwise may create a deadlock with joining threads.
    fn wait(&self) -> Result<(), Error> {
        panic!("{}.wait | Is not implemented", self.name())
    }
    ///
    /// Checks if the [Service] has finished running.
    /// 
    /// To finish the [Service] call exit
    fn is_finished(&self) -> bool;
    ///
    /// Sends "exit" signal to the service's thread
    fn exit(&self);
}
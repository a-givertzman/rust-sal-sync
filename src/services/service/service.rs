use sal_core::error::Error;
use crate::{services::{
    entity::{Object, Point, PointConfig}, future::Future, subscription::SubscriptionCriteria
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
        panic!("{}.get_link | Does not supported", self.name())
    }
    ///
    /// Returns Receiver
    #[allow(unused_variables)]
    fn subscribe(&self, receiver_id: &str, points: &[SubscriptionCriteria]) -> (Sender<Point>, Receiver<Point>) {
        panic!("{}.subscribe | Does not supported", self.name())
    }
    ///
    /// Extends the sucessfully with additiuonal points
    #[allow(unused_variables)]
    fn extend_subscription(&self, receiver_name: &str, points: &[SubscriptionCriteria]) -> Result<(), Error> {
        panic!("{}.extend_subscription | Does not supported", self.name())
    }
    ///
    /// Canceling the subsciption
    #[allow(unused_variables)]
    fn unsubscribe(&self, receiver_name: &str, points: &[SubscriptionCriteria]) -> Result<(), Error> {
        panic!("{}.unsubscribe | Does not supported", self.name())
    }
    ///
    /// Starts service's main loop in the individual thread
    fn run(&self) -> Result<(), Error>;
    ///
    /// Returns list of configurations of the defined points
    fn points(&self) -> Vec<PointConfig> {
        vec![]
    }
    ///
    /// Returns `Future<Point>`, where will be pushed all points by subscription
    fn gi(&self, _receiver_name: &str, _points: &[SubscriptionCriteria]) -> Future<Point> {
        panic!("{}.gi | Does not supported", self.name())
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
        panic!("{}.wait | Does not supported", self.name())
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
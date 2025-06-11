use std::{fmt::Debug, hash::BuildHasherDefault};
use hashers::fx_hash::FxHasher;
use sal_core::error::Error;
use crate::{collections::FxDashMap, services::entity::Point, sync::channel::Sender};
///
/// Unique id of the service receiving the Point's by the subscription
/// This id used to identify the service produced the Points. 
/// To avoid send back self produced Point's.
type ReceiverId = usize;
///
/// Destination of the point,
/// Currently it's just a concat of the Point.cot & Point.id 
type PointDest = String; 
///
/// Contains map of Sender's
/// - Where Sender - is pair of String ID & Sender<PointType>
#[derive(Clone)]
pub struct Subscriptions {
    dbg: String,
    multicast: FxDashMap<PointDest, FxDashMap<ReceiverId, Sender<Point>>>,
    broadcast: FxDashMap<ReceiverId, Sender<Point>>,
}
//
// 
impl Subscriptions {
    ///
    /// Creates new instance of Subscriptions
    pub fn new(parent: impl Into<String>, ) -> Self {
        Self {
            dbg: format!("{}/Subscriptions", parent.into()),
            multicast: FxDashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
            broadcast: FxDashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
        }
    }
    ///
    /// Adds subscription for receiver_id with destination 
    pub fn add_multicast(&self, receiver_id: usize, destination: &str, sender: Sender<Point>) {
        self.multicast
            .entry(destination.to_owned())
            .or_insert(FxDashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()))
            .insert(receiver_id, sender);
    }
    ///
    /// Extends subscription if exists, otherwise returns error
    pub fn extend_multicast(&self, receiver_id: usize, destination: &str) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "extend_multicast");
        match self.multicast.iter().find_map(|r| {
            r
                .value()
                .get(&receiver_id)
                .map(|v| v.clone())
        }) {
            Some(sender) => {
                self.add_multicast(receiver_id, destination, sender);
                Ok(())
            }
            None => {
                Err(error.err(format!("Receiver '{}' - not found in subscriptions", receiver_id)))
            }
        }
    }
    ///
    /// Adds subscription for receiver_id without destination, all destinations will be received
    pub fn add_broadcast(&self, receiver_id: usize, sender: Sender<Point>) {
        self.broadcast.insert(
            receiver_id,
            sender,
        );
    }
    ///
    /// Returns all pairs of `key`, `Senders`
    pub fn get(&self, point_id: &str) -> Vec<(usize, Sender<Point>)> {
        match self.multicast.get(point_id) {
            Some(multicast) => {
                log::trace!("{}.iter | \n\t Multicast: {:?} \n\t Broadcast: {:?}", self.dbg, multicast, self.broadcast);
                multicast.iter().chain(&self.broadcast).map(|r| (*r.key(), r.value().clone())).collect()
            }
            None => {
                log::trace!("{}.iter | \n\t Broadcast: {:?}", self.dbg, self.broadcast);
                self.broadcast.iter().map(|r| (*r.key(), r.value().clone())).collect()
            }
        }
    }
    ///
    /// Removes single subscription by Point Id for receiver ID
    pub fn remove(&self, receiver_id: &usize, point_id: &str) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "remove");
        match self.multicast.get_mut(point_id) {
            Some(senders) => {
                match senders.remove(receiver_id) {
                    Some(_) => Ok(()),
                    None => Err(error.err(format!("Subscription '{}', receiver '{}' - not found", point_id, receiver_id))),
                }
            }
            None => Err(error.err(format!("Subscription '{}' - not found", point_id))),
        }
    }
    ///
    /// Removes all subscriptions for receiver ID
    pub fn remove_all(&self, receiver_id: &usize) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "remove_all");
        let mut changed = false;
        let mut messages = vec![];
        let keys: Vec<String> = self.multicast.iter().map(|r| r.key().clone()).collect();
        for point_id in keys {
            match self.multicast.get_mut(&point_id) {
                Some(senders) => {
                    match senders.remove(receiver_id) {
                        Some(_) => {
                            changed |= true;
                        }
                        None => {
                            messages.push(format!("{}.run | Multicast Subscription '{}', receiver '{}' - not found", self.dbg, point_id, receiver_id));
                        }
                    }
                }
                None => {
                    messages.push(format!("{}.run | Multicast Subscription '{}' - not found", self.dbg, point_id));
                }
            }
        }
        match self.broadcast.remove(receiver_id) {
            Some(_) => {
                changed |= true;
            }
            None => {
                messages.push(format!("{}.run | Broadcast Subscription by receiver '{}' - not found", self.dbg, receiver_id));
            }
        }
        if changed {
            Ok(())
        } else {
            Err(error.err(messages.join("\n")))
        }
    }
    ///
    /// Removes all subscriptions
    pub fn exit(&self) {
        self.broadcast.clear();
        self.multicast.clear();
    }
}
//
// 
impl Debug for Subscriptions {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Subscriptions")
            .field("dbg", &self.dbg)
            .finish()
    }
}

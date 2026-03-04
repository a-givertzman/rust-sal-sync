use std::{fmt::Debug, hash::BuildHasherDefault, sync::Arc};
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
    registry: FxDashMap<ReceiverId, Sender<Point>>,
    multicast: FxDashMap<PointDest, Arc<FxDashMap<ReceiverId, Sender<Point>>>>,
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
            registry: FxDashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
            multicast: FxDashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
            broadcast: FxDashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
        }
    }
    ///
    /// Adds subscription for receiver_id with destination 
    pub fn add_multicast(&self, receiver_id: usize, destination: &str, sender: Sender<Point>) {
        self.registry.entry(receiver_id).or_insert(sender.clone());
        match self.multicast.get(destination).map(|r| r.value().clone()) {
            Some(multicast) => {
                multicast.insert(receiver_id, sender);
            }
            None => {
                let receivers = Arc::new(FxDashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()));
                receivers.insert(receiver_id, sender);
                self.multicast.insert(destination.to_owned(), receivers);
            }
        }
    }
    ///
    /// Extends subscription if exists, otherwise returns error
    pub fn extend_multicast(&self, receiver_id: usize, destination: &str) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "extend_multicast");
        log::debug!("{}.extend_multicast | Extending (multicast) for receiver: {} ({})...", self.dbg, destination, receiver_id);
        let s = self.multicast.iter()
            .find(|r| r.contains_key(&receiver_id))
            .map(|r| r.value().get(&receiver_id).map(|s| s.value().clone()))
            .flatten();
        match self.registry.get(&receiver_id).map(|s| s.clone()) {
            Some(sender) => {
                match self.multicast.get(destination).map(|r| r.value().clone()) {
                    Some(multicast) => {
                        multicast.insert(receiver_id, sender);
                    }
                    None => {
                        let receivers = Arc::new(FxDashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()));
                        receivers.insert(receiver_id, sender);
                        self.multicast.insert(destination.to_owned(), receivers);
                    }
                }
                log::debug!("{}.extend_multicast | Extending (multicast) for receiver: {} ({}) - Ok", self.dbg, destination, receiver_id);
                Ok(())
            }
            None => {
                log::warn!("{}.extend_multicast | Extending (multicast) for receiver: {} ({receiver_id}) - Receiver '{receiver_id}' - not found", self.dbg, destination);
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
        match self.multicast.get(point_id).map(|r| r.value().clone()) {
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
        match self.multicast.get_mut(point_id).map(|r| r.value().clone()) {
            Some(senders) => {
                match senders.remove(receiver_id) {
                    Some((_, s)) => {
                        if s.sender_count() <= 2 {
                            self.registry.remove(receiver_id);
                        }
                        Ok(())
                    }
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
        let dest_ids: Vec<PointDest> = self.multicast.iter().map(|r| r.key().clone()).collect();
        for point_dest in dest_ids {
            match self.multicast.get(&point_dest).map(|r| r.value().clone()) {
                Some(senders) => {
                    match senders.remove(receiver_id) {
                        Some(_) => {
                            changed |= true;
                        }
                        None => {
                            messages.push(format!("{}.run | Multicast Subscription '{}', receiver '{}' - not found", self.dbg, point_dest, receiver_id));
                        }
                    }
                }
                None => {
                    messages.push(format!("{}.run | Multicast Subscription '{}' - not found", self.dbg, point_dest));
                }
            }
        }
        self.registry.remove(receiver_id);
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

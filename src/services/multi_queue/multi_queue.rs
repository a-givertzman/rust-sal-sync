use std::{
    collections::HashMap, fmt::Debug, fs, hash::BuildHasherDefault, io::Write,
    sync::{atomic::{AtomicBool, Ordering}, Arc},
};
use coco::Stack;
use concat_string::concat_string;
use sal_core::{dbg::{self, dbg, Dbg}, error::Error};
use crate::{
    collections::FxDashMap, services::{
        entity::{Name, Object, Point, PointTxId},
        service::{LinkName, Service, RECV_TIMEOUT},
        services::Services, subscription::{SubscriptionCriteria, Subscriptions},
    },
    sync::{channel::{self, Receiver, Sender}, WaitBox}, thread_pool::Scheduler,
};
use super::multi_queue_conf::MultiQueueConf;
///
/// ### Receive and destribute `Point`'s across multiple services
/// - Thread safe
/// - Receives `Point`'s into the MPSC queue in the blocking mode
/// - If new point received, immediately sends it to the all subscribed consumers
/// - Keeps all consumers subscriptions in the single map
pub struct MultiQueue {
    dbg: Dbg,
    name: Name,
    subscriptions: Arc<Subscriptions>,
    subscriptions_changed: Arc<AtomicBool>,
    rx_send: HashMap<String, Sender<Point>>,
    rx_recv: Stack<Receiver<Point>>,
    send_queues: Vec<LinkName>,
    services: Arc<Services>,
    schrduler: Option<Scheduler>,
    receiver_dictionary: FxDashMap<usize, String>,
    handle: Stack<Box<dyn WaitBox<()>>>,
    is_finished: Arc<AtomicBool>,
    exit: Arc<AtomicBool>,
}
//
//
impl MultiQueue {
    ///
    /// Creates new instance of [ApiClient]
    /// - [parent] - the ID if the parent entity
    pub fn new(conf: MultiQueueConf, services: Arc<Services>, schrduler: Option<Scheduler>) -> Self {
        let dbg = Dbg::new(conf.name.parent(), conf.name.me());
        let (send, recv) = channel::unbounded();
        let send_queues = conf.send_to;
        let rx_recv = Stack::new();
        rx_recv.push(recv);
        Self {
            name: conf.name.clone(),
            subscriptions: Arc::new(Subscriptions::new(&dbg)),
            subscriptions_changed: Arc::new(AtomicBool::new(false)),
            rx_send: HashMap::from([(conf.rx, send)]),
            rx_recv,
            send_queues,
            services,
            schrduler,
            receiver_dictionary: FxDashMap::with_hasher(BuildHasherDefault::default()),
            handle: Stack::new(),
            is_finished: Arc::new(AtomicBool::new(false)),
            exit: Arc::new(AtomicBool::new(false)),
            dbg,
        }
    }
    ///
    /// Writes Subscription's to the log file 
    fn log(&self, name: &str, receiver_name: &str, rceiver_hash: usize, points: &[SubscriptionCriteria]) {
        let path = concat_string!("./logs", self.name.join(), name);
        let destinations: Vec<String> = points.iter().map(|cr| {cr.destination()}).collect();
        match fs::OpenOptions::new().create(true).append(true).open(&path) {
            Ok(mut f) => {
                f.write_fmt(format_args!("\n\n\t{} ({})", receiver_name, rceiver_hash)).unwrap();
                match serde_json::to_writer_pretty(f, &destinations) {
                    Ok(_) => {}
                    Err(err) => {
                        if log::max_level() >= log::LevelFilter::Trace {
                            log::warn!("{}.log | Error writing to file: '{}'\n\terror: {:?}", self.dbg, path, err)
                        }
                    }
                }
            }
            Err(err) => {
                if log::max_level() >= log::LevelFilter::Trace {
                    log::warn!("{}.log | Error open file: '{}'\n\terror: {:?}", self.dbg, path, err)
                }
            }
        }
    }
    ///
    /// Writes Point's to the log file 
    fn log_point(dbg: &Dbg, parent: &Name, point_id: &str, point: &Point) {
        let path = concat_string!("./logs", parent.join(), "/points.log");
        match fs::OpenOptions::new().create(true).append(true).open(&path) {
            Ok(mut f) => {
                f.write_fmt(format_args!("'{}': {:?}\n",point_id, point)).unwrap();
            }
            Err(err) => {
                if log::max_level() >= log::LevelFilter::Trace {
                    log::warn!("{}.log | Error open file: '{}'\n\terror: {:?}", dbg, path, err)
                }
            }
        }
    }
    ///
    /// Main loop
    fn run_(dbg: Dbg, name: Name, recv: Receiver<Point>, subscriptions_ref: Arc<Subscriptions>, subscriptions_changed: Arc<AtomicBool>, exit: Arc<AtomicBool>) {
            log::info!("{}.run | Preparing thread - ok", dbg);
            let mut subscriptions = subscriptions_ref.clone();
            loop {
                if subscriptions_changed.load(Ordering::Relaxed) {
                    subscriptions_changed.store(false, Ordering::SeqCst);
                    log::debug!("{}.run | Subscriptions changes detected", dbg);
                    subscriptions = subscriptions_ref.clone();
                }
                match recv.recv_timeout(RECV_TIMEOUT) {
                    Ok(point) => {
                        let point_id = SubscriptionCriteria::new(&point.name(), point.cot()).destination();
                        log::trace!("{}.run | received: \n\t{:?}", dbg, point);
                        Self::log_point(&dbg, &name, &point_id, &point);
                        for (receiver_hash, sender) in subscriptions.get(&point_id) {
                            if receiver_hash != point.tx_id() {
                                match sender.send(point.clone()) {
                                    Ok(_) => {
                                        log::trace!("{}.run | sent to '{}' point: {:?}", dbg, receiver_hash, point);
                                    }
                                    Err(err) => {
                                        log::error!("{}.run | subscriptions '{}', receiver '{}' - send error: {:?}", dbg, point_id, receiver_hash, err);
                                    }
                                };
                            }
                        }
                    }
                    Err(err) => {
                        log::trace!("{}.run | recv timeout: {:?}", dbg, err);
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    subscriptions_ref.exit();
                    break;
                }
            }
            log::info!("{}.run | Exit", dbg);
    }
}
//
//
impl Object for MultiQueue {
    fn name(&self) -> Name {
        self.name.clone()
    }
}
//
//
impl Debug for MultiQueue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("MultiQueue")
            .field("id", &self.dbg)
            .finish()
    }
}
//
//
impl Service for MultiQueue {
    //
    //
    fn get_link(&self, name: &str) -> Sender<Point> {
        match self.rx_send.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.dbg, name),
        }
    }
    //
    //
    #[dbg]
    fn subscribe(&self, receiver_name: &str, points: &[SubscriptionCriteria]) -> (Sender<Point>, Receiver<Point>) {
        let (send, recv) = channel::unbounded();
        let receiver_hash = PointTxId::from_str(receiver_name);
        self.receiver_dictionary.insert(receiver_hash, receiver_name.to_string());
        if points.is_empty() {
            self.subscriptions.add_broadcast(receiver_hash, send.clone());
            self.log("/broadcast.log", receiver_name, receiver_hash, points);
            dbg::debug!("Broadcast registered, receiver: \n\t{} ({})", receiver_name, receiver_hash);
        } else {
            for subscription_criteria in points {
                self.subscriptions.add_multicast(receiver_hash, &subscription_criteria.destination(), send.clone());
            }
            self.log("/multicast.log", receiver_name, receiver_hash, points);
            dbg::debug!("Multicast registered, receiver: \n\t{} ({}) \n\tpoints: {:#?}", receiver_name, receiver_hash, points.len());
            dbg::trace!("Multicast registered, receiver: \n\t{} ({}) \n\tpoints: {:#?}", receiver_name, receiver_hash, points);
        }
        self.subscriptions_changed.store(true, Ordering::SeqCst);
        (send, recv)
    }
    //
    //
    #[dbg]
    fn extend_subscription(&self, receiver_name: &str, points: &[SubscriptionCriteria]) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "extend_subscription");
        let receiver_hash = PointTxId::from_str(receiver_name);
        if points.is_empty() {
            Err(error.err(format!("Can't be extended (broadcast), receiver: {} ({})", receiver_name, receiver_hash)))
        } else {
            let mut message = String::new();
            for subscription_criteria in points {
                dbg::trace!("Extending (multicast) for receiver: {} ({})...", receiver_name, receiver_hash);
                if let Err(err) = self.subscriptions.extend_multicast(receiver_hash, &subscription_criteria.destination()) {
                    message = concat_string!(message, err.to_string(), "\n");
                };
            }
            self.log("/multicast.log", receiver_name, receiver_hash, points);
            if message.is_empty() {
                dbg::debug!("Extended (multicast), receiver: {} ({})", receiver_name, receiver_hash);
                self.subscriptions_changed.store(true, Ordering::SeqCst);
                Ok(())
            } else {
                dbg::debug!("Extended (multicast), receiver: {} ({}) \n\t with errors: {:?}", receiver_name, receiver_hash, message);
                self.subscriptions_changed.store(true, Ordering::SeqCst);
                Err(error.err(message))
            }
        }
    }
    //
    //
    #[dbg]
    fn unsubscribe(&self, receiver_name: &str, points: &[SubscriptionCriteria]) -> Result<(), Error> {
        let mut changed = false;
        let error = Error::new(&self.dbg, "unsubscribe");
        let receiver_hash = PointTxId::from_str(receiver_name);
        if points.is_empty() {
            match self.subscriptions.remove_all(&receiver_hash) {
                Ok(_) => {
                    self.receiver_dictionary.remove(&receiver_hash);
                    changed |= true;
                    dbg::debug!("Broadcast subscription removed, receiver: {} ({})", receiver_name, receiver_hash);
                }
                Err(err) => {
                    return Err(error.pass(err))
                }
            }
        } else {
            for subscription_criteria in points {
                match self.subscriptions.remove(&receiver_hash, &subscription_criteria.destination()) {
                    Ok(_) => {
                        self.receiver_dictionary.remove(&receiver_hash);
                        changed |= true;
                        dbg::debug!("Multicat subscription '{}' removed, receiver: {} ({})", subscription_criteria.destination(), receiver_name, receiver_hash);
                    }
                    Err(err) => {
                        return Err(error.pass(err))
                    }
                }
            }
        }
        if changed {
            self.subscriptions_changed.store(true, Ordering::SeqCst);
        }
        Ok(())
    }
    //
    //
    fn run(&self) -> Result<(), Error> {
        log::info!("{}.run | Starting...", self.dbg);
        let dbg = self.dbg.clone();
        let name = self.name.clone();
        let recv = self.rx_recv.pop().unwrap();
        let subscriptions_ref = self.subscriptions.clone();
        let subscriptions_changed = self.subscriptions_changed.clone();
        // let receiver_dictionary = self.receiver_dictionary.clone();
        for receiver_name in &self.send_queues {
            let send = self.services.get_link(receiver_name).unwrap_or_else(|err| {
                panic!("{}.run | services.get_link error: {:#?}", dbg, err);
            });
            let receiver_hash = PointTxId::from_str(&receiver_name.name());
            self.subscriptions.add_broadcast(receiver_hash, send.clone());
            log::debug!("{}.run | Broadcast subscription registered, receiver: \n\t{} ({})", self.dbg, receiver_name, receiver_hash);
        }
        let exit = self.exit.clone();
        let error = Error::new(&self.dbg, "run");
        let handle: Box<dyn WaitBox<()>> = match &self.schrduler {
            Some(schrduler) => {
                let h = schrduler.spawn(move|| {
                    Self::run_(dbg, name, recv, subscriptions_ref, subscriptions_changed, exit);
                    Ok(())
                }).map_err(|err| error.pass_with("Start failed on Scheduler", err.to_string()))?;
                Box::new(h)
            }
            None => {
                let h= std::thread::Builder::new().name(format!("{}.run", dbg.clone())).spawn(move || {
                    Self::run_(dbg, name, recv, subscriptions_ref, subscriptions_changed, exit);
                }).map_err(|err| error.pass_with("Start failed on std::thread", err.to_string()))?;
                Box::new(h)
            }
        };
        log::info!("{}.run | Started", self.dbg);
        self.handle.push(handle);
        Ok(())
    }
    //
    //
    fn is_finished(&self) -> bool {
        self.is_finished.load(Ordering::SeqCst)
    }
    //
    //
    #[dbg]
    fn wait(&self) -> Result<(), Error> {
        while !self.handle.is_empty() {
            if let Some(handle) = self.handle.pop() {
                if let Err(err) = handle.wait() {
                    dbg::warn!("Error: {:?}", err);
                    return Err(Error::new(&self.dbg, "wait").err(format!("{:?}", err)));
                }
            }
        }
        self.is_finished.store(true, Ordering::SeqCst);
        Ok(())
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}

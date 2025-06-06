use std::{
    collections::HashMap, fmt::Debug, fs, io::Write,
    sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}, Arc, Mutex, RwLock},
    thread::{self, JoinHandle},
};
use coco::Stack;
use concat_string::concat_string;
use sal_core::{dbg::{self, dbg, Dbg}, error::Error};
use crate::services::{
    entity::{Name, Object, Point, PointTxId},
    safe_lock::rwlock::SafeLock, service::{LinkName, Service, RECV_TIMEOUT},
    services::Services, subscription::{SubscriptionCriteria, Subscriptions},
};
use super::multi_queue_conf::MultiQueueConf;
///
/// - Receives points into the MPSC queue in the blocking mode
/// - If new point received, immediately sends it to the all subscribed consumers
/// - Keeps all consumers subscriptions in the single map:
pub struct MultiQueue {
    dbg: Dbg,
    name: Name,
    subscriptions: Arc<RwLock<Subscriptions>>,
    subscriptions_changed: Arc<AtomicBool>,
    rx_send: HashMap<String, Sender<Point>>,
    rx_recv: Mutex<Option<Receiver<Point>>>,
    send_queues: Vec<LinkName>,
    services: Arc<RwLock<Services>>,
    receiver_dictionary: HashMap<usize, String>,
    handle: Stack<JoinHandle<()>>,
    is_finished: Arc<AtomicBool>,
    exit: Arc<AtomicBool>,
}
//
//
impl MultiQueue {
    ///
    /// Creates new instance of [ApiClient]
    /// - [parent] - the ID if the parent entity
    pub fn new(conf: MultiQueueConf, services: Arc<RwLock<Services>>) -> Self {
        let dbg = Dbg::new(conf.name.parent(), conf.name.me());
        let (send, recv) = mpsc::channel();
        let send_queues = conf.send_to;
        Self {
            name: conf.name.clone(),
            subscriptions: Arc::new(RwLock::new(Subscriptions::new(&dbg))),
            subscriptions_changed: Arc::new(AtomicBool::new(false)),
            rx_send: HashMap::from([(conf.rx, send)]),
            rx_recv: Mutex::new(Some(recv)),
            send_queues,
            services,
            receiver_dictionary: HashMap::new(),
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
    fn get_link(&mut self, name: &str) -> Sender<Point> {
        match self.rx_send.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.dbg, name),
        }
    }
    //
    //
    fn subscribe(&mut self, receiver_name: &str, points: &[SubscriptionCriteria]) -> (Sender<Point>, Receiver<Point>) {
        let (send, recv) = mpsc::channel();
        let receiver_hash = PointTxId::from_str(receiver_name);
        self.receiver_dictionary.insert(receiver_hash, receiver_name.to_string());
        if points.is_empty() {
            self.subscriptions.wlock(&self.dbg).add_broadcast(receiver_hash, send.clone());
            self.log("/broadcast.log", receiver_name, receiver_hash, points);
            log::debug!("{}.subscribe | Broadcast subscription registered, receiver: \n\t{} ({})", self.dbg, receiver_name, receiver_hash);
        } else {
            for subscription_criteria in points {
                self.subscriptions.wlock(&self.dbg).add_multicast(receiver_hash, &subscription_criteria.destination(), send.clone());
            }
            self.log("/multicast.log", receiver_name, receiver_hash, points);
            log::debug!("{}.subscribe | Multicast subscription registered, receiver: \n\t{} ({}) \n\tpoints: {:#?}", self.dbg, receiver_name, receiver_hash, points.len());
            log::trace!("{}.subscribe | Multicast subscription registered, receiver: \n\t{} ({}) \n\tpoints: {:#?}", self.dbg, receiver_name, receiver_hash, points);
        }
        self.subscriptions_changed.store(true, Ordering::SeqCst);
        (send, recv)
    }
    //
    //
    fn extend_subscription(&mut self, receiver_name: &str, points: &[SubscriptionCriteria]) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "extend_subscription");
        let receiver_hash = PointTxId::from_str(receiver_name);
        if points.is_empty() {
            let message = format!("{}.extend_subscription | Broadcast subscription can't be extended, receiver: {} ({})", self.dbg, receiver_name, receiver_hash);
            log::warn!("{}", message);
            Err(error.err(message))
        } else {
            let mut message = String::new();
            for subscription_criteria in points {
                log::trace!("{}.extend_subscription | Multicast subscription extending for receiver: {} ({})...", self.dbg, receiver_name, receiver_hash);
                if let Err(err) = self.subscriptions.wlock(&self.dbg).extend_multicast(receiver_hash, &subscription_criteria.destination()) {
                    message = concat_string!(message, err, "\n");
                };
            }
            self.log("/multicast.log", receiver_name, receiver_hash, points);
            if message.is_empty() {
                log::debug!("{}.extend_subscription | Multicast subscription extended, receiver: {} ({})", self.dbg, receiver_name, receiver_hash);
                self.subscriptions_changed.store(true, Ordering::SeqCst);
                Ok(())
            } else {
                log::debug!("{}.extend_subscription | Multicast subscription extended, receiver: {} ({}) \n\t with errors: {:?}", self.dbg, receiver_name, receiver_hash, message);
                self.subscriptions_changed.store(true, Ordering::SeqCst);
                Err(error.err(message))
            }
        }
    }
    //
    //
    fn unsubscribe(&mut self, receiver_name: &str, points: &[SubscriptionCriteria]) -> Result<(), Error> {
        let mut changed = false;
        let error = Error::new(&self.dbg, "unsubscribe");
        let receiver_hash = PointTxId::from_str(receiver_name);
        if points.is_empty() {
            match self.subscriptions.wlock(&self.dbg).remove_all(&receiver_hash) {
                Ok(_) => {
                    self.receiver_dictionary.remove(&receiver_hash);
                    changed |= true;
                    log::debug!("{}.unsubscribe | Broadcast subscription removed, receiver: {} ({})", self.dbg, receiver_name, receiver_hash);
                }
                Err(err) => {
                    return Err(error.pass(err))
                }
            }
        } else {
            for subscription_criteria in points {
                match self.subscriptions.wlock(&self.dbg).remove(&receiver_hash, &subscription_criteria.destination()) {
                    Ok(_) => {
                        self.receiver_dictionary.remove(&receiver_hash);
                        changed |= true;
                        log::debug!("{}.unsubscribe | Multicat subscription '{}' removed, receiver: {} ({})", self.dbg, subscription_criteria.destination(), receiver_name, receiver_hash);
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
    fn run(&mut self) -> Result<(), Error> {
        log::info!("{}.run | Starting...", self.dbg);
        let self_id = self.dbg.clone();
        let self_name = self.name.clone();
        let exit = self.exit.clone();
        let recv = self.rx_recv.lock().unwrap().take().unwrap();
        let subscriptions_ref = self.subscriptions.clone();
        let subscriptions_changed = self.subscriptions_changed.clone();
        // let receiver_dictionary = self.receiver_dictionary.clone();
        for receiver_name in &self.send_queues {
            let send = self.services.rlock(&self_id).get_link(receiver_name).unwrap_or_else(|err| {
                panic!("{}.run | services.get_link error: {:#?}", self_id, err);
            });
            let receiver_hash = PointTxId::from_str(&receiver_name.name());
            self.subscriptions.wlock(&self_id).add_broadcast(receiver_hash, send.clone());
            log::debug!("{}.run | Broadcast subscription registered, receiver: \n\t{} ({})", self.dbg, receiver_name, receiver_hash);
        }
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            log::info!("{}.run | Preparing thread - ok", self_id);
            let mut subscriptions = subscriptions_ref.wlock(&self_id).clone();
            loop {
                if subscriptions_changed.load(Ordering::Relaxed) {
                    subscriptions_changed.store(false, Ordering::SeqCst);
                    log::debug!("{}.run | Subscriptions changes detected", self_id);
                    subscriptions = subscriptions_ref.rlock(&self_id).clone();
                }
                match recv.recv_timeout(RECV_TIMEOUT) {
                    Ok(point) => {
                        let point_id = SubscriptionCriteria::new(&point.name(), point.cot()).destination();
                        log::trace!("{}.run | received: \n\t{:?}", self_id, point);
                        Self::log_point(&self_id, &self_name, &point_id, &point);
                        for (receiver_hash, sender) in subscriptions.iter(&point_id) {
                            if receiver_hash != &point.tx_id() {
                                match sender.send(point.clone()) {
                                    Ok(_) => {
                                        log::trace!("{}.run | sent to '{}' point: {:?}", self_id, receiver_hash, point);
                                    }
                                    Err(err) => {
                                        log::error!("{}.run | subscriptions '{}', receiver '{}' - send error: {:?}", self_id, point_id, receiver_hash, err);
                                    }
                                };
                            }
                        }
                    }
                    Err(err) => {
                        log::trace!("{}.run | recv timeout: {:?}", self_id, err);
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    subscriptions_ref.wlock(&self_id).exit();
                    break;
                }
            }
            log::info!("{}.run | Exit", self_id);
        });
        match handle {
            Ok(handle) => {
                log::info!("{}.run | Started", self.dbg);
                self.handle.push(handle);
                Ok(())
            }
            Err(err) => {
                let err = Error::new(&self.dbg, "run").pass_with("Start failed", err.to_string());
                log::warn!("{}", err);
                Err(err)
            }
        }
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
        if let Some(handle) = self.handle.pop() {
            if let Err(err) = handle.join() {
                dbg::warn!("Error: {:?}", err);
                return Err(Error::new(&self.dbg, "wait").err(format!("{:?}", err)));
            }
            self.is_finished.store(true, Ordering::SeqCst);
        }
        Ok(())
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}

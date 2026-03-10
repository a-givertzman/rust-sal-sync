use std::{
    collections::HashMap, fmt::Debug, fs, hash::BuildHasherDefault, io::Write,
    sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration,
};
use concat_string::concat_string;
use sal_core::{dbg::{self, dbg, Dbg}, error::Error};
use crate::{
    collections::FxDashMap, services::{
        entity::{Name, Object, Point, PointTxId}, future::Sink, service::{LinkName, Service, RECV_TIMEOUT}, services::Services, subscription::{SubscriptionCriteria, Subscriptions}, ServiceWaiting
    },
    sync::{channel::{self, Receiver, Sender}, Handles, Owner}, thread_pool::Scheduler,
};
use super::multi_queue_conf::MultiQueueConf;
///
/// Unique id of the service (TxId) receiving the Point's by the subscription
/// This id used to identify the service produced the Points. 
/// To avoid send back self produced Point's.
type ReceiverId = usize;
///
/// ### Receive and destribute `Point`'s across multiple services
/// - Thread safe
/// - Receives `Point`'s into the MPSC queue in the blocking mode
/// - If new point received, immediately sends it to the all subscribed consumers
/// - Keeps all consumers subscriptions in the single map
pub struct MultiQueue {
    dbg: Dbg,
    name: Name,
    wait_started: Option<Duration>,
    // wait_finished: Option<Duration>,
    subscriptions: Arc<Subscriptions>,
    rx_send: HashMap<String, Sender<Point>>,
    rx_recv: Owner<Receiver<Point>>,
    send_queues: Vec<LinkName>,
    services: Arc<Services>,
    scheduler: Option<Scheduler>,
    receiver_dictionary: FxDashMap<ReceiverId, String>,
    handles: Handles<()>,
    exit: Arc<AtomicBool>,
}
//
//
impl MultiQueue {
    ///
    /// Creates new instance of [ApiClient]
    /// - [parent] - the ID if the parent entity
    pub fn new(conf: MultiQueueConf, services: Arc<Services>, scheduler: Option<Scheduler>) -> Self {
        let dbg = Dbg::new(conf.name.parent(), conf.name.me());
        let (send, recv) = channel::unbounded();
        let send_queues = conf.send_to;
        Self {
            name: conf.name.clone(),
            wait_started: conf.wait_started,
            // wait_finished: conf.wait_finished,
            subscriptions: Arc::new(Subscriptions::new(&dbg)),
            rx_send: HashMap::from([(conf.rx, send)]),
            rx_recv: Owner::new(recv),
            send_queues,
            services,
            scheduler,
            receiver_dictionary: FxDashMap::with_hasher(BuildHasherDefault::default()),
            handles: Handles::new(&dbg),
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
                match f.write_fmt(format_args!("\n\n\t{} ({})", receiver_name, rceiver_hash)) {
                    Ok(_) => {
                        if let Err(err) = serde_json::to_writer_pretty(f, &destinations) {
                            if log::max_level() >= log::LevelFilter::Debug {
                                log::warn!("{}.log | Error writing to file: '{}'\n\terror: {:?}", self.dbg, path, err)
                            }
                        }
                    },
                    Err(err) => {
                        if log::max_level() >= log::LevelFilter::Debug {
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
    fn log_point(dbg: &Dbg, parent: &Name, destination: &str, point: &Point) {
        let path = concat_string!("./logs", parent.join(), "/points.log");
        match fs::OpenOptions::new().create(true).append(true).open(&path) {
            Ok(mut f) => {
                if let Err(err) = f.write_fmt(format_args!("'{}': {:?}\n",destination, point)) {
                    if log::max_level() >= log::LevelFilter::Debug {
                        log::warn!("{}.log | Error write file: '{}'\n\terror: {:?}", dbg, path, err)
                    }
                }
            }
            Err(err) => {
                if log::max_level() >= log::LevelFilter::Debug {
                    log::warn!("{}.log | Error open file: '{}'\n\terror: {:?}", dbg, path, err)
                }
            }
        }
    }
    ///
    /// Main loop
    fn run_(
        dbg: Dbg,
        name: Name,
        recv: Receiver<Point>,
        subscriptions: Arc<Subscriptions>,
        started: Option<Sink<Result<(), Error>>>,
        exit: Arc<AtomicBool>,
    ) {
        log::info!("{}.run | Preparing thread - ok", dbg);
        started.map(|started| started.add(Ok(())));
        loop {
            match recv.recv_timeout(RECV_TIMEOUT) {
                Ok(point) => {
                    let destination = point.dest();    // SubscriptionCriteria::new(&point.name(), point.cot()).destination();
                    log::trace!("{}.run | received: \n\t{:?}", dbg, point);
                    if log::max_level() >= log::Level::Debug {
                        Self::log_point(&dbg, &name, &destination, &point);
                    }
                    for (receiver_id, sender) in subscriptions.get_view(&destination).iter() {
                        if *receiver_id != point.txid() {
                            match sender.send(point.clone()) {
                                Ok(_) => {
                                    log::trace!("{}.run | sent to '{}' point: {:?}", dbg, receiver_id, point);
                                }
                                Err(err) => {
                                    log::error!("{}.run | subscriptions '{}', receiver '{}' - send error: {:?}", dbg, destination, receiver_id, err);
                                }
                            };
                        }
                    }
                }
                Err(err) => {
                    match err {
                        kanal::ReceiveErrorTimeout::Timeout => {},
                        _ => {
                            log::trace!("{}.run | recv error: {:?}", dbg, err);
                            break;
                        }
                    }
                }
            }
            if exit.load(Ordering::Acquire) {
                subscriptions.exit();
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
        let receiver_id = PointTxId::from_str(receiver_name);
        self.receiver_dictionary.insert(receiver_id, receiver_name.to_string());
        if points.is_empty() {
            self.subscriptions.add_broadcast(receiver_id, send.clone());
            self.log("broadcast.log", receiver_name, receiver_id, points);
            dbg::debug!("Broadcast registered, receiver: \n\t{} ({})", receiver_name, receiver_id);
        } else {
            for subscription_criteria in points {
                self.subscriptions.add_multicast(receiver_id, &subscription_criteria.destination(), send.clone());
            }
            self.log("multicast.log", receiver_name, receiver_id, points);
            dbg::debug!("Multicast registered, receiver: \n\t{} ({}) \n\tpoints: {:#?}", receiver_name, receiver_id, points.len());
            dbg::trace!("Multicast registered, receiver: \n\t{} ({}) \n\tpoints: {:#?}", receiver_name, receiver_id, points);
        }
        (send, recv)
    }
    //
    //
    #[dbg]
    fn extend_subscription(&self, receiver_name: &str, points: &[SubscriptionCriteria]) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "extend_subscription");
        let receiver_id = PointTxId::from_str(receiver_name);
        if points.is_empty() {
            Err(error.err(format!("Can't be extended (broadcast), receiver: {} ({})", receiver_name, receiver_id)))
        } else {
            let mut message = String::new();
            dbg::debug!("Extending (multicast) for receiver: {} ({})...", receiver_name, receiver_id);
            for subscription_criteria in points {
                if let Err(err) = self.subscriptions.extend_multicast(receiver_id, &subscription_criteria.destination()) {
                    message = concat_string!(message, err.to_string(), "\n");
                };
            }
            // self.log("/multicast.log", receiver_name, receiver_id, points);
            if message.is_empty() {
                dbg::debug!("Extended (multicast), receiver: {} ({})", receiver_name, receiver_id);
                Ok(())
            } else {
                dbg::debug!("Extended (multicast), receiver: {} ({}) \n\t with errors: {:?}", receiver_name, receiver_id, message);
                Err(error.err(message))
            }
        }
    }
    //
    //
    #[dbg]
    fn unsubscribe(&self, receiver_name: &str, points: &[SubscriptionCriteria]) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "unsubscribe");
        let receiver_id = PointTxId::from_str(receiver_name);
        if points.is_empty() {
            self.subscriptions.remove_all(receiver_id);
            self.receiver_dictionary.remove(&receiver_id);
            dbg::debug!("Broadcast subscription removed, receiver: {} ({})", receiver_name, receiver_id);
            Ok(())
        } else {
            let destinations: Vec<String> = points.into_iter().map(|p| p.destination()).collect();
            self.subscriptions.remove(receiver_id, &destinations);
            for s in destinations {
                dbg::debug!("Multicat subscription '{s}' removed, receiver: {receiver_name} ({receiver_id})");
            }
            if !self.subscriptions.is_subscribed(receiver_id) {
                self.receiver_dictionary.remove(&receiver_id);
            }
            Ok(())
        }
    }
    //
    //
    fn run(&self) -> Result<(), Error> {
        log::info!("{}.run | Starting...", self.dbg);
        let dbg = self.dbg.clone();
        let name = self.name.clone();
        let recv = self.rx_recv.take().ok_or(Error::new(&name, "run").err("Can't get required 'self.rx_recv'"))?;
        let subscriptions = self.subscriptions.clone();
        // let receiver_dictionary = self.receiver_dictionary.clone();
        for receiver_name in &self.send_queues {
            let send = self.services.get_link(receiver_name).unwrap_or_else(|err| {
                panic!("{}.run | services.get_link error: {:#?}", dbg, err);
            });
            let receiver_hash = PointTxId::from_str(&receiver_name.name());
            self.subscriptions.add_broadcast(receiver_hash, send.clone());
            log::debug!("{}.run | Broadcast subscription registered, receiver: \n\t{} ({})", self.dbg, receiver_name, receiver_hash);
        }
        let service_waiting = ServiceWaiting::new(&name, self.wait_started);
        let service_release = self.wait_started.map(|_| service_waiting.release());
        let exit = self.exit.clone();
        let error = Error::new(&self.dbg, "run");
        match &self.scheduler {
            Some(scheduler) => {
                let handle = scheduler.spawn(move|| {
                    Self::run_(dbg, name, recv, subscriptions, service_release, exit);
                    Ok(())
                }).map_err(|err| error.pass_with("Start failed on Scheduler", err.to_string()))?;
                self.handles.push(handle);
            }
            None => {
                let handle= std::thread::Builder::new().name(format!("{}.run", dbg.clone())).spawn(move || {
                    Self::run_(dbg, name, recv, subscriptions, service_release, exit);
                }).map_err(|err| error.pass_with("Start failed on std::thread", err.to_string()))?;
                self.handles.push(handle);
            }
        };
        let r = match self.wait_started {
            Some(_) => service_waiting.wait(),
            None => Ok(()),
        };
        log::info!("{}.run | Started", self.dbg);
        r
    }
    //
    //
    fn is_finished(&self) -> bool {
        self.handles.is_finished()
    }
    //
    //
    fn wait(&self) -> Result<(), Error> {
        self.handles.wait()
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::Release);
    }
}

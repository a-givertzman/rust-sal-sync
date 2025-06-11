use std::{collections::HashMap, fmt::Debug, str::FromStr, sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc}, thread::{self, JoinHandle}};
use coco::Stack;
use log::{info, warn, error, trace};
use sal_core::{dbg::{self, dbg}, error::Error};
use crate::services::{entity::{Name, Object, Point, PointTxId}, types::Mutex, LinkName, Service, Services, SubscriptionCriteria, Subscriptions};
///
/// - Receives points into the MPSC queue in the blocking mode
/// - If new point received, immediately sends it to the all subscribed consumers
/// - Keeps all consumers subscriptions in the single map:
pub struct MockMultiQueue {
    dbg: String,
    name: Name,
    subscriptions: Arc<Subscriptions>,
    rx_send: HashMap<String, Sender<Point>>,
    rx_recv: Mutex<Option<Receiver<Point>>>,
    send_queues: Vec<String>,
    services: Arc<Services>,
    handle: Stack<JoinHandle<()>>,
    is_finished: Arc<AtomicBool>,
    exit: Arc<AtomicBool>,
}
//
// 
impl MockMultiQueue {
    ///
    /// Creates new instance of [ApiClient]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, tx_queues: Vec<String>, rx_queue: impl Into<String>, services: Arc<Services>) -> Self {
        let name = Name::new(parent, format!("MockMultiQueue{}", COUNT.fetch_add(1, Ordering::Relaxed)));
        let (send, recv) = mpsc::channel();
        Self {
            dbg: name.join(),
            name: name.clone(),
            subscriptions: Arc::new(Subscriptions::new(name)),
            rx_send: HashMap::from([(rx_queue.into(), send)]),
            rx_recv: Mutex::new(Some(recv)),
            send_queues: tx_queues,
            services,
            handle: Stack::new(),
            is_finished: Arc::new(AtomicBool::new(false)),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
//
// 
impl Object for MockMultiQueue {
    fn name(&self) -> Name {
        self.name.clone()
    }
}
//
// 
impl Debug for MockMultiQueue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("MockMultiQueue")
            .field("id", &self.dbg)
            .finish()
    }
}
//
//
impl Service for MockMultiQueue {
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
    fn subscribe(&self, receiver_id: &str, points: &[SubscriptionCriteria]) -> (Sender<Point>, Receiver<Point>) {
        let (send, recv) = mpsc::channel();
        let receiver_id = PointTxId::from_str(receiver_id);
        if points.is_empty() {
            self.subscriptions.add_broadcast(receiver_id, send.clone());
        } else {
            for subscription_criteria in points {
                self.subscriptions.add_multicast(receiver_id, &subscription_criteria.destination(), send.clone());
            }
        }
        (send, recv)
    }
    //
    //
    fn unsubscribe(&self, receiver_id: &str, points: &[SubscriptionCriteria]) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "unsubscribe");
        let receiver_id = PointTxId::from_str(receiver_id);
        for subscription_criteria in points {
            match self.subscriptions.remove(&receiver_id, &subscription_criteria.destination()) {
                Ok(_) => {}
                Err(err) => {
                    return Err(error.pass(err))
                }
            }
        }
        Ok(())
    }
    //
    //
    fn run(&self) -> Result<(), Error> {
        info!("{}.run | Starting...", self.dbg);
        let dbg = self.dbg.clone();
        let exit = self.exit.clone();
        let recv = self.rx_recv.lock().take().unwrap();
        let subscriptions = self.subscriptions.clone();
        let mut static_subscriptions: HashMap<usize, Sender<Point>> = HashMap::new();
        for send_queue in &self.send_queues {
            let tx_send = self.services.get_link(&LinkName::from_str(send_queue).unwrap()).unwrap_or_else(|err| {
                panic!("{}.run | services.get_link error: {:#?}", self.dbg, err);
            });
            static_subscriptions.insert(PointTxId::from_str(send_queue), tx_send);
        }
        let handle = thread::Builder::new().name(format!("{}.run", dbg.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", dbg);
            loop {
                match recv.recv() {
                    Ok(point) => {
                        let point_id = point.name();
                        trace!("{}.run | received: {:?}", dbg, point);
                        for (receiver_id, sender) in subscriptions.get(&point_id).chain(&static_subscriptions) {
                            match sender.send(point.clone()) {
                                Ok(_) => {}
                                Err(err) => {
                                    error!("{}.run | subscriptions '{}', receiver '{}' - send error: {:?}", dbg, point_id, receiver_id, err);
                                }
                            };
                        }
                    }
                    Err(err) => {
                        warn!("{}.run | recv error: {:?}", dbg, err);
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
        });
        match handle {
            Ok(handle) => {
                info!("{}.run | Starting - ok", self.dbg);
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
///
/// Global static counter of FnOut instances
static COUNT: AtomicUsize = AtomicUsize::new(0);

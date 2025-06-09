use crate::{
    kernel::state::ChangeNotify,
    services::{
        conf::ServicesConf,
        entity::{Name, Object, Point, PointConfig},
        future::{Future, Sink}, retain::{RetainConf, RetainPointId},
        service::{LinkName, Service, ServiceCycle},
        subscription::SubscriptionCriteria,
    },
};
use std::{
    collections::HashMap, fmt::Debug,
    sync::{atomic::{AtomicBool, Ordering}, mpsc::{Receiver, Sender}, Arc},
    thread::{self, JoinHandle}, time::Duration,
};
use coco::Stack;
use concat_string::concat_string;
use dashmap::DashMap;
use sal_core::{dbg::Dbg, error::Error};
///
/// Holds a map of the all services in app by there names
pub struct Services {
    dbg: Dbg,
    name: Name,
    map: Arc<DashMap<String, Arc<dyn Service>>>,
    conf: ServicesConf,
    retain_point_id: Option<Arc<RetainPointId>>,
    points_request: Arc<Stack<(String, Sink<Vec<PointConfig>>)>>,
    handle: Stack<JoinHandle<()>>,
    exit: Arc<AtomicBool>,
}
//
//
impl Services {
    pub const API_CLIENT: &'static str = "ApiClient";
    pub const MULTI_QUEUE: &'static str = "MultiQueue";
    pub const PROFINET_CLIENT: &'static str = "ProfinetClient";
    pub const TASK: &'static str = "Task";
    pub const TCP_CLIENT: &'static str = "TcpClient";
    pub const TCP_SERVER: &'static str = "TcpServer";
    pub const PRODUCER_SERVICE: &'static str = "ProducerService";
    pub const CACHE_SERVICE: &'static str = "CacheService";
    pub const SLMP_CLIENT: &'static str = "SlmpClient";
    ///
    /// Creates new instance of the Services
    pub fn new(parent: impl Into<String>, conf: ServicesConf) -> Self {
        let parent = parent.into();
        let name = Name::new(&parent, "Services");
        let name_str = name.join();
        Self {
            dbg: Dbg::new(parent, "Services"),
            name,
            map: Arc::new(DashMap::new()),
            retain_point_id: match &conf.retain.point {
                Some(_) => Some(Arc::new(RetainPointId::new(&name_str, conf.retain.clone()))),
                None => None,
            },
            conf: conf,
            points_request: Arc::new(Stack::new()),
            handle: Stack::new(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Prepairing retained points id's
    fn prepare_point_ids(dbg: &Dbg, notify: &mut ChangeNotify<NotifyState, String>, retain_point_id: &Option<Arc<RetainPointId>>, services: &Arc<DashMap<String, Arc<dyn Service>>>) {
        match retain_point_id {
            Some(retain_point_id) => {
                log::info!("{}.prepare_point_ids | Preparing retained Point's id's...", dbg);
                for (service_id, service) in services.iter().map(|r| (r.key().clone(), r.value().clone())) {
                    let service_points = service.points();
                    retain_point_id.insert(&service_id, service_points);
                };
                log::info!("{}.prepare_point_ids | Point's is chashed: {}", dbg, retain_point_id.is_cached());
                let points = retain_point_id
                    .points()
                    .iter()
                    .map(|(owner, p)| {
                        p.iter().map(|p| {
                            concat_string!(owner, " | ", p.id.to_string(), " | ", p.type_.to_string(), " | ", p.name, "\n")
                        }).collect()
                    }).collect::<Vec<String>>();
                log::trace!("{}.prepare_point_ids | Point's: {:#?}", dbg, points);
                log::info!("{}.prepare_point_ids | Preparing retained Point's id's - ok", dbg);
            }
            None => notify.add(NotifyState::RetainPointNotConfiguredWarn, format!("{}.run | Retain->Point - not configured", dbg)),
        }
    }
    ///
    /// Main loop of the Services
    pub fn run(&self) -> Result<(), Error> {
        log::info!("{}.run | Starting...", self.dbg);
        let dbg = self.dbg.clone();
        let name = self.name.clone();
        let points_request = self.points_request.clone();
        let retain_point_id = self.retain_point_id.clone();
        let services = self.map.clone();
        let exit = self.exit.clone();
        log::info!("{}.run | Preparing thread...", dbg);
        let handle = thread::Builder::new().name(format!("{}.run", dbg)).spawn(move || {
            log::info!("{}.run | Preparing thread - ok", dbg);
            let mut notify = ChangeNotify::new(
                &dbg,
                NotifyState::Start,
                vec![
                    (NotifyState::Start,  Box::new(|message| log::info!("{}", message))),
                    (NotifyState::Info,   Box::new(|message| log::info!("{}", message))),
                    (NotifyState::Warn,   Box::new(|message| log::warn!("{}", message))),
                    (NotifyState::RetainPointNotConfiguredWarn,   Box::new(|message| log::warn!("{}", message))),
                    (NotifyState::Error,  Box::new(|message| log::error!("{}", message))),
                    (NotifyState::PointsRequestsAccessError,  Box::new(|message| log::error!("{}", message))),
                    (NotifyState::PointsRequestsIsEmpty,  Box::new(|message| log::error!("{}", message))),
                ],
            );
            Self::prepare_point_ids(&dbg, &mut notify, &retain_point_id, &services);
            let mut cycle = ServiceCycle::new(&name.join(), Duration::from_millis(10));
            loop {
                cycle.start();
                if !points_request.is_empty() {
                    match points_request.pop() {
                        Some((requester_name, sink)) => {
                            log::debug!("{}.run | Points requested from: '{}'", dbg, requester_name);
                            match &retain_point_id {
                                Some(retain_point_id) => {
                                    let points = retain_point_id.points()
                                    .into_iter().filter_map(|(owner, points)| {
                                        if *owner != requester_name {
                                            Some(points)
                                        } else {
                                            None
                                        }
                                    }).flatten().collect();
                                    sink.add(points);
                                    log::debug!("{}.run | Points requested from: '{}' - Ok", dbg, requester_name);
                                }
                                None => {
                                    notify.add(NotifyState::RetainPointNotConfiguredWarn, format!("{}.run | Retain->Point - not configured", dbg));
                                    sink.add(vec![]);
                                }
                            }
                        }
                        None => notify.add(NotifyState::PointsRequestsIsEmpty, format!("{}.run | Points requests is empty", dbg)),
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
                cycle.wait();
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
            log::info!("{}.run | Exit", dbg);
        });
        thread::sleep(Duration::from_millis(50));
        match handle {
            Ok(handle) => {
                log::info!("{}.run | Starting - ok", self.dbg);
                self.handle.push(handle);
                Ok(())
            }
            Err(err) => {
                let message = format!("{}.run | Start failed: {:#?}", self.dbg, err);
                log::warn!("{}", message);
                Err(Error::new(&self.dbg, "run").err(message))
            }
        }

    }
    ///
    /// Returns all holding services in the map<service id, service reference>
    pub fn all(&self) -> HashMap<String, Arc<dyn Service>> {
        HashMap::from_iter(
            self.map.iter().map(|r| (r.key().clone(), r.value().clone()))
        )
    }
    ///
    /// Inserts a new service into the collection
    pub fn insert(&self, service: Arc<dyn Service>) {
        let name = service.name().join();
        if self.map.contains_key(&name) {
            panic!("{}.insert | Duplicated service name '{:?}'", self.dbg, name);
        }
        self.map.insert(name, service);
    }
    ///
    /// Returns Service
    pub fn get(&self, name: &str) -> Option<Arc<dyn Service>> {
        match self.map.get(name) {
            Some(r) => Some(r.value().clone()),
            None => {
                log::warn!("{}.get | service '{:?}' - not found", self.dbg, name);
                None
            },
        }
    }
    ///
    /// Returns copy of the Sender - service's incoming queue by service link name (Service.link)
    pub fn get_link(&self, name: &LinkName) -> Result<Sender<Point>, Error> {
        let (service, queue) = name.split();
        match self.get(&service) {
            Some(srvc) => Ok(srvc.get_link(&queue)),
            None => Err(Error::new(&self.dbg, "get_link").err(format!("service '{:?}' - not found", name))),
        }
    }
    ///
    /// Returns Receiver
    /// - service - the name of the service to subscribe on
    pub fn subscribe(&self, service: &str, receiver_name: &str, points: &[SubscriptionCriteria]) -> (Sender<Point>, Receiver<Point>) {
        match self.get(service) {
            Some(srvc) => {
                let r = srvc.subscribe(receiver_name, points);
                r
            }
            None => panic!("{}.subscribe | service '{:?}' - not found", self.dbg, service),
        }
    }
    ///
    /// Returns ok if subscription extended sucessfully
    /// - service - the name of the service to extend subscribtion on
    pub fn extend_subscription(&self, service: &str, receiver_name: &str, points: &[SubscriptionCriteria]) -> Result<(), Error> {
        // panic!("{}.extend_subscription | Not implemented yet", self.id);
        match self.get(service) {
            Some(srvc) => {
                let r = srvc.extend_subscription(receiver_name, points);
                r
            }
            None => panic!("{}.extend_suscription | service '{:?}' - not found", self.dbg, service),
        }
    }
    ///
    /// Returns ok if subscription removed sucessfully
    /// - service - the name of the service to unsubscribe on
    pub fn unsubscribe(&self, service: &str, receiver_name: &str, points: &[SubscriptionCriteria]) -> Result<(), Error> {
        match self.get(service) {
            Some(srvc) => {
                let r = srvc.unsubscribe(receiver_name, points);
                r
            }
            None => panic!("{}.unsubscribe | service '{:?}' - not found", self.dbg, service),
        }
    }
    ///
    /// Returns list of point configurations over the all services
    ///  - requester_name - Service name !!!
    pub fn points(&self, requester_name: impl Into<String>) -> Future<Vec<PointConfig>> {
        let (future, sink) = Future::new();
        self.points_request.push((requester_name.into(), sink));
        future
    }
    ///
    /// Sends the General Interogation request to all services
    pub fn gi(&self, _service: &str, _points: &[SubscriptionCriteria]) -> Receiver<Point> {
        panic!("{}.gi | Not implemented yet", self.dbg);
    }
    ///
    /// Returns Retain configuration
    pub fn retain(&self) -> RetainConf {
        self.conf.retain.clone()
    }
    ///
    /// Returns [Ok] when all [Service]'s are finished
    pub fn wait(&self) -> Result<(), Error> {
        if let Some(handle) = self.handle.pop() {
            if let Err(err) = handle.join() {
                log::warn!("{}.wait | Error: {:?}", self.dbg, err);
            }
        }
        Ok(())
    }
    ///
    /// 
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
//
//
impl Object for Services {
    fn name(&self) -> Name {
        self.name.clone()
    }
}
//
// 
impl Debug for Services {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Services")
            .field("id", &self.dbg)
            .finish()
    }
}
///
/// States of the Services behavior for logging
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum NotifyState {
    Start,
    Info,
    Warn,
    RetainPointNotConfiguredWarn,
    Error,
    PointsRequestsIsEmpty,
    PointsRequestsAccessError,
}

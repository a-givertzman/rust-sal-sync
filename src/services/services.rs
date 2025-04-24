use crate::{
    kernel::state::ChangeNotify,
    services::{
        conf::ServicesConf,
        entity::{Name, Object, Point, PointConfig},
        future::{Future, Sink}, retain::{RetainConf, RetainPointId},
        safe_lock::rwlock::SafeLock,
        service::{link_name::LinkName, service::Service, service_cycle::ServiceCycle},
        subscription::SubscriptionCriteria,
    },
};
use std::{
    collections::HashMap, fmt::Debug,
    sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, mpsc::{Receiver, Sender}, Arc, RwLock},
    thread::{self, JoinHandle}, time::Duration,
};
use coco::Stack;
use concat_string::concat_string;
use sal_core::{dbg::Dbg, error::Error};
///
/// Holds a map of the all services in app by there names
pub struct Services {
    dbg: Dbg,
    name: Name,
    map: Arc<RwLock<HashMap<String, Arc<RwLock<dyn Service>>>>>,
    conf: ServicesConf,
    retain_point_id: Option<Arc<RwLock<RetainPointId>>>,
    points_requested: Arc<AtomicUsize>,
    points_request: Arc<RwLock<Vec< (String, Sink<Vec<PointConfig>>) >>>,
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
            map: Arc::new(RwLock::new(HashMap::new())),
            retain_point_id: match &conf.retain.point {
                Some(_) => Some(Arc::new(RwLock::new(RetainPointId::new(&name_str, conf.retain.clone())))),
                None => None,
            },
            conf: conf,
            points_requested: Arc::new(AtomicUsize::new(0)),
            points_request: Arc::new(RwLock::new(vec![])),
            handle: Stack::new(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Prepairing retained points id's
    fn prepare_point_ids(dbg: &Dbg, notify: &mut ChangeNotify<NotifyState, String>, retain_point_id: &Option<Arc<RwLock<RetainPointId>>>, services: &Arc<RwLock<HashMap<String, Arc<RwLock<dyn Service>>>>>) {
        match retain_point_id {
            Some(retain_point_id) => {
                log::info!("{}.prepare_point_ids | Preparing retained Point's id's...", dbg);
                match services.read() {
                    Ok(services) => {
                        for (service_id, service) in services.iter() {
                            let service_points = service.rlock(dbg).points();
                            match retain_point_id.write() {
                                Ok(mut retain_point_id) => {
                                    retain_point_id.insert(&service_id, service_points);
                                }
                                Err(err) => log::error!("{}.prepare_point_ids | Points id's write access error: {:#?}", dbg, err),
                            }
                        };
                        log::info!("{}.prepare_point_ids | Point's is chashed: {}", dbg, retain_point_id.read().unwrap().is_cached());
                        let points = retain_point_id.write().unwrap()
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
                    Err(err) => log::error!("{}.prepare_point_ids | Services read access error: {:#?}", dbg, err),
                }
            }
            None => notify.add(NotifyState::RetainPointNotConfiguredWarn, format!("{}.run | Retain->Point - not configured", dbg)),
        }
    }
    ///
    /// Main loop of the Services
    pub fn run(&mut self) -> Result<(), Error> {
        log::info!("{}.run | Starting...", self.dbg);
        let dbg = self.dbg.clone();
        let name = self.name.clone();
        let points_requested = self.points_requested.clone();
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
                if points_requested.load(Ordering::SeqCst) > 0 {
                    match points_request.write() {
                        Ok(mut requests) => {
                            match requests.pop() {
                                Some((requester_name, sink)) => {
                                    log::debug!("{}.run | Points requested from: '{}'", dbg, requester_name);
                                    points_requested.fetch_sub(1, Ordering::SeqCst);
                                    match &retain_point_id {
                                        Some(retain_point_id) => match retain_point_id.write() {
                                            Ok(mut retain_point_id) => {
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
                                            Err(err) => {
                                                log::error!("{}.run | Points id's write access error, requester: '{}', error: {:#?}", dbg, requester_name, err);
                                                sink.add(vec![]);
                                            }
                                        },
                                        None => {
                                            notify.add(NotifyState::RetainPointNotConfiguredWarn, format!("{}.run | Retain->Point - not configured", dbg));
                                            sink.add(vec![]);
                                        }
                                    }
                                }
                                None => notify.add(NotifyState::PointsRequestsIsEmpty, format!("{}.run | Points requests is empty", dbg)),
                            }
                        }
                        Err(err) => {
                            notify.add(NotifyState::PointsRequestsAccessError, format!("{}.run | Points requests access error: {:#?}", dbg, err));
                        }
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
    pub fn all(&self) -> HashMap<String, Arc<RwLock<dyn Service>>> {
        let mut map = HashMap::new();
        match self.map.read() {
            Ok(services) => services.clone_into(&mut map),
            Err(err) => log::error!("{}.all | Services read access error: {:#?}", self.dbg, err),
        }
        map
    }
    ///
    /// Inserts a new service into the collection
    pub fn insert(&mut self, service: Arc<RwLock<dyn Service>>) {
        let name = service.rlock(&self.dbg).name().join();
        match self.map.write() {
            Ok(mut services) => {
                if services.contains_key(&name) {
                    panic!("{}.insert | Duplicated service name '{:?}'", self.dbg, name);
                }
                services.insert(name, service);
            }
            Err(err) => log::error!("{}.insert | Services write access error: {:#?}", self.dbg, err),
        }
    }
    ///
    /// Returns Service
    pub fn get(&self, name: &str) -> Option<Arc<RwLock<dyn Service>>> {
        match self.map.read() {
            Ok(services) => {
                match services.get(name) {
                    Some(srvc) => Some(srvc.clone()),
                    None => {
                        log::warn!("{}.get | service '{:?}' - not found", self.dbg, name);
                        None
                    },
                }
            }
            Err(err) => {
                log::error!("{}.get | Services read access error: {:#?}", self.dbg, err);
                None
            }
        }
    }
    ///
    /// Returns copy of the Sender - service's incoming queue by service link name (Service.link)
    pub fn get_link(&self, name: &LinkName) -> Result<Sender<Point>, Error> {
        let (service, queue) = name.split();
        match self.get(&service) {
            Some(srvc) => Ok(srvc.wlock(&self.dbg).get_link(&queue)),
            None => Err(Error::new(&self.dbg, "get_link").err(format!("service '{:?}' - not found", name))),
        }
    }
    ///
    /// Returns Receiver
    /// - service - the name of the service to subscribe on
    pub fn subscribe(&mut self, service: &str, receiver_name: &str, points: &[SubscriptionCriteria]) -> (Sender<Point>, Receiver<Point>) {
        match self.get(service) {
            Some(srvc) => {
                let r = srvc.wlock(&self.dbg).subscribe(receiver_name, points);
                r
            }
            None => panic!("{}.subscribe | service '{:?}' - not found", self.dbg, service),
        }
    }
    ///
    /// Returns ok if subscription extended sucessfully
    /// - service - the name of the service to extend subscribtion on
    pub fn extend_subscription(&mut self, service: &str, receiver_name: &str, points: &[SubscriptionCriteria]) -> Result<(), Error> {
        // panic!("{}.extend_subscription | Not implemented yet", self.id);
        match self.get(service) {
            Some(srvc) => {
                let r = srvc.wlock(&self.dbg).extend_subscription(receiver_name, points);
                r
            }
            None => panic!("{}.extend_suscription | service '{:?}' - not found", self.dbg, service),
        }
    }
    ///
    /// Returns ok if subscription removed sucessfully
    /// - service - the name of the service to unsubscribe on
    pub fn unsubscribe(&mut self, service: &str, receiver_name: &str, points: &[SubscriptionCriteria]) -> Result<(), Error> {
        match self.get(service) {
            Some(srvc) => {
                let r = srvc.wlock(&self.dbg).unsubscribe(receiver_name, points);
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
        match self.points_request.write() {
            Ok(mut points_request) => {
                points_request.push((requester_name.into(), sink));
                self.points_requested.fetch_add(1, Ordering::SeqCst);
            }
            Err(err) => log::error!("{}.get | Services read access error: {:#?}", self.dbg, err),
        }
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
    /// Returns [Future] to wait for [Service] will finished
    pub fn wait(&self) -> Future<()> {
        let (future, sink) = Future::new();
        let dbg = self.dbg.clone();
        if let Some(handle) = self.handle.pop() {
            std::thread::spawn(move|| {
                if let Err(err) = handle.join() {
                    log::warn!("{}.wait | Error: {:?}", dbg, err);
                }
                sink.add(());
            });
        }
        future
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

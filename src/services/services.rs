use crate::{
    kernel::state::ChangeNotify, services::{
        conf::ServicesConf,
        entity::{Name, Object, Point, PointConf},
        future::{Future, Sink}, RegistryConf, PointRegistry,
        service::{LinkName, Service, ServiceCycle},
        subscription::SubscriptionCriteria,
    }, sync::{channel::{Receiver, Sender}, Handles, Owner}, thread_pool::Scheduler
};
use std::{
    fmt::Debug, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration
};
use concat_string::concat_string;
use crossbeam_skiplist::SkipSet;
use dashmap::DashMap;
use sal_core::{dbg::Dbg, error::Error};
///
/// Holds a map of the all services in app by there names
pub struct Services {
    dbg: Dbg,
    name: Name,
    map: Arc<DashMap<String, Arc<dyn Service>>>,
    order: SkipSet<String>,
    conf: ServicesConf,
    retain_point_id: Option<Arc<PointRegistry>>,
    points_request: Arc<Owner<(String, Sink<Vec<PointConf>>)>>,
    scheduler: Option<Scheduler>,
    handles: Handles<()>,
    exit: Arc<AtomicBool>,
}
//
//
impl Services {
    ///
    /// Creates new instance of the Services
    pub fn new(parent: impl Into<String>, conf: ServicesConf, scheduler: Option<Scheduler>) -> Self {
        let parent = parent.into();
        let name = Name::new(&parent, "Services");
        let name_str = name.join();
        let dbg = Dbg::new(parent, "Services");
        Self {
            name,
            map: Arc::new(DashMap::new()),
            order: SkipSet::new(),
            retain_point_id: match &conf.retain.point {
                Some(_) => Some(Arc::new(PointRegistry::new(&name_str, conf.retain.clone(), scheduler.clone()))),
                None => None,
            },
            conf: conf,
            points_request: Arc::new(Owner::empty()),
            scheduler,
            handles: Handles::new(&dbg),
            dbg,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Prepairing retained points id's
    fn prepare_point_ids(dbg: &Dbg, notify: &mut ChangeNotify<NotifyState, String>, retain_point_id: &Option<Arc<PointRegistry>>, services: &Arc<DashMap<String, Arc<dyn Service>>>) {
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
        match &self.scheduler {
            Some(scheduler) => {
                log::debug!("{}.run | Starting scheduler::thread...", dbg);
                let handle = scheduler.spawn(move || {
                    Self::run_(dbg, name, points_request, retain_point_id, services, exit);
                    Ok(())
                })?;
                self.handles.push(handle);
            }
            None => {
                log::debug!("{}.run | Starting std::thread...", dbg);
                let handle = std::thread::Builder::new().name(format!("{}.run", dbg)).spawn(move || {
                    Self::run_(dbg, name, points_request, retain_point_id, services, exit);
                }).map_err(|err| Error::new(&self.dbg, "run").err(err.to_string()))?;
                self.handles.push(handle);
            }
        };
        std::thread::sleep(Duration::from_millis(50));
        log::info!("{}.run | Starting - ok", self.dbg);
        Ok(())
    }
    ///
    /// Main loop
    fn run_(
        dbg: Dbg,
        name: Name,
        points_request: Arc<Owner<(String, Sink<Vec<PointConf>>)>>,
        retain_point_id: Option<Arc<PointRegistry>>,
        services: Arc<DashMap<String, Arc<dyn Service + 'static>>>,
        exit: Arc<AtomicBool>,
    ) {
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
                match points_request.take() {
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
    }
    ///
    /// Returns all holding `Service`s in the `Vec<(Service id, Service ref)>`, ordered by insertion
    pub fn all(&self) -> Vec<(String, Arc<dyn Service>)> {
        let mut result = vec![];
        for key in self.order.iter() {
            match self.map.get(key.value()) {
                Some(r) => {
                    result.push((r.key().clone(), r.value().clone()))
                },
                None => log::warn!("{}.all | Service '{}' - is not found", self.dbg, key.value()),
            }
        }
        // HashMap::from_iter(
        //     self.map.iter().map(|r| (r.key().clone(), r.value().clone()))
        // )
        result
    }
    ///
    /// Inserts a new service into the collection
    pub fn insert(&self, service: Arc<dyn Service>) {
        let name = service.name().join();
        log::debug!("{}.insert | Inserting Service '{name}' ...", self.dbg);
        if self.map.contains_key(&name) {
            panic!("{}.insert | Duplicated service name '{name}'", self.dbg);
        }
        self.map.insert(name.clone(), service);
        self.order.insert(name.clone());
        log::debug!("{}.insert | Inserting Service '{name}' - Ok", self.dbg);
    }
    ///
    /// Returns Service
    pub fn get(&self, name: &str) -> Option<Arc<dyn Service>> {
        log::debug!("{}.get | Get Service '{name}' ...", self.dbg);
        match self.map.get(name) {
            Some(r) => {
                log::debug!("{}.get | Get Service '{name}' - Ok", self.dbg);
                Some(r.value().clone())
            }
            None => {
                log::warn!("{}.get | Get Service '{name}' - not found", self.dbg);
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
    pub fn points(&self, requester_name: impl Into<String>) -> Future<Vec<PointConf>> {
        let (future, sink) = Future::new();
        self.points_request.replace((requester_name.into(), sink));
        future
    }
    ///
    /// Sends the General Interogation request to all services
    pub fn gi(&self, _service: &str, _points: &[SubscriptionCriteria]) -> Future<Vec<Point>> {
        panic!("{}.gi | Not implemented yet", self.dbg);
    }
    ///
    /// Returns Retain configuration
    pub fn retain(&self) -> RegistryConf {
        self.conf.retain.clone()
    }
    ///
    /// Returns [Ok] when all [Service]'s are finished
    pub fn wait(&self) -> Result<(), Error> {
        self.handles.wait()
    }
    ///
    /// Checks if finished running.
    /// 
    /// To finish call exit
    /// 
    /// **Importent! Does not mean all sevices being finished**
    pub fn is_finished(&self) -> bool {
        self.handles.is_finished()
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

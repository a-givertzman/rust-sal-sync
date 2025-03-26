use log::{info, warn, trace};
use std::{collections::HashMap, fmt::Debug, str::FromStr, sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, mpsc::{self, Receiver, Sender}, Arc, Mutex, RwLock}, thread};
use testing::entities::test_value::Value;
use crate::services::{
    entity::{name::Name, object::Object, point::{point::{Point, ToPoint}, point_tx_id::PointTxId}},
    safe_lock::rwlock::SafeLock, service::{link_name::LinkName, service::Service, service_handles::ServiceHandles, RECV_TIMEOUT},
    services::Services,
};
///
/// 
pub struct MockRecvSendService {
    id: String,
    name: Name,
    rx_send: HashMap<String, Sender<Point>>,
    rx_recv: Mutex<Option<Receiver<Point>>>,
    send_to: LinkName,
    services: Arc<RwLock<Services>>,
    test_data: Vec<Value>,
    sent: Arc<RwLock<Vec<Point>>>,
    received: Arc<RwLock<Vec<Point>>>,
    recv_limit: Option<usize>,
    exit: Arc<AtomicBool>,
}
//
// 
impl MockRecvSendService {
    pub fn new(parent: impl Into<String>, rx_queue: &str, send_to: &str, services: Arc<RwLock<Services>>, test_data: Vec<Value>, recv_limit: Option<usize>) -> Self {
        let name = Name::new(parent, format!("MockRecvSendService{}", COUNT.fetch_add(1, Ordering::Relaxed)));
        let (send, recv) = mpsc::channel::<Point>();
        Self {
            id: name.join(),
            name,
            rx_send: HashMap::from([(rx_queue.to_string(), send)]),
            rx_recv: Mutex::new(Some(recv)),
            send_to: LinkName::from_str(send_to).unwrap(),
            services,
            test_data,
            sent: Arc::new(RwLock::new(vec![])),
            received: Arc::new(RwLock::new(vec![])),
            recv_limit,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn sent(&self) -> Arc<RwLock<Vec<Point>>> {
        self.sent.clone()
    }
    ///
    /// 
    pub fn received(&self) -> Arc<RwLock<Vec<Point>>> {
        self.received.clone()
    }
}
//
// 
impl Object for MockRecvSendService {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> Name {
        self.name.clone()
    }
}
//
// 
impl Debug for MockRecvSendService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("MockRecvSendService")
            .field("id", &self.id)
            .finish()
    }
}
//
//
impl Service for MockRecvSendService {
    //
    //
    fn get_link(&mut self, name: &str) -> std::sync::mpsc::Sender<Point> {
        match self.rx_send.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    //
    //
    fn run(&mut self) -> Result<ServiceHandles<()>, String> {
        info!("{}.run | Starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let rx_recv = self.rx_recv.lock().unwrap().take().unwrap();
        let received = self.received.clone();
        let recv_limit = self.recv_limit.clone();
        let handle_recv = thread::Builder::new().name(format!("{}.run | Recv", self_id)).spawn(move || {
            info!("{}.run | Preparing thread Recv - ok", self_id);
            match recv_limit {
                Some(recv_limit) => {
                    let mut received_count = 0;
                    loop {
                        match rx_recv.recv_timeout(RECV_TIMEOUT) {
                            Ok(point) => {
                                trace!("{}.run | received: {:?}", self_id, point);
                                received.write().unwrap().push(point);
                                received_count += 1;
                            }
                            Err(_) => {}
                        };
                        if received_count >= recv_limit {
                            break;
                        }
                        if exit.load(Ordering::SeqCst) {
                            break;
                        }
                    }
                }
                None => {
                    loop {
                        match rx_recv.recv_timeout(RECV_TIMEOUT) {
                            Ok(point) => {
                                trace!("{}.run | received: {:?}", self_id, point);
                                received.write().unwrap().push(point);
                            }
                            Err(_) => {}
                        };
                        if exit.load(Ordering::SeqCst) {
                            break;
                        }
                    }
                }
            }
        });
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let tx_send = self.services.rlock(&self_id).get_link(&self.send_to).unwrap_or_else(|err| {
            panic!("{}.run | services.get_link error: {:#?}", self.id, err);
        });
        let test_data = self.test_data.clone();
        let sent = self.sent.clone();
        let handle_send = thread::Builder::new().name(format!("{}.run | Send", self_id)).spawn(move || {
            info!("{}.run | Preparing thread Send - ok", self_id);
            let tx_id = PointTxId::from_str(&self_id);
            for value in test_data.iter() {
                let point = value.to_point(tx_id,&format!("{}/test", self_id));
                match tx_send.send(point.clone()) {
                    Ok(_) => {
                        trace!("{}.run | send: {:?}", self_id, point);
                        sent.write().unwrap().push(point);
                    }
                    Err(err) => {
                        warn!("{}.run | send error: {:?}", self_id, err);
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
        });
        match (handle_recv, handle_send) {
            (Ok(handle_recv), Ok(handle_send)) => Ok(ServiceHandles::new(vec![
                (format!("{}/read", self.id), handle_recv),
                (format!("{}/write", self.id), handle_send),
                ])),
            // TODO Exit 'write if read returns error'
            (Ok(_handle_recv), Err(err)) => Err(format!("{}.run | Error starting inner thread 'recv': {:#?}", self.id, err)),
            // TODO Exit 'read if write returns error'
            (Err(err), Ok(_handle_send)) => Err(format!("{}.run | Error starting inner thread 'send': {:#?}", self.id, err)),
            (Err(read_err), Err(write_err)) => Err(format!("{}.run | Error starting inner thread: \n\t  recv: {:#?}\n\t send: {:#?}", self.id, read_err, write_err)),
        }
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

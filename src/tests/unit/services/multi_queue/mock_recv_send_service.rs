use coco::Stack;
use log::{info, warn, trace};
use sal_core::{dbg::Dbg, error::Error};
use std::{
    collections::HashMap, fmt::Debug, str::FromStr,
    sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, mpsc::{self, Receiver, Sender}, Arc, Mutex, RwLock},
    thread::{self, JoinHandle},
};
use testing::entities::test_value::Value;
use crate::services::{
    entity::{Name, Object, Point, ToPoint, PointTxId},
    safe_lock::rwlock::SafeLock, service::{LinkName, Service, RECV_TIMEOUT},
    services::Services,
};
///
/// 
pub struct MockRecvSendService {
    dbg: Dbg,
    name: Name,
    rx_send: HashMap<String, Sender<Point>>,
    rx_recv: Mutex<Option<Receiver<Point>>>,
    send_to: LinkName,
    services: Arc<RwLock<Services>>,
    test_data: Vec<Value>,
    sent: Arc<RwLock<Vec<Point>>>,
    received: Arc<RwLock<Vec<Point>>>,
    recv_limit: Option<usize>,
    handles: Stack<(String, JoinHandle<()>)>,
    is_finished: Arc<AtomicBool>,
    exit: Arc<AtomicBool>,
}
//
// 
impl MockRecvSendService {
    pub fn new(parent: impl Into<String>, rx_queue: &str, send_to: &str, services: Arc<RwLock<Services>>, test_data: Vec<Value>, recv_limit: Option<usize>) -> Self {
        let parent = parent.into();
        let me = format!("MockRecvSendService{}", COUNT.fetch_add(1, Ordering::Relaxed));
        let (send, recv) = mpsc::channel::<Point>();
        Self {
            dbg: Dbg::new(&parent, &me),
            name: Name::new(parent, me),
            rx_send: HashMap::from([(rx_queue.to_string(), send)]),
            rx_recv: Mutex::new(Some(recv)),
            send_to: LinkName::from_str(send_to).unwrap(),
            services,
            test_data,
            sent: Arc::new(RwLock::new(vec![])),
            received: Arc::new(RwLock::new(vec![])),
            recv_limit,
            handles: Stack::new(),
            is_finished: Arc::new(AtomicBool::new(false)),
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
            .field("id", &self.dbg)
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
            None => panic!("{}.run | link '{:?}' - not found", self.dbg, name),
        }
    }
    //
    //
    fn run(&mut self) -> Result<(), Error> {
        info!("{}.run | Starting...", self.dbg);
        let dbg = self.dbg.clone();
        let exit = self.exit.clone();
        let rx_recv = self.rx_recv.lock().unwrap().take().unwrap();
        let received = self.received.clone();
        let recv_limit = self.recv_limit.clone();
        let handle_recv = thread::Builder::new().name(format!("{}.run | Recv", dbg)).spawn(move || {
            info!("{}.run | Preparing thread Recv - ok", dbg);
            match recv_limit {
                Some(recv_limit) => {
                    let mut received_count = 0;
                    loop {
                        match rx_recv.recv_timeout(RECV_TIMEOUT) {
                            Ok(point) => {
                                trace!("{}.run | received: {:?}", dbg, point);
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
                                trace!("{}.run | received: {:?}", dbg, point);
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
        let dbg = self.dbg.clone();
        let name = self.dbg.to_string();
        let exit = self.exit.clone();
        let tx_send = self.services.rlock(&dbg).get_link(&self.send_to).unwrap_or_else(|err| {
            panic!("{}.run | services.get_link error: {:#?}", self.dbg, err);
        });
        let test_data = self.test_data.clone();
        let sent = self.sent.clone();
        let handle_send = thread::Builder::new().name(format!("{}.run | Send", dbg)).spawn(move || {
            info!("{}.run | Preparing thread Send - ok", dbg);
            let tx_id = PointTxId::from_str(&name);
            for value in test_data.iter() {
                let point = value.to_point(tx_id,&format!("{}/test", dbg));
                match tx_send.send(point.clone()) {
                    Ok(_) => {
                        trace!("{}.run | send: {:?}", dbg, point);
                        sent.write().unwrap().push(point);
                    }
                    Err(err) => {
                        warn!("{}.run | send error: {:?}", dbg, err);
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
        });
        match (handle_recv, handle_send) {
            (Ok(handle_recv), Ok(handle_send)) => {
                self.handles.push((format!("{}/read", self.dbg), handle_recv));
                self.handles.push((format!("{}/write", self.dbg), handle_send));
                Ok(())
            }
            (Ok(handle_recv), Err(err)) => {
                self.handles.push((format!("{}/read", self.dbg), handle_recv));
                Err(Error::new(&self.dbg, "run").err(format!("Error starting inner thread 'recv': {:#?}", err)))
            }
            (Err(err), Ok(handle_send)) => {
                self.handles.push((format!("{}/write", self.dbg), handle_send));
                Err(Error::new(&self.dbg, "run").err(format!("Error starting inner thread 'send': {:#?}", err)))
            }
            (Err(read_err), Err(write_err)) => Err(Error::new(&self.dbg, "run").err(format!("Error starting inner thread: \n\t  recv: {:#?}\n\t send: {:#?}", read_err, write_err))),
        }
    }
    //
    //
    fn is_finished(&self) -> bool {
        self.is_finished.load(Ordering::SeqCst)
    }
    //
    //
    fn wait(&self) -> Result<(), Error> {
        let dbg = self.dbg.clone();
        while !self.handles.is_empty() {
            if let Some((id, handle)) = self.handles.pop() {
                if let Err(err) = handle.join() {
                    log::warn!("{dbg}.wait | Wait for '{id}' error: {:?}", err);
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
///
/// Global static counter of FnOut instances
static COUNT: AtomicUsize = AtomicUsize::new(0);

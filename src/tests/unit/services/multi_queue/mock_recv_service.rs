use std::{collections::HashMap, fmt::Debug, sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, mpsc::{self, Receiver, Sender}, Arc}, thread::{self, JoinHandle}};
use coco::Stack;
use log::{info, trace, warn};
use sal_core::{dbg::Dbg, error::Error};
use crate::services::{
    entity::{Name, Object, Point}, types::{Mutex, RwLock}, Service, RECV_TIMEOUT
};
///
/// Global static counter of FnOut instances
static COUNT: AtomicUsize = AtomicUsize::new(0);
///
/// 
pub struct MockRecvService {
    dbg: Dbg,
    name: Name,
    rx_send: HashMap<String, Sender<Point>>,
    rx_recv: Mutex<Option<Receiver<Point>>>,
    received: Arc<RwLock<Vec<Point>>>,
    recv_limit: Option<usize>,
    handle: Stack<JoinHandle<()>>,
    exit: Arc<AtomicBool>,
}
//
// 
impl MockRecvService {
    pub fn new(parent: impl Into<String>, rx_queue: &str, recv_limit: Option<usize>) -> Self {
        let name = Name::new(parent, format!("MockRecvService{}", COUNT.fetch_add(1, Ordering::Relaxed)));
        let (send, recv) = mpsc::channel::<Point>();
        Self {
            dbg: Dbg::new(name.parent(), name.me()),
            name,
            rx_send: HashMap::from([(rx_queue.to_string(), send)]),
            rx_recv: Mutex::new(Some(recv)),
            received: Arc::new(RwLock::new(vec![])),
            recv_limit,
            handle: Stack::new(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    // pub fn id(&self) -> String {
    //     self.id.clone()
    // }
    ///
    /// 
    pub fn received(&self) -> Arc<RwLock<Vec<Point>>> {
        self.received.clone()
    }
}
//
// 
impl Object for MockRecvService {
    fn name(&self) -> Name {
        self.name.clone()
    }
}
//
// 
impl Debug for MockRecvService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("MockRecvService")
            .field("id", &self.dbg)
            .finish()
    }
}
//
//
impl Service for MockRecvService {
    //
    //
    fn get_link(&self, name: &str) -> std::sync::mpsc::Sender<Point> {
        match self.rx_send.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.dbg, name),
        }
    }
    //
    //
    fn run(&self) -> Result<(), Error> {
        info!("{}.run | Starting...", self.dbg);
        let self_id = self.dbg.clone();
        let exit = self.exit.clone();
        let in_recv = self.rx_recv.lock().unwrap().take().unwrap();
        let received = self.received.clone();
        let recv_limit = self.recv_limit.clone();
        let handle = thread::Builder::new().name(format!("{}.run", self_id)).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            match recv_limit {
                Some(recv_limit) => {
                    let mut received_count = 0;
                    loop {
                        match in_recv.recv_timeout(RECV_TIMEOUT) {
                            Ok(point) => {
                                trace!("{}.run | received: {:?}", self_id, point);
                                match received.write() {
                                    Ok(mut received) => received.push(point),
                                    Err(err) => log::warn!("{}.run | RwLock.write error: {:?}", self_id, err),
                                };
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
                        match in_recv.recv_timeout(RECV_TIMEOUT) {
                            Ok(point) => {
                                trace!("{}.run | received: {:?}", self_id, point);
                                match received.write() {
                                    Ok(mut received) => received.push(point),
                                    Err(err) => log::warn!("{}.run | RwLock.write error: {:?}", self_id, err),
                                };
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
        match handle {
            Ok(handle) => {
                info!("{}.run | Starting - ok", self.dbg);
                self.handle.push(handle);
                Ok(())
            }
            Err(err) => {
                let message = format!("{}.run | Start failed: {:#?}", self.dbg, err);
                warn!("{}", message);
                Err(Error::new(&self.dbg, "run").err(message))
            }
        }        
    }
    //
    //
    fn wait(&self) -> Result<(), Error> {
        let dbg = self.dbg.clone();
        if let Some(handle) = self.handle.pop() {
            if let Err(err) = handle.join() {
                log::warn!("{dbg}.wait | Error: {:?}", err);
            }
        }
        Ok(())
    }
    //
    //
    fn is_finished(&self) -> bool {
        todo!()
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}

use std::{fmt::Debug, sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, RwLock}, thread::{self, JoinHandle}, time::Duration};
use coco::Stack;
use log::{info, trace, warn};
use sal_core::{dbg::Dbg, error::Error};
use crate::services::{entity::{Name, Object, Point}, safe_lock::rwlock::SafeLock, service::Service, services::Services};
#[cfg(test)]

mod multi_queue {
    use log::debug;
    use sal_core::dbg::Dbg;
    use std::{sync::{Arc, Once, RwLock}, thread, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::{
        entities::test_value::Value,
        stuff::{random_test_values::RandomTestValues, max_test_duration::TestDuration},
    };
    use crate::{
        services::{conf::{ConfTree, ServicesConf}, multi_queue::{MultiQueue, MultiQueueConf}, safe_lock::rwlock::SafeLock, service::Service, services::Services},
        tests::unit::services::multi_queue::{mock_send_service::MockSendService, multi_queue_subscribe_test::MockReceiver},
    };
    ///
    ///
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Test MultiQueue for broadcast subscription
    /// - events sent by multiple senders
    ///     - number of events = iterations
    /// - events received by multiple receivers
    ///     - each receiver must receive events: iterations * sender_count
    #[test]
    fn subscribe_broadcast() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let dbg = Dbg::own("multi_queue_subscribe");
        println!("\n{}", dbg);
        let sender_count = 10;         // count of MockSendService's
        let receiver_count = 10;         // count of MockReceiver's
        let iterations = 1000;          // test data length of the single sender
        let total_test_events = sender_count * iterations;
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let conf = r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                send-to:  # direct send links - are empty, because only client subscribtions will be used
        "#.to_string();
        let conf = serde_yaml::from_str(&conf).unwrap();
        let mq_conf = MultiQueueConf::from_yaml(&dbg, &conf);
        debug!("mqConf: {:?}", mq_conf);
        let services = Arc::new(RwLock::new(Services::new(&dbg, ServicesConf::new(
            &dbg, 
            ConfTree::new_root(serde_yaml::from_str(r#""#).unwrap()),
        ))));
        let mq_service = Arc::new(RwLock::new(MultiQueue::new(mq_conf, services.clone())));
        services.wlock(&dbg).insert(mq_service.clone());
        let mut receivers = vec![];
        for _ in 0..receiver_count {
            let receiver = Arc::new(RwLock::new(MockReceiver::new(
                &dbg,
                &format!("/{}/MultiQueue", dbg),
                services.clone(),
                Some(total_test_events),
            )));
            services.wlock(&dbg).insert(receiver.clone());
            receivers.push(receiver);
        }
        mq_service.write().unwrap().run().unwrap();
        for receiver in &receivers {
            receiver.write().unwrap().run().unwrap();
        }
        println!("All MockReceiver's threads - started");
        thread::sleep(Duration::from_millis(100));
        let mut senders = vec![];
        let time = Instant::now();
        for i in 0..sender_count {
            let dynamic_test_data = RandomTestValues::new(
                &dbg,
                vec![
                    Value::String(format!("dynamic01{}", i)),
                    Value::String(format!("dynamic02{}", i)),
                    Value::String(format!("dynamic03{}", i)),
                    Value::String(format!("dynamic04{}", i)),
                    Value::String(format!("dynamic05{}", i)),
                    Value::String(format!("dynamic06{}", i)),
                    Value::String(format!("dynamic07{}", i)),
                ],
                iterations,
            );
            let dynamic_test_data: Vec<Value> = dynamic_test_data.collect();
            let sender = Arc::new(RwLock::new(MockSendService::new(
                &dbg,
                &format!("/{}/MultiQueue.in-queue", dbg),
                services.clone(),
                dynamic_test_data.clone(),
                None,
            )));
            services.wlock(&dbg).insert(sender.clone());
            senders.push(sender.clone());
        }
        services.wlock(&dbg).run().unwrap();
        for sender in &senders {
            sender.write().unwrap().run().unwrap();
        }
        for s in &senders {
            s.read().unwrap().wait().wait().unwrap()
        }
        for r in &receivers {
            r.read().unwrap().wait().wait().unwrap();
        }
        for receiver in &receivers {
            receiver.read().unwrap().exit();
        }
        let elapsed = time.elapsed();
        println!("Total elapsed: {:?}", elapsed);
        println!("Total test events: {:?}", total_test_events);
        println!("Elapsed per event: {:?}", elapsed.div_f64(total_test_events as f64));
        let target = iterations;
        for sender in senders {
            let sent = sender.read().unwrap().sent();
            let result = sent.read().unwrap().len();
            println!("\t {} sent: {:?}", sender.read().unwrap().id(), result);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        let target = total_test_events;
        for receiver in receivers {
            let result = receiver.read().unwrap().received.read().unwrap().len();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        mq_service.read().unwrap().exit();
        services.rlock(&dbg).exit();
        mq_service.read().unwrap().wait().wait().unwrap();
        _ = services.rlock(&dbg).wait().wait();
        test_duration.exit();
    }
}

///
/// Receiver with subscribtion to Multiqueue
struct MockReceiver {
    dbg: Dbg,
    name: Name,
    subscribe: String,
    services: Arc<RwLock<Services>>,
    received: Arc<RwLock<Vec<Point>>>,
    recv_limit: Option<usize>,
    handle: Stack<JoinHandle<()>>,
    exit: Arc<AtomicBool>,
}
//
//
impl MockReceiver {
    pub fn new(parent: impl Into<String>, subscribe: &str, services: Arc<RwLock<Services>>, recv_limit: Option<usize>) -> Self {
        let name = Name::new(parent, format!("MockReceiver{}", COUNT.fetch_add(1, Ordering::Relaxed)));
        Self {
            dbg: Dbg::new(name.parent(), name.me()),
            name,
            subscribe: subscribe.to_owned(),
            services,
            received: Arc::new(RwLock::new(vec![])),
            recv_limit,
            handle: Stack::new(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
//
//
impl Object for MockReceiver {
    fn name(&self) -> Name {
        self.name.clone()
    }
}
//
//
impl Debug for MockReceiver {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("MockReceiver")
            .field("id", &self.dbg)
            .finish()
    }
}
//
//
impl Service for MockReceiver {
    //
    //
    fn run(&mut self) -> Result<(), Error> {
        let dbg = self.dbg.clone();
        let exit = self.exit.clone();
        let recv_limit = self.recv_limit;
        let subscribe = self.subscribe.clone();
        let received = self.received.clone();
        let services = self.services.clone();
        let handle = thread::Builder::new().name(format!("{}.run", dbg)).spawn(move || {
            let dbg = dbg.clone();
            let points = vec![];
            let (_, recv) = services.wlock(&dbg).subscribe(&subscribe, &dbg.to_string(), &points);
            match recv_limit {
                Some(recv_limit) => {
                    let mut received_len = 0;
                    while received_len < recv_limit {
                        match recv.recv_timeout(Duration::from_secs(3)) {
                            Ok(point) => {
                                received_len += 1;
                                trace!("{}.run | Received point: {:#?}", dbg, point);
                                received.write().unwrap().push(point);
                            }
                            Err(err) => match err {
                                std::sync::mpsc::RecvTimeoutError::Timeout      => warn!("{}.run | Receive error: {:#?}", dbg, err),
                                std::sync::mpsc::RecvTimeoutError::Disconnected => {}
                            }
                        }
                        if exit.load(Ordering::SeqCst) {
                            break;
                        }
                    }
                }
                None => {
                    loop {
                        match recv.recv_timeout(Duration::from_secs(3)) {
                            Ok(point) => {
                                received.write().unwrap().push(point)
                            }
                            Err(err) => match err {
                                std::sync::mpsc::RecvTimeoutError::Timeout      => warn!("{}.run | Receive error: {:#?}", dbg, err),
                                std::sync::mpsc::RecvTimeoutError::Disconnected => {}
                            }
                        }
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
    fn wait(&self) -> crate::services::future::Future<()> {
        let dbg = self.dbg.clone();
        let (future, sink) = crate::services::future::Future::new();
        if let Some(handle) = self.handle.pop() {
            std::thread::spawn(move|| {
                if let Err(err) = handle.join() {
                    log::warn!("{dbg}.wait | Error: {:?}", err);
                }
                sink.add(());
            });
        }
        future
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

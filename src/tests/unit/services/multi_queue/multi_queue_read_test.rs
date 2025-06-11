#[cfg(test)]

mod multi_queue {
    use log::debug;
    use std::{sync::{Arc, Once}, time::{Duration, Instant}};
    use testing::{entities::test_value::Value, stuff::{max_test_duration::TestDuration, random_test_values::RandomTestValues}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        services::{
            conf::{ConfTree, ServicesConf}, entity::Object, task::functions::AtomicReset, MultiQueue, MultiQueueConf, Service, Services
        },
        tests::unit::services::multi_queue::{mock_recv_service::MockRecvService, mock_send_service::{self, MockSendService}}, thread_pool::ThreadPool,
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
    /// Test MultiQueue for - static link
    /// - action: read
    #[test]
    fn static_read() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let dbg = "multi_queue_read_test";
        println!("\n{}", dbg);
        //
        // can be changed
        let iterations = 10;
        let test_data = RandomTestValues::new(
            dbg,
            vec![
                Value::Int(i64::MIN),
                Value::Int(i64::MAX),
                Value::Int(-7),
                Value::Int(0),
                Value::Int(12),
                Value::Real(f32::MAX),
                Value::Real(f32::MIN),
                Value::Real(f32::MIN_POSITIVE),
                Value::Real(-f32::MIN_POSITIVE),
                Value::Real(0.0),
                Value::Real(1.33),
                Value::Double(f64::MAX),
                Value::Double(f64::MIN),
                Value::Double(f64::MIN_POSITIVE),
                Value::Double(-f64::MIN_POSITIVE),
                Value::Bool(true),
                Value::Bool(false),
                Value::Bool(false),
                Value::Bool(true),
                Value::String("test1".to_string()),
                Value::String("test1test1test1test1test1test1test1test1test1test1test1test1test1test1test1".to_string()),
                Value::String("test2".to_string()),
                Value::String("test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2".to_string()),
            ],
            iterations,
        );
        let test_data: Vec<Value> = test_data.collect();
        let test_data_len = test_data.len();
        let count = 30;
        let total_count = count * test_data.len();
        let test_duration = TestDuration::new(dbg, Duration::from_secs(30));
        test_duration.run().unwrap();
        let thread_pool = ThreadPool::new(dbg, Some(8));
        let services = Arc::new(Services::new(dbg, ServicesConf::new(
                dbg, 
                ConfTree::new_root(serde_yaml::from_str(r#"
                    retain:
                        path: assets/testing/retain/
                        point:
                            path: point/id.json
                "#).unwrap()),
            ),
            Some(thread_pool.scheduler()),
        ));
        let mut recv_services = vec![];
        for _ in 0..count {
            let recv_service = Arc::new(MockRecvService::new(
                dbg,
                "in-queue",
                Some(iterations),
            ));
            services.insert(recv_service.clone());
            recv_services.push(recv_service);
        }
        let mut conf = r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                send-to:
        "#.to_string();
        for s in &recv_services {
            // conf = format!("{}\n                    - /{}/MockRecvService{}.in-queue", conf, self_id, i)
            conf = format!("{}\n                    - {}.in-queue", conf, s.name().join())
        }
        let conf = serde_yaml::from_str(&conf).unwrap();
        let mq_conf = MultiQueueConf::from_yaml(dbg, &conf);
        debug!("mqConf: {:?}", mq_conf);
        let mq_service = Arc::new(MultiQueue::new(mq_conf, services.clone(), Some(thread_pool.scheduler())));
        services.insert(mq_service.clone());
        let timer = Instant::now();
        mock_send_service::COUNT.reset(0);
        let send_service = Arc::new(MockSendService::new(
            dbg,
            &format!("/{}/MultiQueue.in-queue", dbg),
            services.clone(),
            test_data.clone(),
            None,
        ));
        services.insert(send_service.clone());
        mq_service.run().unwrap();
        for service in &mut recv_services {
            service.run().unwrap();
        }
        send_service.run().unwrap();
        for thd in &recv_services {
            thd.wait().unwrap();
        }
        println!("\nelapsed: {:?}", timer.elapsed());
        println!("total test events: {:?}", total_count);
        println!("sent events: {:?}\n", count * send_service.sent().read().len());
        let mut received = vec![];
        let target = test_data_len;
        for recv_service in &recv_services {
            let len = recv_service.received().read().len();
            assert!(len == target, "\nresult: {:?}\ntarget: {:?}", len, target);
            received.push(len);
        }
        println!("recv events: {} {:?}", received.iter().sum::<usize>(), received);
        for service in recv_services {
            service.exit();
        }
        services.exit();
        services.wait().unwrap();
        mq_service.exit();
        test_duration.exit();
    }
}

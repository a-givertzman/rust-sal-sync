#[cfg(test)]

mod multi_queue {
    use std::{sync::{Arc, Once}, thread, time::{Duration, Instant}};
    use testing::{entities::test_value::Value, stuff::{max_test_duration::TestDuration, random_test_values::RandomTestValues}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{services::{conf::{ConfTree, ServicesConf}, entity::{Name, Object}, MultiQueue, MultiQueueConf, Service, Services}, tests::unit::services::multi_queue::{mock_recv_service::MockRecvService, mock_send_service::MockSendService}};
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
    /// Can be changed
    const ITERATIONS: usize = 1_000_000;
    ///
    /// Use to estimate performance of multiqueue without matching producer's id
    #[ignore = "MultiQueue Performance test"]
    #[test]
    fn performance() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let dbg = "MQ-Performance-test";
        println!("\n{}", dbg);
        let iterations = ITERATIONS;
        let test_duration = TestDuration::new(dbg, Duration::from_secs(30));
        test_duration.run().unwrap();
        let receiver_count = 3;
        let producer_count = 3;
        let total_count = iterations * producer_count;
        let mut receivers: Vec<Arc<MockRecvService>> = vec![];
        let mut producers: Vec<MockSendService> = vec![];
        let services = Arc::new(Services::new(dbg, ServicesConf::new(
            dbg, 
            ConfTree::new_root(serde_yaml::Value::Null),
        )));
        for i in 0..receiver_count {
            let receiver = Arc::new(MockRecvService::new(
                dbg,
                "rx-queue",
                Some(total_count)
            ));
            let receiver_id = format!("Receiver{}", i + 1);
            services.insert(receiver.clone());
            receivers.push(receiver);
            println!(" Receiver {} created", receiver_id);
        }
        println!(" All receivers created");
        println!(" Creating Mock Multiqueue...");
        let conf = serde_yaml::from_str(&format!(r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                send-to:
                    {:?}
        "#, receivers.iter().map(|v| format!("{}.rx-queue", v.name())).collect::<Vec<String>>())).unwrap();
        println!(" Multiqueue conf: {:#?}", conf);
        let conf = MultiQueueConf::from_yaml(dbg, &conf);
        let mq = Arc::new(MultiQueue::new(conf, services.clone()));
        println!(" Creating {} - ok", mq.name());
        println!(" Inserting Mock Multiqueue into Services...");
        services.insert(mq.clone());
        println!(" Inserting Mock Multiqueue into Services - ok");
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
                Value::Double(0.0),
                Value::Double(1.33),
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
        services.run().unwrap();
        println!(" Trying to start Multiqueue...:");
        mq.run().unwrap();
        for recv in &receivers {
            recv.run().unwrap();
        }
        for _ in 0..producer_count {
            let prod = MockSendService::new(dbg, &format!("/{dbg}/MultiQueue.in-queue"), services.clone(), test_data.clone(), None);
            prod.run().unwrap();
            producers.push(prod);
        }

        let timer = Instant::now();
        for h in &receivers {
            h.wait().unwrap();
        }
        services.wait().unwrap();
        println!("\n Elapsed: {:?}", timer.elapsed());
        println!(" Total test events: {:?}", total_count);
        let (total_sent, all_sent) = get_sent(&producers);
        println!(" Sent events: {}\t{:?}", total_sent, all_sent);
        let (total_received, all_received) = get_received(&receivers);
        println!(" Recv events: {}\t{:?}\n", total_received, all_received);

        assert!(total_sent == total_count, "\nresult: {:?}\ntarget: {:?}", total_sent, total_count);
        assert!(total_received == total_count * receiver_count, "\nresult: {:?}\ntarget: {:?}", total_received, total_count * receiver_count);
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
    ///
    /// Use to estimate performance of multiqueue with matching producer's id
    #[ignore = "Performance test"]
    #[test]
    fn match_performance() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let dbg = "test MQ-Performance(matching by producer ID)";
        println!("\n{}", dbg);
        let iterations = ITERATIONS;
        let test_duration = TestDuration::new(dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let receiver_count = 3;
        let producer_count = 3;
        let total_count = iterations * producer_count;
        let mut receivers = vec![];
        let mut producers = vec![];
        let services = Arc::new(Services::new(dbg, ServicesConf::new(
            dbg, 
            ConfTree::new_root(serde_yaml::Value::Null),
        )));
        for i in 0..receiver_count {
            let receiver = Arc::new(MockRecvService::new(
                dbg,
                "rx-queue",
                Some(total_count)
            ));
            let receiver_id = format!("/{}/MockRecvService{}", dbg, i);
            services.insert(receiver.clone());
            receivers.push(receiver);
            println!(" Receiver {} created", receiver_id);
        }
        println!(" All receivers created");
        println!(" Creating Mock Multiqueue...");
        let conf = serde_yaml::from_str(&format!(r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                send-to:
                    {:?}
        "#, receivers.iter().map(|v| format!("{}.rx-queue", v.name())).collect::<Vec<String>>())).unwrap();
        println!(" Multiqueue conf: {:#?}", conf);
        let conf = MultiQueueConf::from_yaml(dbg, &conf);
        let mq_service = Arc::new(MultiQueue::new(conf, services.clone()));
        println!(" Creating Mock Multiqueue - ok");
        println!(" Inserting Mock Multiqueue into Services...");
        services.insert(mq_service.clone());
        println!(" Inserting Mock Multiqueue into Services - ok");
        let test_data = RandomTestValues::new(
            dbg,
            vec![
                Value::Int(7),
                Value::Real(1.3),
                Value::Double(1.3),
                Value::Bool(true),
                Value::Bool(false),
                Value::String("test1".to_string()),
                Value::String("test2".to_string()),
            ],
            iterations,
        );
        let test_data: Vec<Value> = test_data.collect();
        services.run().unwrap();
        thread::sleep(Duration::from_millis(50));
        println!(" Trying to start Multiqueue...:");
        mq_service.run().unwrap();
        let mut recv_handles  = vec![];
        for recv in &receivers {
            let h = recv.run().unwrap();
            recv_handles.push(h)
        }
        for _ in 0..producer_count {
            let prod = MockSendService::new(
                dbg,
                &Name::new(dbg, "MockMultiQueueMatch0.rx-queue").join(),
                services.clone(),
                test_data.clone(),
                None,
            );
            prod.run().unwrap();
            producers.push(prod);
        }
        let timer = Instant::now();
        for h in &receivers {
            h.wait().unwrap();
        }
        services.wait().unwrap();
        println!("\n Elapsed: {:?}", timer.elapsed());
        println!(" Total test events: {:?}", total_count);
        let (total_sent, all_sent) = get_sent(&producers);
        println!(" Sent events: {}\t{:?}", total_sent, all_sent);
        let (total_received, all_received) = get_received(&receivers);
        println!(" Recv events: {}\t{:?}\n", total_received, all_received);
        assert!(total_sent == total_count, "\nresult: {:?}\ntarget: {:?}", total_sent, total_count);
        assert!(total_received == total_count * receiver_count, "\nresult: {:?}\ntarget: {:?}", total_received, total_count * receiver_count);
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
    ///
    ///
    fn get_sent(producers: &Vec<MockSendService>) -> (usize, Vec<(String, usize)>) {
        let mut total_sent = 0;
        let mut all_sent = vec![];
        for prod in producers {
            let sent = prod.sent().read().len();
            total_sent += sent;
            all_sent.push((prod.name().join(), sent));
        }
        (total_sent, all_sent)
    }
    ///
    ///
    fn get_received(receivers: &Vec<Arc<MockRecvService>>) -> (usize, Vec<(String, usize)>) {
        let mut total_received = 0;
        let mut all_received = vec![];
        for recv in receivers {
            let recved = recv.received().read().len();
            total_received += recved;
            all_received.push((recv.name().join(), recved));
        }
        (total_received, all_received)
    }
}
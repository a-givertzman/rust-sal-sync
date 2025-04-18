#[cfg(test)]

mod trait_service {
    use log::debug;
    use sal_core::{dbg::Dbg, error::Error};
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::services::{entity::{name::Name, object::Object}, service::{service::Service, service_handles::ServiceHandles}};
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
    /// Testing trait Service
    #[test]
    fn basic() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = Dbg::own("test");
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(&self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let name = Name::new(&self_id, "ServiceTest");
        let mut service_test = ServiceTest { id: name.join(), name  };
        let result = service_test.run();
        let target: Result<ServiceHandles<()>, Error> = Err(Error::new("ServiceTest", "run").err("testing"));
        match result {
            Ok(_) => panic!(""),
            Err(result) => {
                if let Err(target) = target {
                    assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
                }
            }
        }
        let result = service_test.points();
        let target = vec![];
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
    ///
    /// Testing trait Service::get_link
    #[test]
    #[should_panic]
    fn get_link() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "get_link";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let name = Name::new(self_id, "ServiceTest");
        let mut service_test = ServiceTest { id: name.join(), name  };
        let _ = service_test.get_link("");
        test_duration.exit();
    }    
    ///
    /// Testing trait Service::subscribe
    #[test]
    #[should_panic]
    fn subscribe() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "subscribe";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let name = Name::new(self_id, "ServiceTest");
        let mut service_test = ServiceTest { id: name.join(), name  };
        let _ = service_test.subscribe("", &[]);
        test_duration.exit();
    }
    ///
    /// Testing trait Service::extend_subscription
    #[test]
    #[should_panic]
    fn extend_subscription() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "extend_subscription";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let name = Name::new(self_id, "ServiceTest");
        let mut service_test = ServiceTest { id: name.join(), name  };
        let _ = service_test.extend_subscription("", &[]);
        test_duration.exit();
    }
    ///
    /// Testing trait Service::unsubscribe
    #[test]
    #[should_panic]
    fn unsubscribe() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "unsubscribe";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let name = Name::new(self_id, "ServiceTest");
        let mut service_test = ServiceTest { id: name.join(), name  };
        let _ = service_test.unsubscribe("", &[]);
        test_duration.exit();
    }
    ///
    /// Testing trait Service::gi
    #[test]
    #[should_panic]
    fn gi() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "gi";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let name = Name::new(self_id, "ServiceTest");
        let service_test = ServiceTest { id: name.join(), name  };
        let _ = service_test.gi("", &[]);
        test_duration.exit();
    }
    ///
    /// 
    struct ServiceTest {
        id: String,
        name: Name,
    }
    //
    //
    impl Service for ServiceTest {
        fn run(&mut self) -> Result<ServiceHandles<()>, Error> {
            Err(Error::new("ServiceTest", "run").err("testing"))
        }
    
        fn exit(&self) {
        }
    }
    //
    //
    impl std::fmt::Debug for ServiceTest {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ServiceTest").finish()
        }
    }
    //
    //
    impl Object for ServiceTest {
        fn id(&self) -> &str {
            self.id.as_str()
        }
    
        fn name(&self) -> crate::services::entity::name::Name {
            self.name.clone()
        }
    }
}

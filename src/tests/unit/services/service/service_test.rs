#[cfg(test)]

mod trait_service {
    use log::debug;
    use sal_core::{dbg::Dbg, error::Error};
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::services::{entity::{Name, Object}, Service};
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
        let dbg = Dbg::own("basic");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let name = Name::new(&dbg, "ServiceTest");
        let service_test = ServiceTest { name  };
        let result = service_test.run();
        let target: Result<(), Error> = Err(Error::new("ServiceTest", "run").err("testing"));
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
        let dbg = Dbg::own("get_link");
        debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let service_test = ServiceTest { name: Name::new(&dbg, "ServiceTest") };
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
        let dbg = Dbg::own("subscribe");
        debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let service_test = ServiceTest { name: Name::new(&dbg, "ServiceTest") };
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
        let dbg = Dbg::own("extend_subscription");
        debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let name = Name::new(&dbg, "ServiceTest");
        let service_test = ServiceTest { name  };
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
        let dbg = Dbg::own("unsubscribe");
        debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let name = Name::new(&dbg, "ServiceTest");
        let service_test = ServiceTest { name  };
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
        let dbg = Dbg::own("gi");
        debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let name = Name::new(&dbg, "ServiceTest");
        let service_test = ServiceTest { name  };
        let _ = service_test.gi("", &[]);
        test_duration.exit();
    }
    ///
    /// 
    struct ServiceTest {
        name: Name,
    }
    //
    //
    impl Service for ServiceTest {
        fn run(&self) -> Result<(), Error> {
            Err(Error::new("ServiceTest", "run").err("testing"))
        }
        fn is_finished(&self) -> bool {
            todo!()
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
        fn name(&self) -> crate::services::entity::Name {
            self.name.clone()
        }
    }
}

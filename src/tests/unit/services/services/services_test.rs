#[cfg(test)]

mod services {
    use std::{env, sync::{atomic::{AtomicBool, Ordering}, Arc, Once}, time::{Duration, Instant}};
    use coco::Stack;
    use sal_core::dbg::Dbg;
    use testing::stuff::{max_test_duration::TestDuration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{services::{conf::{ConfTree, ServicesConf}, entity::{Name, Object}, Service, Services}, sync::WaitBox, thread_pool::{Scheduler, ThreadPool}};
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
    /// Testing `Services` on `std::thread`
    #[test]
    fn services_thread() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("test-Services-thread");
        log::debug!("\n{}", dbg);
        let tasks = 100;
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        log::trace!("dir: {:?}", env::current_dir());
        let time = Instant::now();
        let services = Arc::new(Services::new(&dbg, ServicesConf::new(
            &dbg, 
            ConfTree::empty(),
        ), None));
        services.run().unwrap();
        let tasks: Vec<Name> = (0..tasks).map(|i| {
            let task = Arc::new(ServiceMok::new(&dbg, i, None));
            let name = task.name();
            task.run().unwrap();
            services.insert(task);
            name
        }).collect();
        // assert!(points_count == target, "\nresult: {:?}\ntarget: {:?}", points_count, target);
        for t in tasks {
            let task = services.get(&t.join()).unwrap();
            task.exit();
            task.wait().unwrap();
        }
        services.exit();
        services.wait().unwrap();
        log::info!("{dbg} | All finished in {:?}", time.elapsed());
        test_duration.exit();
    }
    ///
    /// Testing `Services` on `std::thread`
    #[test]
    fn services_scheduler() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("test-Services-scheduler");
        log::debug!("\n{}", dbg);
        let tasks = 100;
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        log::trace!("dir: {:?}", env::current_dir());
        let time = Instant::now();
        let thread_pool = ThreadPool::new(&dbg, None);
        let services = Arc::new(Services::new(&dbg, ServicesConf::new(
            &dbg, 
            ConfTree::empty(),
        ), Some(thread_pool.scheduler())));
        services.run().unwrap();
        let tasks: Vec<Name> = (0..tasks).map(|i| {
            let task = Arc::new(ServiceMok::new(&dbg, i, Some(thread_pool.scheduler())));
            let name = task.name();
            task.run().unwrap();
            services.insert(task);
            name
        }).collect();
        // assert!(points_count == target, "\nresult: {:?}\ntarget: {:?}", points_count, target);
        for t in tasks {
            let task = services.get(&t.join()).unwrap();
            task.exit();
            task.wait().unwrap();
        }
        services.exit();
        services.wait().unwrap();
        log::info!("{dbg} | All finished in {:?}", time.elapsed());
        test_duration.exit();
    }
    ///
    /// Used for testing only
    struct ServiceMok {
        dbg: Dbg,
        name: Name,
        schrduler: Option<Scheduler>,
        is_finished: Arc<AtomicBool>,
        handle: Stack<Box<dyn WaitBox<()>>>,
        exit: Arc<AtomicBool>,
    }
    impl ServiceMok {
        fn new(parent: impl Into<String>, index: usize, schrduler: Option<Scheduler>) -> Self {
            let parent = parent.into();
            let me = format!("ServiceMok-{index}");
            let name = Name::new(&parent, &me);
            Self {
                dbg: Dbg::new(parent, me),
                name,
                schrduler,
                handle: Stack::new(),
                is_finished: Arc::new(AtomicBool::new(false)),
                exit: Arc::new(AtomicBool::new(false)),
            }
        }
        fn run_(dbg: Dbg, exit: Arc<AtomicBool>) {
            loop {
                std::thread::sleep(Duration::from_millis(50));
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
        }
    }
    impl Object for ServiceMok {
        fn name(&self) -> Name {
            self.name.clone()
        }
    }
    impl Service for ServiceMok {
        fn run(&self) -> Result<(), sal_core::error::Error> {
            let dbg = self.dbg.clone();
            let exit = self.exit.clone();
            let h: Box<dyn WaitBox<()>> = match &self.schrduler {
                Some(schrduler) => {
                    let h = schrduler.spawn(move|| {
                        Self::run_(dbg, exit);
                        Ok(())
                    })?;
                    Box::new(h)
                }
                None => {
                    let h = std::thread::spawn(move|| {
                        Self::run_(dbg, exit);
                    });
                    Box::new(h)
                }
            };
            self.handle.push(h);
            Ok(())
        }
        //
        fn is_finished(&self) -> bool {
            self.is_finished.load(Ordering::SeqCst)
        }
        //
        fn wait(&self) -> Result<(), sal_core::error::Error> {
            while !self.handle.is_empty() {
                if let Some(handle) = self.handle.pop() {
                    if let Err(err) = handle.wait() {
                        log::warn!("{}.wait | Error: {:?}", self.dbg, err);
                    }
                }
            }
            Ok(())
        }
        //
        fn exit(&self) {
            self.exit.store(true, Ordering::SeqCst);
        }
    }
    impl std::fmt::Debug for ServiceMok {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ServiceMok")
                .field("dbg", &self.dbg)
                .field("name", &self.name)
                .finish()
        }
    }
}


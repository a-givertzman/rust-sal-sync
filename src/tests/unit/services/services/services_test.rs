#[cfg(test)]

use std::{env, sync::{atomic::{AtomicBool, Ordering}, Arc, Once}, time::{Duration, Instant}};
use sal_core::dbg::Dbg;
use testing::stuff::{max_test_duration::TestDuration};
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
use crate::{services::{conf::{ConfTree, ServicesConf}, entity::{Name, Object}, Service, Services}, sync::Handles, thread_pool::{Scheduler, ThreadPool}};
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
    DebugSession::new().filter(LogLevel::Info).init();
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
/// Testing `Services` on `ThreadPool`
#[test]
fn services_scheduler() {
    DebugSession::new().filter(LogLevel::Info).init();
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
/// Testing `Services.all` insertion order
#[test]
fn services_all() {
    DebugSession::new().filter(LogLevel::Info).init();
    init_once();
    init_each();
    let dbg = Dbg::own("Services-all");
    log::debug!("\n{}", dbg);
    let tasks = 7;
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
        services.insert(task);
        name
    }).collect();
    // assert!(points_count == target, "\nresult: {:?}\ntarget: {:?}", points_count, target);
    let result: Vec<String> = services.all().into_iter().map(|(k, _)| k).collect();
    let target: Vec<String> = tasks.iter().map(|n| n.join()).collect();
    assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
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
    scheduler: Option<Scheduler>,
    handle: Handles<()>,
    exit: Arc<AtomicBool>,
}
impl ServiceMok {
    fn new(parent: impl Into<String>, index: usize, scheduler: Option<Scheduler>) -> Self {
        let parent = parent.into();
        let me = format!("ServiceMok-{index}");
        let name = Name::new(&parent, &me);
        let dbg = Dbg::new(parent, me);
        Self {
            name,
            scheduler,
            handle: Handles::new(&dbg),
            exit: Arc::new(AtomicBool::new(false)),
            dbg,
        }
    }
    fn run_(dbg: Dbg, exit: Arc<AtomicBool>) {
        log::trace!("{dbg} | Start");
        loop {
            std::thread::sleep(Duration::from_millis(50));
            if exit.load(Ordering::SeqCst) {
                break;
            }
        }
        log::trace!("{dbg} | Exit");
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
        match &self.scheduler {
            Some(scheduler) => {
                let h = scheduler.spawn(move|| {
                    Self::run_(dbg, exit);
                    Ok(())
                })?;
                self.handle.push(h);
            }
            None => {
                let h = std::thread::spawn(move|| {
                    Self::run_(dbg, exit);
                });
                self.handle.push(h);
            }
        };
        Ok(())
    }
    //
    fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }
    //
    fn wait(&self) -> Result<(), sal_core::error::Error> {
        self.handle.wait()
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

#[cfg(test)]

mod scheduler {
    use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc, Once}, time::{Duration, Instant}};
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::thread_pool::tread_pool::ThreadPool;
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
    /// Testing spawn with capacity = 1
    #[test]
    fn single_capacity() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("single_capacity");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let threads = 10;
        let thread_pool = ThreadPool::new(Some(1));
        let scheduler = thread_pool.scheduler();
        let time = Instant::now();
        let result = Arc::new(AtomicUsize::new(0));
        let load = 50;
        for i in 0..threads {
            let dbg_ = Dbg::new(&dbg, format!("thread{i}"));
            let result = result.clone();
            scheduler.spawn(move || {
                log::debug!("{dbg_}", );
                std::thread::sleep(Duration::from_millis(load));
                result.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }).unwrap();
        }
        std::thread::sleep(Duration::from_millis(load * (threads + 1)));
        log::debug!("Jobs sheduled: {threads} in: {:?}", time.elapsed());
        thread_pool.join().unwrap();
        log::debug!("Total elapsed: {:?}", time.elapsed());
        let target = threads as usize;
        let result = result.load(Ordering::SeqCst);
        assert!(result == target, "{} \nresult: {:?}\ntarget: {:?}", dbg, result, target);
        test_duration.exit();
    }
    ///
    /// Testing spawn with capacity = jobs + 30 %
    #[test]
    fn spawn() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("spawn");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let threads = 100;
        let thread_pool = ThreadPool::new(Some(threads + threads / 3));
        let scheduler = thread_pool.scheduler();
        let time = Instant::now();
        let result = Arc::new(AtomicUsize::new(0));
        for i in 0..threads {
            let dbg_ = Dbg::new(&dbg, format!("thread{i}"));
            let result = result.clone();
            scheduler.spawn(move || {
                log::debug!("{dbg_}", );
                std::thread::sleep(Duration::from_secs(1));
                result.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }).unwrap();
        }
        log::debug!("Jobs sheduled: {threads} in: {:?}", time.elapsed());
        std::thread::sleep(Duration::from_millis(100));
        thread_pool.join().unwrap();
        log::debug!("All Jobs done ({threads})");
        log::debug!("Total elapsed: {:?}", time.elapsed());
        let target = threads;
        let result = result.load(Ordering::SeqCst);
        assert!(result == target, "{} \nresult: {:?}\ntarget: {:?}", dbg, result, target);
        test_duration.exit();
    }
}

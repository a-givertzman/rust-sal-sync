#[cfg(test)]

mod thread_pool {
    use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc, Once}, time::{Duration, Instant}};
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{sync::Handles, thread_pool::ThreadPool};
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
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let threads = 10;
        let thread_pool = ThreadPool::new(&dbg, Some(1));
        let time = Instant::now();
        let result = Arc::new(AtomicUsize::new(0));
        let load = 50;
        for i in 0..threads {
            let dbg_ = Dbg::new(&dbg, format!("thread{i}"));
            let result = result.clone();
            thread_pool.spawn(move || {
                log::debug!("{dbg_}", );
                std::thread::sleep(Duration::from_millis(load));
                result.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }).unwrap();
        }
        std::thread::sleep(Duration::from_millis(load * (threads + 1)));
        log::debug!("Jobs sheduled: {threads} in: {:?}", time.elapsed());
        thread_pool.shutdown().unwrap();
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
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let threads = 100;
        let thread_pool = ThreadPool::new(&dbg, Some(threads + threads / 3));
        let time = Instant::now();
        let result = Arc::new(AtomicUsize::new(0));
        let handles = Handles::new(&dbg);
        for i in 0..threads {
            let dbg_ = Dbg::new(&dbg, format!("thread{i}"));
            let result = result.clone();
            let handle = thread_pool.spawn(move || {
                log::debug!("{dbg_}", );
                std::thread::sleep(Duration::from_millis(100));
                result.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }).unwrap();
            handles.push(handle);
        }
        log::debug!("Jobs sheduled: {threads} in: {:?}", time.elapsed());
        handles.wait().unwrap();
        thread_pool.shutdown().unwrap();
        log::debug!("All Jobs done ({threads})");
        log::debug!("Total elapsed: {:?}", time.elapsed());
        let target = threads;
        let result = result.load(Ordering::SeqCst);
        assert!(result == target, "{} \nresult: {:?}\ntarget: {:?}", dbg, result, target);
        test_duration.exit();
    }
}

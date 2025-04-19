#[cfg(test)]

mod thread_pool {
    use std::{sync::Once, time::{Duration, Instant}};
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
    /// Testing such functionality / behavior
    #[test]
    fn functionality() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("thread_pool_method");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        let thread_pool = ThreadPool::new(Some(10));
        let threads = 10;
        let time = Instant::now();
        for i in 0..threads {
            let dbg_ = Dbg::new(&dbg, format!("thread{i}"));
            thread_pool.spawn(move || {
                log::debug!("{dbg_}", );
                std::thread::sleep(Duration::from_secs(1));
                Ok(())
            });
        }
        log::debug!("Jobs sheduled: {threads} in: {:?}", time.elapsed());
        thread_pool.join().unwrap();
        log::debug!("Total elapsed: {:?}", time.elapsed());
        test_duration.run().unwrap();
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        test_duration.exit();
    }
}

#[cfg(test)]

mod future {
    use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc, Once}, time::{Duration, Instant}};
    use sal_core::{dbg::Dbg, error::Error};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        services::future::future::Future,
        thread_pool::tread_pool::ThreadPool,
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
    /// Testing spawn method
    #[test]
    fn spawn() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("spawn");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let threads = 10;
        let test_data = [
            (01, 11, Ok(11 * 2)),
            (02, 22, Ok(22 * 2)),
            (03, 33, Ok(33 * 2)),
            (04, 44, Ok(44 * 2)),
            (05, 55, Ok(55 * 2)),
            (06, 66, Ok(66 * 2)),
            (07, 77, Ok(77 * 2)),
            (08, 88, Ok(88 * 2)),
            (09, 99, Ok(99 * 2)),
            (10, -100, Err(Error::new(&dbg, "").err("err"))),
        ];
        let thread_pool = ThreadPool::new(&dbg, Some(threads + threads / 3));
        let scheduler = thread_pool.scheduler();
        let time = Instant::now();
        let result = Arc::new(AtomicUsize::new(0));
        let mut futures = vec![];
        for (step, value, _) in test_data.clone() {
            let dbg_ = Dbg::new(&dbg, format!("thread{step}"));
            let result = result.clone();
            let f: Future<Result<i32, _>> = Future::spawn(scheduler.clone(), move || {
                log::debug!("{dbg_}", );
                std::thread::sleep(Duration::from_secs(1));
                result.fetch_add(1, Ordering::SeqCst);
                if value > 0 {
                    Ok(value * 2)
                } else {
                    Err(Error::new(&dbg_, "").err("err"))
                }
            }).unwrap();
            futures.push(f);
        }
        for (i, future) in futures.into_iter().enumerate() {
            let (step, value, target) = test_data[i].clone();
            let result = future.wait().unwrap();
            log::debug!("{} | step {} value: {} \nresult: {:?}\ntarget: {:?}", dbg, step, value, result, target);
            match (result, target) {
                (Ok(result), Ok(target)) => assert!(result == target, "{} | step {} \nresult: {:?}\ntarget: {:?}", dbg, step, result, target),
                (Ok(result), Err(target)) => panic!("{} | step {} \nresult: {:?}\ntarget: {:?}", dbg, step, result, target),
                (Err(result), Ok(target)) => panic!("{} | step {} \nresult: {:?}\ntarget: {:?}", dbg, step, result, target),
                (Err(_), Err(_)) => {},
            }
        }
        log::debug!("Jobs sheduled: {threads} in: {:?}", time.elapsed());
        std::thread::sleep(Duration::from_millis(100));
        thread_pool.join().unwrap();
        log::debug!("All Jobs done ({threads})");
        log::debug!("Total elapsed: {:?}", time.elapsed());
        let target = test_data.len();
        let result = result.load(Ordering::SeqCst);
        assert!(result == target, "{} \nresult: {:?}\ntarget: {:?}", dbg, result, target);
        test_duration.exit();
    }
}

#[cfg(test)]

mod handles {
    use std::{sync::{Arc, Once}, time::{Duration, Instant}};
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::sync::{channel, Handles};
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
    /// Testing Handles::wait
    #[test]
    fn wait() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("handles.wait");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (01, 100),
            (02, 200),
            (03, 300),
            (04, 400),
            (05, 500),
            (06, 600),
        ];
        let handles = Arc::new(Handles::new(&dbg));
        let handles_clone = handles.clone();
        let (s, r) = channel::unbounded();
        let time = Instant::now();
        let handle = std::thread::spawn(move || {
            for (step, value) in test_data {
                let s_clone = s.clone();
                let handle = std::thread::spawn(move || {
                    s_clone.send(value).unwrap();
                    log::debug!("step {} sent: {:?}", step, value);
                    std::thread::sleep(Duration::from_secs(3));
                });
                handles.push(handle);
            }
            for (step, value) in test_data {
                s.send(value).unwrap();
                log::debug!("step {} sent: {:?}", step, value);
            }
        });
        handles_clone.push(handle);
        handles_clone.wait().unwrap();
        for event in r {
            log::debug!("received: {:?}", event);
        }
        log::debug!("Elapsed: {:?}", time.elapsed());
        test_duration.exit();
    }
}

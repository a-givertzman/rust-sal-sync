#[cfg(test)]

mod kanal_channel {
    use std::{sync::Once, time::{Duration, Instant}};
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::sync::channel;
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
    fn iter() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("kanal_channel::iter");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (01, 100),
            (02, 200),
            (03, 300),
            (04, 400),
            (05, 500),
            (06, 600),
        ];
        let (s, r) = channel::unbounded();
        let time = Instant::now();
        std::thread::spawn(move || {
            for (step, value) in test_data {
                s.send(value).unwrap();
                log::debug!("step {} sent: {:?}", step, value);
            }
            std::thread::sleep(Duration::from_secs(3));
            for (step, value) in test_data {
                s.send(value).unwrap();
                log::debug!("step {} sent: {:?}", step, value);
            }
        });
        for event in r {
            log::debug!("received: {:?}", event);
        }
        log::debug!("Elapsed: {:?}", time.elapsed());
        // for (step, value) in test_data {
        //     let result = r.recv().unwrap();
        //     let target = value;
        //     assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        // }
        test_duration.exit();
    }
}

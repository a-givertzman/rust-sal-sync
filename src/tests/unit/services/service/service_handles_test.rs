#[cfg(test)]

mod service_handles {
    use log::debug;
    use std::{sync::Once, thread::{self, JoinHandle}, time::Duration};
    use testing::stuff::{max_test_duration::TestDuration, wait::WaitTread};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::services::service::service_handles::ServiceHandles;
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
    fn init_each() -> [(usize, &'static str, JoinHandle<usize>); 7] {
        [
            (01, "h1", thread::spawn(|| {01})),
            (02, "h2", thread::spawn(|| {02})),
            (03, "h3", thread::spawn(|| {03})),
            (04, "h4", thread::spawn(|| {04})),
            (05, "h5", thread::spawn(|| {05})),
            (06, "h6", thread::spawn(|| {06})),
            (07, "h7", thread::spawn(|| {07})),
        ]
    }
    ///
    /// Testing ServiceHandles::into_iter
    #[test]
    fn into_iter() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        let test_data = init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data_len = test_data.len();
        let mut handles = ServiceHandles::new(vec![]);
        let mut target_id = vec![];
        for (step, id, handle) in test_data {
            debug!("step {}  |  id: {}  |  target: {:?}", step, id, handle);
            target_id.push((step, id));
            handles.insert(id, handle)
        }
        let result = handles.len();
        let target = test_data_len;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        debug!("target_id: {:#?}", target_id);
        let mut target_id_iter = target_id.into_iter();
        for (result_id, handle) in handles {
            let (step, target_id) = target_id_iter.next().unwrap();
            assert!(result_id == target_id, "step {} \nresult: {:?}\ntarget: {:?}", step, result_id, target_id);
            let result = handle.join().unwrap();
            let target = step;
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
    ///
    /// Testing ServiceHandles::wait
    #[test]
    fn wait() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        let test_data = init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data_len = test_data.len();
        let mut handles = ServiceHandles::new(vec![]);
        for (_, id, handle) in test_data {
            handles.insert(id, handle)
        }
        let result = handles.len();
        let target = test_data_len;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        handles.wait().unwrap();
        test_duration.exit();
    }
}

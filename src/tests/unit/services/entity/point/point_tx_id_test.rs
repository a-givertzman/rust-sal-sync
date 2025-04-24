#[cfg(test)]

mod point_tx_id {
    use log::debug;
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::services::entity::PointTxId;
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
    /// Testing Bool::add
    #[test]
    fn test_add() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (01, "/App/Service/Point1", 5250787495257260775usize),
            (02, "/App/Service/Point2", 2795070213056021901),
            (03, "/App/Service/Point3", 16824025039985537841),
            (04, "/App/Service/Point4", 12383472895625139830),
        ];
        for (step, value, target) in test_data {
            let result = PointTxId::from_str(value);
            debug!("step: {}  |  value: {}, target: {}, result: {}", step, value, target, result);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}

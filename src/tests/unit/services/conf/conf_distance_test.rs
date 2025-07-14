use std::str::FromStr;
#[cfg(test)]

use std::{sync::Once, time::Duration};
use sal_core::dbg::Dbg;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
use crate::services::{conf::{ConfDistance, ConfDistanceUnit}};

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
fn from_str() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    init_each();
    log::debug!("");
    let dbg = Dbg::own("ConfDistance::from_str");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
    test_duration.run().unwrap();
    let test_data = [
        (01, "5.15 m", ConfDistance::new(5.15, ConfDistanceUnit::Meter)), 
        (01, "5.15m", ConfDistance::new(5.15, ConfDistanceUnit::Meter)), 
        (02, "7.23 mm", ConfDistance::new(7.23, ConfDistanceUnit::Millimeter)), 
        (03, "-5.15 in", ConfDistance::new(-5.15, ConfDistanceUnit::Inch)), 
        (03, "+5.15 km", ConfDistance::new(5.15, ConfDistanceUnit::Kilometer)), 
        (03, "5.15 nm", ConfDistance::new(5.15, ConfDistanceUnit::Nanometer)), 
        (03, "5.15 um", ConfDistance::new(5.15, ConfDistanceUnit::Micrometer)), 
        (03, "5.15 mm", ConfDistance::new(5.15, ConfDistanceUnit::Millimeter)), 
        (03, "5.15 cm", ConfDistance::new(5.15, ConfDistanceUnit::Centimetre)), 
        (03, "5.15 m", ConfDistance::new(5.15, ConfDistanceUnit::Meter)), 
        (03, "5.15", ConfDistance::new(5.15, ConfDistanceUnit::Meter)),
        (03, "5.15 km", ConfDistance::new(5.15, ConfDistanceUnit::Kilometer)),
        (03, "5.15 in", ConfDistance::new(5.15, ConfDistanceUnit::Inch)),
    ];
    for (step, conf, target) in test_data {
        let result = ConfDistance::from_str(conf).unwrap();
        assert!(result == target, "{dbg} | step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    }
    test_duration.exit();
}

use std::str::FromStr;
#[cfg(test)]

use std::{sync::Once, time::Duration};
use sal_core::dbg::Dbg;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel};
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
/// Testing ConfDistance
#[test]
fn from_str() {
    DebugSession::new().filter(LogLevel::Info).init();
    init_once();
    init_each();
    log::debug!("");
    let dbg = Dbg::own("ConfDistance::from_str");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
    test_duration.run().unwrap();
    let test_data = [
        (01, "5.15 m", ConfDistance::new(5.15, ConfDistanceUnit::Meter)), 
        (02, "5.15m", ConfDistance::new(5.15, ConfDistanceUnit::Meter)), 
        (03, "7.23 mm", ConfDistance::new(7.23, ConfDistanceUnit::Millimeter)), 
        (04, "-5.15 in", ConfDistance::new(-5.15, ConfDistanceUnit::Inch)), 
        (05, "+5.15 km", ConfDistance::new(5.15, ConfDistanceUnit::Kilometer)), 
        (06, "5.15 nm", ConfDistance::new(5.15, ConfDistanceUnit::Nanometer)), 
        (07, "5.15 um", ConfDistance::new(5.15, ConfDistanceUnit::Micrometer)), 
        (08, "5.15 mm", ConfDistance::new(5.15, ConfDistanceUnit::Millimeter)), 
        (09, "5.15 cm", ConfDistance::new(5.15, ConfDistanceUnit::Centimetre)), 
        (10, "5.15 m", ConfDistance::new(5.15, ConfDistanceUnit::Meter)), 
        (11, "5.15", ConfDistance::new(5.15, ConfDistanceUnit::Meter)),
        (12, "5.15 km", ConfDistance::new(5.15, ConfDistanceUnit::Kilometer)),
        (13, "5.15 in", ConfDistance::new(5.15, ConfDistanceUnit::Inch)),
        (14, "5 mm", ConfDistance::new(5.0, ConfDistanceUnit::Millimeter)),
        (15, "1.4e-3", ConfDistance::new(1.4e-3, ConfDistanceUnit::Meter)),
        (16, "1.5e-6 mm", ConfDistance::new(1.5e-6, ConfDistanceUnit::Millimeter)),
        (17, "1.6e-9cm", ConfDistance::new(1.6e-9, ConfDistanceUnit::Centimetre)),
        (18, "2E+4", ConfDistance::new(2E+4, ConfDistanceUnit::Meter)),
        (19, "2E+6 mm", ConfDistance::new(2E+6, ConfDistanceUnit::Millimeter)),
        (20, "2E+12cm", ConfDistance::new(2E+12, ConfDistanceUnit::Centimetre)),
    ];
    for (step, conf, target) in test_data {
        let result = ConfDistance::from_str(conf).unwrap();
        assert!(result.unit == target.unit, "{dbg} | step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        assert!((result.value - target.value).abs() < f64::EPSILON, "{dbg} | step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    }
    test_duration.exit();
}

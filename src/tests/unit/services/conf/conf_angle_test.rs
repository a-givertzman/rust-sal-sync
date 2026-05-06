use std::str::FromStr;
#[cfg(test)]

use std::{sync::Once, time::Duration};
use sal_core::dbg::Dbg;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel};
use crate::services::{conf::{ConfAngle, ConfAngleUnit}};

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
/// Testing ConfAngle
#[test]
fn from_str() {
    DebugSession::new().filter(LogLevel::Info).init();
    init_once();
    init_each();
    log::debug!("");
    let dbg = Dbg::own("ConfAngle::from_str");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
    test_duration.run().unwrap();
    let test_data = [
        (01, "5.15 deg", ConfAngle::new(5.15, ConfAngleUnit::Degrees)),
        (02, "5.15°", ConfAngle::new(5.15, ConfAngleUnit::Degrees)),
        (03, "0.23rad", ConfAngle::new(0.23, ConfAngleUnit::Radians)),
        (04, "-0.15 rad", ConfAngle::new(-0.15, ConfAngleUnit::Radians)),
        (05, "+5.15 rad", ConfAngle::new(5.15, ConfAngleUnit::Radians)),
        (06, "5.15 gon", ConfAngle::new(5.15, ConfAngleUnit::Gradians)),
        (07, "5.15 %", ConfAngle::new(5.15, ConfAngleUnit::Percent)),
        (08, "5.15%", ConfAngle::new(5.15, ConfAngleUnit::Percent)),
        (09, "5.15e-3", ConfAngle::new(5.15e-3, ConfAngleUnit::Radians)),
        (10, "5.15e-6°", ConfAngle::new(5.15e-6, ConfAngleUnit::Degrees)),
        (11, "5.15e-9 rad", ConfAngle::new(5.15e-9, ConfAngleUnit::Radians)),
        (12, "5E+3", ConfAngle::new(5E+3, ConfAngleUnit::Radians)),
        (13, "5.1E+6°", ConfAngle::new(5.1E+6, ConfAngleUnit::Degrees)),
        (14, "5.0E+12 rad", ConfAngle::new(5.0E+12, ConfAngleUnit::Radians)),
    ];
    for (step, conf, target) in test_data {
        let result = ConfAngle::from_str(conf).unwrap();
        assert!(result.unit == target.unit, "{dbg} | step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        assert!((result.value - target.value).abs() < f64::EPSILON, "{dbg} | step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    }
    test_duration.exit();
}

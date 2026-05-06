#[cfg(test)]

use std::sync::Once;
use debugging::session::debug_session::{DebugSession, LogLevel};
use crate::math::AproxEq;
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
///
#[test]
fn aprox_eq_f32() {
    DebugSession::new().filter(LogLevel::Info).init();
    init_once();
    init_each();
    log::info!("test_f32");

    // let (initial, switches) = init_each();
    let test_data = vec![
        (01, 6, (1.234567000f32, 1.234567110f32)),
        (02, 5, (12.34567000f32, 12.34567110f32)),
        (03, 4, (123.4567000f32, 123.4567110f32)),
        (04, 3, (1234.567000f32, 1234.567110f32)),
        (05, 2, (12345.67999f32, 12345.67890f32)),
        (06, 1, (123456.7999f32, 123456.7890f32)),
        (07, 0, (1234567.000f32, 1234567.110f32)),
        (08, 1, (12345678.90f32, 12345678.91f32)),
        (09, 0, (123456789.0f32, 123456789.1f32)),
    ];
    for (step, decimals, (value, target)) in test_data {
        let aprox_eq = value.aprox_eq(target, decimals);
        log::debug!("step {step}  value: {:?}   |   target: {:?}  |    decimals: {:?}     |   aproxEq: {:?}", value, target, decimals, aprox_eq);
        assert_eq!(aprox_eq, true, "step {step}  value: {:?}   |   target: {:?}  |    decimals: {:?}    |   aproxEq: {:?}", value, target, decimals, aprox_eq);
    }
}

#[test]
fn aprox_eq_f64() {
    DebugSession::new().filter(LogLevel::Info).init();
    init_once();
    init_each();
    log::info!("test_f64");

    // let (initial, switches) = init_each();
    let test_data = vec![
        (01, 16, (1.0123456789123456f64, 1.0123456789123456f64)),
        (02, 15, (12.0123456789123451f64, 12.0123456789123456f64)),
        (03, 14, (123.0123456789123411f64, 123.0123456789123456f64)),
        (04, 13, (1234.0123456789123111f64, 1234.0123456789123456f64)),
        (05, 12, (12345.0123456789121111f64, 12345.0123456789123456f64)),
        (06, 11, (123456.0123456789111111f64, 123456.0123456789123456f64)),
        (07, 10, (1234567.0123456789011111f64, 1234567.0123456789123456f64)),
        (08, 9, (12345678.0123456789011111f64, 12345678.0123456789123456f64)),
        (09, 8, (123456789.0123456789111111f64, 123456789.0123456789123456f64)),
        (10, 7, (1234567890.0123456781111111f64, 1234567890.0123456789123456f64)),
        (11, 6, (12345678901.0123456111111111f64, 12345678901.0123456789123456f64)),
        (12, 5, (123456789012.0123451111111111f64, 123456789012.0123456789123456f64)),
        (13, 4, (1234567890123.0123411111111111f64, 1234567890123.0123456789123456f64)),
        (14, 3, (12345678901234.0123111111111111f64, 12345678901234.0123456789123456f64)),
        (15, 2, (123456789012345.0121111111111111f64, 123456789012345.0123456789123456f64)),
        (16, 1, (1234567890123456.0111111111111111f64, 1234567890123456.0123456789123456f64)),
        (17, 0, (12345678901234567.0111111111111111f64, 12345678901234567.0123456789123456f64)),
        (17, 1, (0.11f64, 0.12f64)),
        (18, 2, (0.111f64, 0.112f64)),
        (20, 3, (0.1111f64, 0.1112f64)),
        (21, 1, (0.55f64, 0.56f64)),
        (22, 2, (0.555f64, 0.556f64)),
        (23, 3, (0.5555f64, 0.5556f64)),
        (24, 1, (0.55f64, 0.56f64)),
        (25, 2, (0.555f64, 0.556f64)),
        (26, 3, (0.5555f64, 0.5556f64)),
        (27, 1, (0.77f64, 0.75f64)),
        (28, 2, (0.777f64, 0.775f64)),
        (29, 3, (0.7777f64, 0.7775f64)),
        (30, 3, (0.7777f64, 0.7779f64)),
        (31, 0, (10.0f64, 9.99f64)),
    ];
    for (step, decimals, (value, target)) in test_data {
        let aprox_eq = value.aprox_eq(target, decimals);
        log::debug!("step {step}  value: {:?}   |   target: {:?}  |    decimals: {:?}     |   aproxEq: {:?}", value, target, decimals, aprox_eq);
        assert_eq!(aprox_eq, true, "step {step}  value: {:?}   |   target: {:?}  |    decimals: {:?}    |   aproxEq: {:?}", value, target, decimals, aprox_eq);
    }
}

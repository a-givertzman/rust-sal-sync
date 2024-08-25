#[cfg(test)]

mod service_cycle {
    use log::{info, warn, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use rand::Rng;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        services::service::service_cycle::ServiceCycle, tests::unit::temp::aprox_eq::AproxEq,
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
    ///
    #[test]
    fn basic() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        println!();
        println!("test ServiceCycle");
        fn load(num: usize) {
            for _ in 0..num {
                let _: u128 = (1..=20).product();
            }
        }
        let test_cycles = 100;
        let mut errors = 0; // a few errors will be ok, but not more then 5% of test cycles
        let errors_allowed = (test_cycles as f64 * 0.20) as usize;
        // const TARGET_CYCLE_INTERVALS: [u64; 4] = [1, 10, 100, 1000];
        // const TARGET_CYCLE_INTERVALS: [u64; 3] = [1, 10, 100];
        const TARGET_CYCLE_INTERVALS: [u64; 2] = [1, 10];
        for target_cycle_interval in TARGET_CYCLE_INTERVALS {  // ms
            let mut max: usize = 10;
            println!();
            let self_id = "service_cycle_test";
            info!("target cycle interval: {} ms", target_cycle_interval);
            let length = target_cycle_interval.checked_ilog10().unwrap_or(0) + 1;
            let digits = 4 - length as usize;
            debug!("length: {:?}", length);
            debug!("aproxEq digits: {:?}", digits);
            info!("detecting load range...");
            let t = Instant::now();
            for _ in 0..9 {
                load(max);
            }
            let elapsed = t.elapsed().as_secs_f64();
            let target_k = ((target_cycle_interval as f64) / 1000.0)  / elapsed;
            max = (max as f64 * 10.0 * 1.2 * target_k) as usize;
            let t = Instant::now();
            load(max);
            info!("load range 1...{:?}", max);
            info!("elapsed for max load: {:?}", t.elapsed());
            let mut cycle = ServiceCycle::new(self_id, Duration::from_millis(target_cycle_interval));
            for _ in 0..test_cycles {
                let num = rand::thread_rng().gen_range(1..max);
                debug!("load: {}", num);
                cycle.start();
                let t = Instant::now();
                load(num);
                let math_elapsed = t.elapsed();
                debug!("math done in: {:?}", math_elapsed.as_secs_f64());
                cycle.wait();
                let cycle_elapsed = t.elapsed();
                debug!("cycle done in: {:?}", cycle_elapsed.as_secs_f64());
                if math_elapsed.as_millis() >= target_cycle_interval.into() {
                    if ! math_elapsed.as_secs_f64().aprox_eq(cycle_elapsed.as_secs_f64(), digits) {
                        errors += 1;
                        warn!(
                            "values must be aprox equals ({} digits): mathElapsed: {:?} != cycleElapsed {:?}",
                            digits,
                            math_elapsed.as_secs_f64(),
                            cycle_elapsed.as_secs_f64(),
                        );
                    }
                } else {
                    let target_in_secs = (target_cycle_interval as f64) / 1000.0;
                    let digits = 4 - length as usize;
                    if ! target_in_secs.aprox_eq(cycle_elapsed.as_secs_f64(), digits) {
                        errors += 1;
                        warn!(
                            "values must be aprox equals ({} digits): targetInSecs: {:?} != cycleElapsed {:?}",
                            digits,
                            target_in_secs,
                            cycle_elapsed.as_secs_f64(),
                        );
                    }
                }
            }
            assert!(errors < errors_allowed, "to much errors ({}), a few errors will be ok, but not more then 5% ({}) of test cycles", errors, errors_allowed);
        }
    }
}
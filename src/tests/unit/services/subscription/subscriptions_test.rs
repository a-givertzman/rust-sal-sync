#[cfg(test)]

mod subscriptions {
    use log::debug;
    use std::{sync::{mpsc, Once}, thread, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::services::{entity::point::point::Point, subscription::subscriptions::Subscriptions};
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
    /// Testing Subscriptions::new
    #[test]
    fn new() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "new";
        debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let mut subscriptions = Subscriptions::new(self_id);
        let mut threads = vec![];
        let mut receivers = vec![];
        let test_receivers = vec![
            101,
            102,
            103,
        ];
        let test_data = [
            (test_receivers[0], vec!["/Destination/Point1", "/Destination/Point2"]),
            (test_receivers[1], vec!["/Destination/Point2"]),
            (test_receivers[2], vec!["/Destination/Point3"]),
        ];
        for (receiver_id, destinations) in test_data.clone() {
            let (send, recv) = mpsc::channel();
            for dest in destinations.clone() {
                subscriptions.add_multicast(receiver_id, dest, send.clone());
            }
            receivers.push(
                (receiver_id, destinations, recv)
            )
        }
        for (receiver_id, mut destinations, recv) in receivers.into_iter() {
            let handle = thread::spawn(move || {
                // let recv = recv.read().unwrap();
                debug!("receiver_id {} destinations: {:?}", receiver_id, destinations);
                loop {
                    match recv.recv_timeout(Duration::from_millis(100)) {
                        Ok(result) => {
                            debug!("receiver_id {} received: {:?}:{:?}", receiver_id, result.name(), result.value());
                            let point_id = result.name();
                            destinations.retain(|dest| dest.to_owned() != point_id);
                        }
                        Err(err) => match err {
                            mpsc::RecvTimeoutError::Timeout => panic!("{}.receive_thread | Not received points: {:?}", self_id, destinations),
                            mpsc::RecvTimeoutError::Disconnected => panic!("{}.receive_thread | Receive error for receiver_id {}", self_id, receiver_id),
                        },
                    };
                    if destinations.is_empty() {
                        break;
                    }
                }
            });
            threads.push(handle);
        }
        let mut value = 0.1f64;
        for (_, destinations) in test_data {
            for point_id in destinations {
                for (receiver_id, subscriber) in subscriptions.iter(point_id) {
                    let point = Point::new(0, &point_id, value);
                    value += 1.0;
                    subscriber.send(point.clone()).unwrap();
                    debug!("receiver_id {} point_id: {} sent: {:?}", receiver_id, point_id, point.value());
                }
            }
        }
        for th in threads {
            th.join().unwrap();
        }
        subscriptions.exit();
        test_duration.exit();
    }
}

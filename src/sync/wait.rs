use sal_core::error::Error;

///
/// wait method for some entity
/// - works like join on thread, blocking method call
pub trait Wait<T> {
    fn wait(self) -> Result<T, Error>;
}

///
/// wait method for boxed entity
/// - works like join on thread, blocking method call
pub trait WaitBox<T>: Send {
    fn wait(self: Box<Self>) -> Result<T, Error>;
}

// impl<T> Wait<T> for std::thread::JoinHandle<T> {
//     fn wait(self) -> Result<T, Error> {
//         self
//             .join()
//             .map_err(|err| Error::new("JoinHandle<T>", "wait").pass(format!("{:?}", err)))
//     }
// }
impl<T> Wait<T> for std::thread::JoinHandle<T> {
    fn wait(self) -> Result<T, Error> {
        self
            .join()
            .map_err(|err| Error::new("JoinHandle<T>", "wait").pass(format!("{:?}", err)))
    }
}

// impl<T> Wait<T> for crate::thread_pool::JoinHandle<T> {
//     fn wait(self) -> Result<T, Error> {
//         self.join()
//     }
// }

impl<T> Wait<T> for crate::thread_pool::JoinHandle<T> {
    fn wait(self) -> Result<T, Error> {
        self.join()
    }
}

impl<T: Send> WaitBox<T> for crate::thread_pool::JoinHandle<T> {
    fn wait(self: Box<Self>) -> Result<T, Error> {
        self.join()
    }
}
impl<T> WaitBox<T> for std::thread::JoinHandle<T> {
    fn wait(self: Box<Self>) -> Result<T, Error> {
        self
            .join()
            .map_err(|err| Error::new("JoinHandle<T>", "wait").pass(format!("{:?}", err)))
    }
}

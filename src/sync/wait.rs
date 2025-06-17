use sal_core::error::Error;

///
/// wait method for some entity
/// - works like join on thread, blocking method call
pub trait Wait<T> {
    ///
    /// Gets the thread's name.
    fn name(&self) -> String;
    ///
    /// Waits for the associated thread to finish.
    fn wait(self) -> Result<T, Error>;
}

///
/// wait method for boxed entity
/// - works like join on thread, blocking method call
pub trait WaitBox<T>: Send {
    ///
    /// Gets the thread's name.
    fn name(&self) -> String;
    ///
    /// Waits for the associated thread to finish.
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
    fn name(&self) -> String {
        self.thread().name().unwrap_or("").to_owned()
    }
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
    fn name(&self) -> String {
        self.name()
    }
    fn wait(self) -> Result<T, Error> {
        self.join()
    }
}

impl<T: Send> WaitBox<T> for crate::thread_pool::JoinHandle<T> {
    fn wait(self: Box<Self>) -> Result<T, Error> {
        self.join()
    }
    fn name(&self) -> String {
        self.name()
    }
}
impl<T> WaitBox<T> for std::thread::JoinHandle<T> {
    fn wait(self: Box<Self>) -> Result<T, Error> {
        self
            .join()
            .map_err(|err| Error::new("JoinHandle<T>", "wait").pass(format!("{:?}", err)))
    }
    fn name(&self) -> String {
        self.thread().name().unwrap_or("").to_owned()
    }
}

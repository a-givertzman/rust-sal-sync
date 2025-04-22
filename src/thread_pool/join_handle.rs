use sal_core::error::Error;

pub struct JoinHandle<T> {
    recv: kanal::Receiver<T>,
}
//
//
impl<T> JoinHandle<T> {
    ///
    /// Returns [JoinHandle] new instance
    pub fn new(recv: kanal::Receiver<T>) -> Self {
        Self { recv }
    }
    ///
    /// Waits for the associated thread to finish.
    /// 
    /// This function will return immediately if the associated thread has already finished.
    /// 
    /// In terms of [atomic memory orderings], the completion of the associated thread synchronizes with this function returning. In other words, all operations performed by that thread happen before all operations that happen after join returns.
    /// 
    /// If the associated thread panics, [Err] is returned with the parameter given to panic (though see the Notes below).
    pub fn join(self) -> Result<T, Error> {
        match self.recv.recv() {
            Ok(v) => Ok(v),
            Err(err) => Err(Error::new("JoinHandle", "join").err(err.to_string())),
        }
    }
}
use sal_core::error::Error;
///
/// Provides to join on a thread (block on its termination).
/// Returns `id` and `name` of associated thread

pub struct JoinHandle<T> {
    id: Option<String>,
    name: Option<String>,
    recv: oneshot::Receiver<T>,
}
//
//
impl<T> JoinHandle<T> {
    ///
    /// Returns [JoinHandle] new instance
    pub fn new(id: Option<impl Into<String>>, name: Option<impl Into<String>>, recv: oneshot::Receiver<T>) -> Self {
        Self {
            id: id.map(|v| v.into()),
            name: name.map(|v| v.into()),
            recv,
        }
    }
    ///
    /// Gets the thread's unique identifier.
    pub fn id(&self) -> String {
        self.id.as_ref().map_or("".to_string(), |v| v.clone())
    }
    /// 
    /// Gets the thread's name.
    pub fn name(&self) -> String {
        self.name.as_ref().map_or("".to_string(), |v| v.clone())
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
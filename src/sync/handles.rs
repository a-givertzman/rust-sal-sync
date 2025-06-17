use std::sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc};
use coco::Stack;
use sal_core::{dbg::{self, dbg, Dbg}, error::Error};
use crate::sync::WaitBox;

///
/// Holds one or multiple `JoinHandle`'s as `dyn WaitBox`
/// - Thread safe
pub struct Handles<T> {
    dbg: Dbg,
    len: AtomicUsize,
    handle: Stack<Box<dyn WaitBox<T>>>,
    is_finished: Arc<AtomicBool>,
}
//
//
impl<T> Handles<T> {
    ///
    /// Returns [Handles] new instance
    pub fn new(parent: &Dbg) -> Self {
        Self {
            dbg: Dbg::new(parent, "Handles"),
            len: AtomicUsize::new(0),
            handle: Stack::new(),
            is_finished: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn push(&self, handle: impl WaitBox<T> + 'static) {
        self.handle.push(Box::new(handle));
        self.len.fetch_add(1, Ordering::SeqCst);
    }
    ///
    /// Returns `true` if all JoinHandles are joined
    pub fn is_finished(&self) -> bool {
        self.is_finished.load(Ordering::SeqCst)
    }
    ///
    /// Waits for all JoinHandles being joined
    /// - Blocking call
    #[dbg]
    pub fn wait(&self) -> Result<(), Error> {
        let mut index = 0;
        while !self.handle.is_empty() {
            if let Some(handle) = self.handle.pop() {
                index += 1;
                // let id = handle.id();
                let name = handle.name();
                dbg::debug!("Waiting for {index} of {} ('{}')...", self.len.load(Ordering::SeqCst), name);
                if let Err(err) = handle.wait() {
                    dbg::warn!("Error: {:?}", err);
                    return Err(Error::new(&self.dbg, "wait").pass_with(format!("Error on {index} of {} ('{}')", self.len.load(Ordering::SeqCst), name), err.to_string()));
                }
                dbg::info!("Finished {index} of {} ('{}')", self.len.load(Ordering::SeqCst), name);
            }
        }
        self.is_finished.store(true, Ordering::SeqCst);
        Ok(())
    }
}

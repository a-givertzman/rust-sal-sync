use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use coco::Stack;
use sal_core::{dbg::{self, dbg, Dbg}, error::Error};
use crate::sync::WaitBox;

///
/// Holds one or multiple `JoinHandle`'s as `dyn WaitBox`
/// - Thread safe
pub struct Handles<T> {
    dbg: Dbg,
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
            handle: Stack::new(),
            is_finished: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn push(&self, handle: impl WaitBox<T> + 'static) {
        self.handle.push(Box::new(handle));
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
        while !self.handle.is_empty() {
            if let Some(handle) = self.handle.pop() {
                if let Err(err) = handle.wait() {
                    dbg::warn!("Error: {:?}", err);
                    return Err(Error::new(&self.dbg, "wait").err(format!("{:?}", err)));
                }
            }
        }
        self.is_finished.store(true, Ordering::SeqCst);
        Ok(())
    }
}
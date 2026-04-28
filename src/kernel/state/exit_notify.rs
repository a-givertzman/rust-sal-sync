use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use sal_core::dbg::Dbg;
///
/// - Contains local/parents's [exit] signal
/// - Contains partner's [exit_pair] signal
/// - If [exit] is true, service exits main thread
/// - Rase [exit_pair] to true when partner service must exit main thread
pub struct ExitNotify {
    #[allow(unused)]
    id: Dbg,
    exit: Arc<AtomicBool>,
    exit_pair: Arc<AtomicBool>,
    exit_parent: Arc<AtomicBool>,
}
//
//
impl ExitNotify {
    ///
    /// Creates new instance of the ExitNotify
    pub fn new(
        parent: impl Into<String>,
        exit: Option<Arc<AtomicBool>>,
        exit_pair: Option<Arc<AtomicBool>>,
    ) -> Self {
        Self {
            id: Dbg::new(parent, "ExitNotify"),
            exit: Arc::new(AtomicBool::new(false)),
            exit_pair: exit_pair.unwrap_or(Arc::new(AtomicBool::new(false))),
            exit_parent: exit.unwrap_or(Arc::new(AtomicBool::new(false))),
        }
    }
    ///
    /// Returns true if exit signal exists localy or from the partner
    pub fn get(&self) -> bool {
        self.exit.load(Ordering::Acquire) ||
        self.exit_pair.load(Ordering::Acquire) ||
        self.exit_parent.load(Ordering::Acquire)
    }
    ///
    /// Sends exit signal localy only
    pub fn exit(&self) {
        self.exit.store(true, Ordering::Release);
    }
    ///
    /// Sends exit signal to the partner only
    pub fn exit_pair(&self) {
        self.exit_pair.store(true, Ordering::Release);
    }
    ///
    /// Sends exit signal localy and to the partner
    pub fn exit_all(&self) {
        self.exit_pair.store(true, Ordering::Release);
        self.exit.store(true, Ordering::Release);
    }
    ///
    /// Resets all exit signals
    pub fn reset_all(&self) {
        self.exit_pair.store(false, Ordering::Release);
        self.exit.store(false, Ordering::Release);
    }
    ///
    /// Resets local/parent exit signal
    pub fn reset(&self) {
        self.exit.store(false, Ordering::Release);
    }
    ///
    /// Resets partner exit signal
    pub fn reset_pair(&self) {
        self.exit_pair.store(false, Ordering::Release);
    }
}

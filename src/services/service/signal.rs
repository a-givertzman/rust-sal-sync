///
/// Aplication system signals
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Signal {
    /// Continue normal appliacation execution
    Continue,
    /// Graceful Shutdown
    Exit,
}

///
/// Execution job or shutdown
pub enum Job {
    Task(Box<dyn FnOnce() + Send + 'static>),
    // Shutdown,
}

use sal_core::error::Error;
///
/// Execution job or shutdown
pub enum Job {
    Task(Box<dyn FnOnce() -> Result<(), Error> + Send + 'static>),
    Shutdown,
}

use sal_core::error::Error;
///
/// Execution job
pub type Job = Box<dyn FnOnce() -> Result<(), Error> + Send + 'static>;
// pub enum Job {
//     Task(Box<dyn FnOnce() -> Result<(), Box<dyn std::error::Error>> + Send + 'static>),
//     Shutdown,
// }

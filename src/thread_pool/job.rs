pub type Job = Box<dyn FnOnce() + Send + 'static>;
// pub enum Job {
//     Task(Box<dyn FnOnce() -> Result<(), Box<dyn std::error::Error>> + Send + 'static>),
//     Shutdown,
// }

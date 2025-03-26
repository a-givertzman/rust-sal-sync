///
/// General kinds of configuration entities 
/// 
/// Optionally used instead of strings to specify a `kind`
/// in the queries to the `ServiceConf`
/// 
/// Strings are also posible, 
/// but Enum more convenient
/// 
/// Note: Configuration keyword konsists of 4 fields:
/// ```ignore
/// | prefix |  kind  |  name     | sufix     |
/// |        |        |           |           |
/// |--------|--------|-----------|-----------|
/// | opt    | requir |  requir   |  opt      |
/// |--------|--------|-----------|-----------|
/// |        | task   | Task      | Task1     |
/// |        | service| ApiClient | ApiClient |
/// | in     | queue  | in-queue  |           |
/// | out    | queue  | out-queue |           |
/// ````
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfKind {
    Task,
    Service,
    Queue,
    Link,
}
//
//
impl From<ConfKind> for String {
    fn from(kind: ConfKind) -> Self {
        String::from(match kind {
            ConfKind::Task => "task",
            ConfKind::Service => "service",
            ConfKind::Queue => "queue",
            ConfKind::Link => "link",
        })
    }
}
//
//
impl From<ConfKind> for &str {
    fn from(kind: ConfKind) -> Self {
        match kind {
            ConfKind::Task => "task",
            ConfKind::Service => "service",
            ConfKind::Queue => "queue",
            ConfKind::Link => "link",
        }
    }
}

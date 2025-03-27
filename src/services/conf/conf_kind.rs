use sal_core::error::Error;

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
        Into::<&str>::into(kind).to_owned()
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
//
//
impl TryFrom<String> for ConfKind {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "task" => Ok(ConfKind::Task),
            "service" => Ok(ConfKind::Service),
            "queue" => Ok(ConfKind::Queue),
            "link" => Ok(ConfKind::Link),
            _ => Err(Error::new("ConfKind", "try_from").err(format!("Unknown variant: `{}`", value)))
        }
    }
}
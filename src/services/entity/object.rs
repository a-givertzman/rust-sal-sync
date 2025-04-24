use crate::services::entity::Name;
///
/// Interface for Service's object
pub trait Object {
    ///
    /// Returns Object's name
    fn name(&self) -> Name;
}
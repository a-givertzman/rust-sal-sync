use crate::services::entity::name::Name;
///
/// Interface for Service's object
pub trait Object {
    ///
    /// Returns Object's debug id
    fn id(&self) -> &str;
    ///
    /// Returns Object's name
    fn name(&self) -> Name;
}
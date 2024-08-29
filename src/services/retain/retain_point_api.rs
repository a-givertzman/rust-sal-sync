use serde::{Deserialize, Serialize};
///
/// The databases table conf parameters to store/load Point's Id's 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetainPointConfApi {
    pub table: String,
    pub address: String,
    pub auth_token: String,
    pub database: String,
}
//
//
impl RetainPointConfApi {
    ///
    /// 
    pub fn new(table: impl Into<String>, address: impl Into<String>, auth_token: impl Into<String>, database: impl Into<String>) -> Self {
        Self { 
            table: table.into(),
            address: address.into(),
            auth_token: auth_token.into(),
            database: database.into()
        }
    }
}
//
//
impl Default for RetainPointConfApi {
    ///
    /// **Returns `RetainPointConfApi` with the default walues**
    /// 
    /// ```
    /// RetainPointConfApi {
    ///    table: "public.tags",
    ///    address: "0.0.0.0:8080",
    ///    auth_token: "123!@#",
    ///    database: "crane_data_server",
    /// }
    /// ```
    fn default() -> Self {
        Self {
            table: "public.tags".to_owned(),
            address: "0.0.0.0:8080".to_owned(),
            auth_token: "123!@#".to_owned(),
            database: "crane_data_server".to_owned(),
        }
    }
}

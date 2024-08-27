use serde::{Deserialize, Serialize};
///
/// Table parameters to acces and store Point's Id's into the databases table
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetainPointApi {
    pub table: String,
    pub address: String,
    pub auth_token: String,
    pub database: String,
}
//
//
impl RetainPointApi {
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
impl Default for RetainPointApi {
    ///
    /// **Returns `RetainPointIdTable` with the default walues**
    /// 
    /// ```
    /// RetainPointApi {
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
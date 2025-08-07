use serde::{Serialize, Deserialize};
///
/// Represents a list of [Point] configuration types
/// - Bool
/// - Bytes
/// - Int
/// - Real
/// - Double
/// - String
/// - Json
pub type PointType = PointConfType;
///
/// Represents a list of [Point] configuration types
/// - Bool
/// - Bytes
/// - Int
/// - Real
/// - Double
/// - String
/// - Json
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PointConfType {
    #[serde(rename = "Bool")]
    #[serde(alias = "bool", alias = "Bool")]
    Bool,
    #[serde(rename = "Bytes")]
    #[serde(alias = "bytes", alias = "Bytes")]
    Bytes,
    #[serde(rename = "Int")]
    #[serde(alias = "int", alias = "Int")]
    Int,
    #[serde(rename = "Real")]
    #[serde(alias = "real", alias = "Real")]
    Real,
    #[serde(rename = "Double")]
    #[serde(alias = "double", alias = "Double")]
    Double,
    #[serde(rename = "String")]
    #[serde(alias = "string", alias = "String")]
    String,
    #[serde(rename = "Json")]
    #[serde(alias = "json", alias = "Json")]
    Json,
}
//
//
impl ToString for PointConfType {
    fn to_string(&self) -> String {
        match self {
            PointConfType::Bool => "Bool".to_owned(),
            PointConfType::Bytes => "Bytes".to_owned(),
            PointConfType::Int => "Int".to_owned(),
            PointConfType::Real => "Real".to_owned(),
            PointConfType::Double => "Double".to_owned(),
            PointConfType::String => "String".to_owned(),
            PointConfType::Json => "Json".to_owned(),
        }
    }
}

use serde::{Serialize, Deserialize};

#[deprecated(note="Use PointType instead")]
pub type PointConfType = PointType;
///
/// Represents a list of `Point` configuration types
/// - Bool
/// - Bytes
/// - Int
/// - Real
/// - Double
/// - String
/// - Json
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PointType {
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
impl ToString for PointType {
    fn to_string(&self) -> String {
        match self {
            PointType::Bool => "Bool".to_owned(),
            PointType::Bytes => "Bytes".to_owned(),
            PointType::Int => "Int".to_owned(),
            PointType::Real => "Real".to_owned(),
            PointType::Double => "Double".to_owned(),
            PointType::String => "String".to_owned(),
            PointType::Json => "Json".to_owned(),
        }
    }
}

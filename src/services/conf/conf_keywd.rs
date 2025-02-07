use std::str::FromStr;
use regex::RegexBuilder;
use serde::Deserialize;
use crate::services::entity::error::str_err::StrErr;
///
/// # Configuration keyword konsists of 4 fields:
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
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct ConfKeywd {
    pub prefix: String,
    pub kind: String,
    pub name: String,
    pub sufix: String,
}
//
// 
impl ConfKeywd {
    ///
    /// Returns prefix
    /// ```markdown
    /// | opt        | requir   |  requir     |  opt      |
    /// | ---------- | -------- | ----------- | --------- |
    /// | **prefix** | kind     | Name        | Sufix     |
    /// ```
    pub fn prefix(&self) -> String {
        self.prefix.clone()
    }
    ///
    /// Returns prefix
    /// ```markdown
    /// | opt        | requir   |  requir     |  opt      |
    /// | ---------- | -------- | ----------- | --------- |
    /// | prefix     | **kind** | Name        | Sufix     |
    /// ```
    pub fn kind(&self) -> String {
        self.kind.clone()
    }
    ///
    /// Returns prefix
    /// ```markdown
    /// | opt        | requir   |  requir     |  opt      |
    /// | ---------- | -------- | ----------- | --------- |
    /// | prefix     | kind     | **Name**    | Sufix     |
    /// ```
    pub fn name(&self) -> String {
        self.name.clone()
    }
    ///
    /// Returns prefix
    /// ```markdown
    /// | opt        | requir   |  requir     |  opt      |
    /// | ---------- | -------- | ----------- | --------- |
    /// | prefix     | kind     | Name        | **Sufix** |
    /// ```
    pub fn sufix(&self) -> String {
        self.sufix.clone()
    }
}
//
// 
impl FromStr for ConfKeywd {
    type Err = StrErr;
    fn from_str(input: &str) -> Result<Self, StrErr> {
        log::trace!("ConfKeywd.from_str | input: {}", input);
        let re = r#"(?:(?:(\w+)[ \t])?(task|service|queue|link){1}(?:$|(?:[ \t](\S+)(?:[ \t](\S+))?)))"#;
        let re = RegexBuilder::new(re).multi_line(false).build().unwrap();
        let group_prefix = 1;
        let group_kind = 2;
        let group_name = 3;
        let group_sufix = 4;
        match re.captures(input) {
            Some(caps) => {
                let prefix = match &caps.get(group_prefix) {
                    Some(first) => String::from(first.as_str()),
                    None => String::new(),
                };
                let kind = match &caps.get(group_kind) {
                    Some(kind) => Ok(kind.as_str().to_owned()),
                    None => Err(StrErr(format!("ConfKeywd.from_str | Error parsing `kind` from keyword '{}'", &input))),
                }?;
                let name = match &caps.get(group_name) {
                    Some(arg) => Ok(arg.as_str().to_string()),
                    None => Err(StrErr(format!("ConfKeywd.from_str | Error parsing `name` from keyword '{}'", &input))),
                }?;
                let sufix = match &caps.get(group_sufix) {
                    Some(first) => String::from(first.as_str()),
                    None => String::new(),
                };
                Ok(Self {
                    prefix,
                    kind,
                    name,
                    sufix,
                })
            }
            None => {
                Err(StrErr(format!("ConfKeywd.from_str | Pattern `prefix Kinde Name sufix` - not found in keyword '{}'", &input)))
            }
        }
    }
}

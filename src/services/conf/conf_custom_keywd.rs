use std::str::FromStr;
use regex::RegexBuilder;
use sal_core::error::Error;
use serde::Deserialize;
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
pub struct ConfCustomKeywd {
    pub prefix: String,
    pub keywd: String,
    pub sufix: String,
}
//
// 
impl ConfCustomKeywd {
    ///
    /// Returns prefix field
    /// ```markdown
    /// | opt        |  requir     |  opt      |
    /// | ---------- | ----------- | --------- |
    /// | **prefix** | keywd       | Sufix     |
    /// ```
    pub fn prefix(&self) -> String {
        self.prefix.clone()
    }
    ///
    /// Returns `keywd` field
    /// ```markdown
    /// | opt        | requir      |  opt      |
    /// | ---------- | ----------  | --------- |
    /// | prefix     | **keywd**   | Sufix     |
    /// ```
    pub fn keywd(&self) -> String {
        self.keywd.clone()
    }
    ///
    /// Returns `sufix` field
    /// ```markdown
    /// | opt        |  requir     |  opt      |
    /// | ---------- | ----------- | --------- |
    /// | prefix     | keywd       | **Sufix** |
    /// ```
    pub fn sufix(&self) -> String {
        self.sufix.clone()
    }
}
//
// 
impl FromStr for ConfCustomKeywd {
    type Err = Error;
    ///
    /// Returns [ConfCustomKeywd] from fields
    /// ```ignore
    /// | prefix |  keywd    | sufix     |
    /// |--------|-----------|-----------|
    /// | opt    |  requir   |  opt      |
    /// |--------|-----------|-----------|
    /// |        | camera    | Camera1   |
    /// | in     | queue     |           |
    /// ```
    fn from_str(input: &str) -> Result<Self, Error> {
        let error = Error::new("ConfCustomKeywd", "from_str");
        log::trace!("ConfCustomKeywd.from_str | input: {}", input);
        let re = r#"(?:(?:(\w+)[ \t])?(\w+)(?:$|(?:[ \t](\S+)(?:[ \t](\S+))?)))"#;
        let re = RegexBuilder::new(re).multi_line(false).build().unwrap();
        let group_prefix = 1;
        let group_keywd = 2;
        let group_sufix = 3;
        match re.captures(input) {
            Some(caps) => {
                let prefix = match &caps.get(group_prefix) {
                    Some(first) => String::from(first.as_str()),
                    None => String::new(),
                };
                let keywd = match &caps.get(group_keywd) {
                    Some(arg) => Ok(arg.as_str().to_string()),
                    None => Err(error.err(format!("Error parsing required `keywd` field from keyword '{}'", &input))),
                }?;
                let sufix = match &caps.get(group_sufix) {
                    Some(first) => String::from(first.as_str()),
                    None => String::new(),
                };
                Ok(Self {
                    prefix,
                    keywd,
                    sufix,
                })
            }
            None => {
                Err(error.err(format!("Pattern `prefix keywd Sufix` - not found in keyword '{}'", &input)))
            }
        }
    }
}

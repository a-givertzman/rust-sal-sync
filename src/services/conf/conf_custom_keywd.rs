use std::str::FromStr;
use regex::Regex;
use sal_core::error::Error;
use serde::Deserialize;
///
/// # Configuration keyword konsists of 3 fields:
/// ```ignore
/// | prefix |  name     | sufix     |
/// |--------|-----------|-----------|
/// | opt    |  requir   |  opt      |
/// |--------|-----------|-----------|
/// |        | camera    | Camera1   |
/// |        | ApiClient | ApiClient |
/// | in     | queue     |           |
/// ````
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct ConfCustomKeywd {
    prefix: String,
    name: String,
    title: String,
}
//
// 
impl ConfCustomKeywd {
    ///
    /// Returns [ConfCustomKeywd] new instance
    pub fn new(prefix: impl Into<String>, name: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            name: name.into(),
            title: title.into(),
        }
    }
    ///
    /// Returns prefix field
    /// ```markdown
    /// | opt        |  requir    |  opt      |
    /// | ---------- | ---------- | --------- |
    /// | **prefix** | name       | Title     |
    /// ```
    pub fn prefix(&self) -> String {
        self.prefix.clone()
    }
    ///
    /// Returns `name` field
    /// ```markdown
    /// | opt        | requir     |  opt      |
    /// | ---------- | ---------  | --------- |
    /// | prefix     | **name**   | Title     |
    /// ```
    pub fn name(&self) -> String {
        self.name.clone()
    }
    ///
    /// Returns `sufix` field
    /// ```markdown
    /// | opt        |  requir    |  opt      |
    /// | ---------- | ---------- | --------- |
    /// | prefix     | name       | **Title** |
    /// ```
    pub fn title(&self) -> String {
        self.title.clone()
    }
}
//
// 
impl FromStr for ConfCustomKeywd {
    type Err = Error;
    ///
    /// Returns [ConfCustomKeywd] from fields
    /// ```ignore
    /// | prefix |  name     | sufix     |
    /// |--------|-----------|-----------|
    /// | opt    |  requir   |  opt      |
    /// |--------|-----------|-----------|
    /// |        | camera    | Camera1   |
    /// | in     | queue     |           |
    /// ```
    fn from_str(input: &str) -> Result<Self, Error> {
        let error = Error::new("ConfCustomKeywd", "from_str");
        log::trace!("ConfCustomKeywd.from_str | input: {}", input);
        let re = r#"^(?:([^ ]+)[ \t]+)??(?:(\w+)(?:[ \t]+(\S+))?$)"#;
        let re = Regex::new(re).unwrap();
        // let re = RegexBuilder::new(re)..multi_line(false).build().unwrap();
        let group_prefix = 1;
        let group_name = 2;
        let group_title = 3;
        match re.captures(input) {
            Some(caps) => {
                let prefix = match &caps.get(group_prefix) {
                    Some(first) => String::from(first.as_str()),
                    None => String::new(),
                };
                let name = match &caps.get(group_name) {
                    Some(arg) => Ok(arg.as_str().to_string()),
                    None => Err(error.err(format!("Error parsing required `name` field from keyword '{}'", &input))),
                }?;
                let title = match &caps.get(group_title) {
                    Some(first) => String::from(first.as_str()),
                    None => String::new(),
                };
                Ok(Self {
                    prefix,
                    name,
                    title,
                })
            }
            None => {
                Err(error.err(format!("Pattern `prefix name Title` - not found in keyword '{}'", &input)))
            }
        }
    }
}

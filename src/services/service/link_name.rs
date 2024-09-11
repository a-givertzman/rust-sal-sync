use std::str::FromStr;
use concat_in_place::strcat;
///
/// Contains Name of the Link (Channel / Queue) in the separate format
/// Service.Link -> Service & Link
#[derive(Debug, Clone, PartialEq)]
pub struct LinkName {
    name: String,
    service: String,
    link: String,
}
///
/// Contains the Service's Link name in the format 'Servece.Link'
impl LinkName {
    ///
    /// Creates new instance of the LinkName from the string like "Service.Link"
    pub fn new(service: impl Into<String>, link: impl Into<String>) -> Self {
        let service = service.into();
        let link = link.into();
        Self {
            name: strcat!(service.as_str() "." link.as_str()),
            service,
            link,
        }
    }
    ///
    /// Returns original name
    pub fn name(&self) -> String {
        self.name.clone()
    }
    ///
    /// Returns splitted Service and Link
    pub fn split(&self) -> (String, String) {
        (self.service.clone(), self.link.clone())
    }
    ///
    /// Returns the Service name
    pub fn service(&self) -> String {
        self.service.clone()
    }
    ///
    /// Returns the Service's Link name
    pub fn link(&self) -> String {
        self.link.clone()
    }
}
//
//
impl std::fmt::Display for LinkName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
//
//
impl FromStr for LinkName {
    type Err = String;
    ///
    /// Creates new instance of the LinkName from the string like "Service.Link"  
    /// Spliting name 'Service.Link' into 
    /// - Service
    /// - Link
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = name.split('.').collect();
        match parts.first() {
            Some(service) => {
                match parts.get(1) {
                    Some(link) => {
                        Ok(Self { name: name.to_owned(), service: service.to_string(), link: link.to_string() })
                    }
                    None => Err(format!("LinkName.from_str | '{}' does not have structure 'Service.Link'", name)),
                }
            }
            None => Err(format!("LinkName.from_str | '{}' does not have structure 'Service.Link'", name)),
        }
    }
}
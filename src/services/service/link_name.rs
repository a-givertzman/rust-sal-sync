use std::cell::RefCell;
///
/// Contains Name of the Link (Channel / Queue) in the separate format
/// Service.Link -> Service & Link
#[derive(Debug, Clone, PartialEq)]
pub struct LinkName {
    name: String,
    service: RefCell<Option<String>>,
    link: RefCell<Option<String>>,
}
///
/// Contains the Service's Link name in the format 'Servece.Link'
impl LinkName {
    ///
    /// Creates new instance of the LinkName from the string like "Service.Link"
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            service: RefCell::new(None),
            link: RefCell::new(None),
        }
    }
    ///
    /// Returns original name
    pub fn name(&self) -> String {
        self.name.clone()
    }
    ///
    /// Splits name 'Service.Link' into 
    /// - Service
    /// - Link
    fn split_(&self) -> Result<(), String> {
        let parts: Vec<&str> = self.name.split('.').collect();
        match parts.first() {
            Some(value) => {
                self.service.borrow_mut().replace(value.to_owned().to_owned());
                match parts.get(1) {
                    Some(value) => {
                        self.link.borrow_mut().replace(value.to_owned().to_owned());
                        Ok(())
                    }
                    None => Err(format!("LinkName.split_ | '{}' does not have structure 'Service.Link'", self.name)),
                }
            }
            None => Err(format!("LinkName.split_ | '{}' does not have structure 'Service.Link'", self.name)),
        }
    }
    ///
    /// Returns splitted Service and Link names
    pub fn split(&self) -> Result<(String, String), String> {
        if self.service.borrow().is_none() || self.link.borrow().is_none() {
            if let Err(err) = self.split_() {
                return Err(err);
            };
        }
        match self.service.borrow().clone() {
            Some(service) => {
                match self.link.borrow().clone() {
                    Some(link) => {
                        Ok((service, link))
                    }
                    None => Err(format!("LinkName.split | Part 'Link' not found in the '{}'", self.name)),
                }
            }
            None => Err(format!("LinkName.split | Part 'service' not found in the '{}'", self.name)),
        }
    }
    ///
    /// Returns the Service name
    pub fn service(&self) -> Result<String, String> {
        if self.service.borrow().is_none() {
            if let Err(err) = self.split_() {
                return Err(err);
            };
        }
        match self.service.borrow().clone() {
            Some(service) => Ok(service),
            None => Err(format!("LinkName.service | Part 'service' not found in the '{}'", self.name)),
        }
    }
    ///
    /// Returns the Service's Link name
    pub fn link(&self) -> Result<String, String> {
        if self.link.borrow().is_none() {
            if let Err(err) = self.split_() {
                return Err(err);
            };
        }
        match self.link.borrow().clone() {
            Some(link) => Ok(link),
            None => Err(format!("LinkName.link | Part 'Link' not found in the '{}'", self.name)),
        }
    }
    ///
    /// Try to split, panics if split() returns err
    pub fn validate(self) -> Self {
        if let Err(err) = self.split() {
            panic!("LinkName.validate | Error: '{}'", err)
        }
        self
    }
}
//
//
impl std::fmt::Display for LinkName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
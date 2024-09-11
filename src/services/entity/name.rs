use std::fmt::{Debug, Display};
use concat_string::concat_string;
///
/// Creates Point name from parts, dividing parts with single "/" character
/// - path1 + "" = /path1
/// - path1 + path2 = /path1/path2
/// - path1/ + path2 = /path1/path2
/// - path1 + /path2 = /path1/path2
/// - path1/ + /path2 = /path1/path2
#[derive(Clone)]
pub struct Name {
    parent: String,
    me: String,
    joined: String,
}
//
// 
impl Name {
    ///
    /// Creates new instance of Name from 'parent' and 'me'
    pub fn new(parent: impl Into<String>, me: impl Into<String>) -> Self {
        let parent = parent.into();
        let me = me.into();
        let joined = Self::joined_(&parent, &me);
        Self {
            parent,
            me,
            joined,
        }
    }
    ///
    /// Returns joined as '/parent/me
    fn joined_(parent: &str, me: &str) -> String {
        let parent = match parent.chars().next() {
            Some(parent_first) => {
                if parent_first == '/' {
                    parent.to_owned()
                } else {
                    concat_string!("/", parent)
                }
            }
            None => {
                "/".to_owned()
            }
        };
        match parent.chars().last() {
            Some(parent_last) => {
                match me.chars().next() {
                    Some(me_first) => {
                        if parent_last == '/' && me_first == '/' {
                            concat_string!(parent, me[1..])
                        } else if parent_last == '/' && me_first != '/' {
                            concat_string!(parent, me)
                        } else if parent_last != '/' && me_first == '/' {
                            concat_string!(parent, me)
                        } else {
                            concat_string!(parent, "/", me)
                        }
                    }
                    None => {
                        parent.to_owned()
                    }
                }
            }
            None => {
                match me.chars().next() {
                    Some(me_first) => {
                        if me_first == '/' {
                            me.to_owned()
                        } else {
                            concat_string!("/", me)
                        }
                    }
                    None => {
                        panic!("PointName.new | Parent or name can't be empty")
                    }
                }
            }
        }
        // joined.unwrap_or_else(|| {
        //     error!("PointName.new | Join error for parent: '{}', me: '{}'", self.parent, me);
        //     String::new()
        // })
    }
    ///
    /// Returns joined as '/parent/me'
    pub fn join(&self) -> String {
        self.joined.clone()
    }
    ///
    /// Returns original parent
    pub fn parent(&self) -> String {
        self.parent.clone()
    }
    ///
    /// Returns original me
    pub fn me(&self) -> String {
        self.me.clone()
    }
}
//
// 
impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.join())
    }
}
//
// 
impl Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.join())
    }
}
// ///
// /// 
impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.join() == other.join()
    }
}
//
// 
impl From<&Name> for String {
    fn from(value: &Name) -> Self {
        value.join()
    }
}
//
// 
impl From<Name> for String {
    fn from(value: Name) -> Self {
        value.join()
    }
}
//
// 
impl From<String> for Name {
    fn from(parent: String) -> Self {
        Name::new(parent, "")
    }
}
//
// 
impl From<&str> for Name {
    fn from(parent: &str) -> Self {
        Name::new(parent, "")
    }
}
///
/// 
unsafe impl Sync for Name {}

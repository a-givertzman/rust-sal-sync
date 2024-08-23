//!
//! #### Couse of transmission.
//! Bitmask (Enum) being a part of the [Point](../point/index.html).  
//! Contains information about transmission cause and direction.  
//! Basic values at the moment (can be extended):
//! ```
//! Inf      = 0b_0000_0010; // 2   (0x2);
//! Act      = 0b_0000_0100; // 4   (0x4);
//! ActCon   = 0b_0000_1000; // 8   (0x8);
//! ActErr   = 0b_0001_0000; // 16  (0x10);
//! Req      = 0b_0010_0000; // 32  (0x20);
//! ReqCon   = 0b_0100_0000; // 64  (0x40);
//! ReqErr   = 0b_1000_0000; // 128 (0x80);
//! Read     = 0b_1101_1010; // 218 (0xDA)
//! Write    = 0b_0010_0100; // 36  (0x24)
//! All      = 0b_1111_1111; // 255 (0xFF)
//!
use serde::{Serialize, Deserialize};
///
mod cot {
    pub const INF    : u32 = 0b00000010;
    pub const ACT    : u32 = 0b00000100;
    pub const ACT_CON: u32 = 0b00001000;
    pub const ACT_ERR: u32 = 0b00010000;
    pub const REQ    : u32 = 0b00100000;
    pub const REQ_CON: u32 = 0b01000000;
    pub const REQ_ERR: u32 = 0b10000000;
}
///
/// Cause and diraction of the transmission
/// Inf - Information
/// Act - Activation
/// ActCon - Activation | Confirmatiom
/// ActErr - Activation | Error
/// JdsService 
/// Req - Request (Common request to the JdsService)
/// ReqCon - Rquest | Confirmatiom reply 
/// ReqErr - Rquest | Error reply
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
#[repr(u32)]
pub enum Cot {
    #[serde(rename = "Inf")]
    #[serde(alias = "inf", alias = "Inf", alias = "INF")]
    Inf = cot::INF,
    #[serde(rename = "Act")]
    #[serde(alias = "act", alias = "Act", alias = "ACT")]
    Act = cot::ACT,
    #[serde(rename = "ActCon")]
    #[serde(alias = "actcon", alias = "ActCon", alias = "ACTCON")]
    ActCon = cot::ACT_CON,
    #[serde(rename = "ActErr")]
    #[serde(alias = "acterr", alias = "ActErr", alias = "ACTERR")]
    ActErr = cot::ACT_ERR,
    #[serde(rename = "Req")]
    #[serde(alias = "req", alias = "Req", alias = "REQ")]
    Req = cot::REQ,
    #[serde(rename = "ReqCon")]
    #[serde(alias = "reqcon", alias = "ReqCon", alias = "REQCON")]
    ReqCon = cot::REQ_CON,
    #[serde(rename = "ReqErr")]
    #[serde(alias = "reqerr", alias = "ReqErr", alias = "REQERR")]
    ReqErr = cot::REQ_ERR,
    #[serde(skip)]
    Read = cot::INF | cot::ACT_CON | cot::ACT_ERR | cot::REQ_CON | cot::REQ_ERR,
    #[serde(skip)]
    Write = cot::ACT | cot::REQ,
    #[serde(skip)]
    All = 0xFF,// cot::INF | cot::ACT_CON | cot::ACT_ERR | cot::REQ_CON | cot::REQ_ERR | cot::ACT | cot::REQ,
}
//
// 
impl Cot {
    ///
    /// Returns true if [self] contains [rhs]
    pub fn contains(&self, rhs: Cot) -> bool {
        (*self & rhs) > 0
    }
    ///
    /// Returns string representation of the given Cot
    pub fn as_str(&self) -> &str {
        match self {
            Cot::Inf => "Inf",
            Cot::Act => "Act",
            Cot::ActCon => "ActCon",
            Cot::ActErr => "ActErr",
            Cot::Req => "Req",
            Cot::ReqCon => "ReqCon",
            Cot::ReqErr => "ReqErr",
            Cot::Read => "Read",
            Cot::Write => "Write",
            Cot::All => "",
        }
    }
}
//
// 
impl Default for Cot {
    fn default() -> Self {
        Self::Inf
    }
}
//
// 
impl AsRef<str> for Cot {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
//
// 
impl std::ops::BitOr for Cot {
    type Output = u32;
    fn bitor(self, rhs: Cot) -> Self::Output {
        self as u32 | rhs as u32
    }
}
//
// 
impl std::ops::BitAnd for Cot {
    type Output = u32;
    fn bitand(self, rhs: Cot) -> Self::Output {
        self as u32 & rhs as u32
    }
}
//
// 
impl std::ops::BitOr<Cot> for u32 {
    type Output = u32;
    fn bitor(self, rhs: Cot) -> Self::Output {
        self | rhs as u32
    }
}
//
// 
impl std::ops::BitAnd<Cot> for u32 {
    type Output = u32;
    fn bitand(self, rhs: Cot) -> Self::Output {
        self & rhs as u32
    }
}
//
// 
impl std::fmt::Binary for Cot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&format!("{:#08b}",self.to_owned() as u32), f)
    }
}

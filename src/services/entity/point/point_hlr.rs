use chrono::DateTime;
use crate::services::{
    entity::{Cot, Status},
    types::Bool,
};
///
/// ### Holds the unit of the information 
#[derive(Clone, Debug, PartialEq)]
pub struct PointHlr<T> {
    pub txid: usize,
    pub name: String,
    pub value: T,
    pub status: Status,
    pub cot: Cot,
    pub timestamp: DateTime<chrono::Utc>,
}
//
// 
impl<T> PointHlr<T> {
    ///
    /// Creates new instance of the Point
    ///     - txId: usize - unique id of the producer of the point, necessary only for internal purposes, like identify the producer of the point in the MultiQueue to prevent send back to the producer
    ///     - name: &str - full name of the point like '/AppName/DeviceName/Point.Name' unique within the entire system, for the Write direction name can be not a full
    ///     - value: T - supported types: bool, i64, f64, String
    ///     - status: Status - indicates Ok or some kind of invalidity
    ///     - direction: Direction - the kind of the direction Read / Write
    ///     - timestamp: DateTime<chrono::Utc> - registration timestamp
    pub fn new(txid: usize, name: impl Into<String>, value: T, status: Status, cot: Cot, ts: DateTime<chrono::Utc>) -> PointHlr<T> {
        Self {
            txid,
            name: name.into(),
            value,
            status,
            cot,
            timestamp: ts,
        }
    }
    ///
    /// Returns `PointHlr` with updated `txid`
    pub fn with_txid(mut self, txid: usize) -> Self {
        self.txid = txid;
        self
    }
    ///
    /// Returns `PointHlr` with updated `name`
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
    ///
    /// Returns `PointHlr` with updated `value`
    pub fn with_value(mut self, value: T) -> Self {
        self.value = value;
        self
    }
    ///
    /// Returns `PointHlr` with updated `status`
    pub fn with_status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }
    ///
    /// Returns `PointHlr` with updated `cot`
    pub fn with_cot(mut self, cot: Cot) -> Self {
        self.cot = cot;
        self
    }
    ///
    /// Returns `PointHlr` with updated `timestamp`
    pub fn with_ts(mut self, ts: DateTime<chrono::Utc>) -> Self {
        self.timestamp = ts;
        self
    }

}
//
// 
impl PointHlr<Bool> {
    ///
    /// Creates `Point<Bool>` with given `name` & `value`, taking current timestamp, `Status::Ok`, `Direction::Read`
    pub fn new_bool(txid: usize, name: impl Into<String>, value: bool) -> PointHlr<Bool> {
        PointHlr {
            txid,
            name: name.into(),
            value: Bool(value),
            status: Status::Ok,
            cot: Cot::default(),
            timestamp: chrono::offset::Utc::now(),
        }
    }
    ///
    /// Returns the Point with the absolute value
    pub fn abs(&self) -> PointHlr<Bool> {
        Self {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value,
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Bool
    pub fn to_bool(&self) -> PointHlr<Bool> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value,
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Int
    pub fn to_int(&self) -> PointHlr<i64> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: if self.value.0 {1} else {0},
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Real
    pub fn to_real(&self) -> PointHlr<f32> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: if self.value.0 {1.0f32} else {0.0f32},
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Double
    pub fn to_double(&self) -> PointHlr<f64> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: if self.value.0 {1.0f64} else {0.0f64},
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the String
    pub fn to_string(&self) -> PointHlr<String> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value.to_string(),
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
}
//
// 
impl PointHlr<i64> {
    ///
    /// Creates `Point<i64>` with given `name` & `value`,
    /// taking current timestamp, `Status::Ok`, `Direction::Read`
    pub fn new_int(txid: usize, name: impl Into<String>, value: i64) -> PointHlr<i64> {
        PointHlr {
            txid,
            name: name.into(),
            value,
            status: Status::Ok,
            cot: Cot::default(),
            timestamp: chrono::offset::Utc::now(),
        }
    }
    ///
    /// Returns the Point with the absolute value
    pub fn abs(&self) -> PointHlr<i64> {
        Self {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value.abs(),
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Bool
    pub fn to_bool(&self) -> PointHlr<Bool> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: Bool(self.value > 0),
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Int
    pub fn to_int(&self) -> PointHlr<i64> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value,
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Real
    pub fn to_real(&self) -> PointHlr<f32> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value as f32,
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Double
    pub fn to_double(&self) -> PointHlr<f64> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value as f64,
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the String
    pub fn to_string(&self) -> PointHlr<String> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value.to_string(),
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Raises self to the `exp` power.
    pub fn pow(&self, exp: Self) -> Self {
        let status = match self.status.cmp(&exp.status) {
            std::cmp::Ordering::Less => exp.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let (txid, timestamp) = match self.timestamp.cmp(&exp.timestamp) {
            std::cmp::Ordering::Less => (exp.txid, exp.timestamp),
            std::cmp::Ordering::Equal => (self.txid, self.timestamp),
            std::cmp::Ordering::Greater => (self.txid, self.timestamp),
        };
        let cot = if self.cot == exp.cot {
            self.cot
        } else {
            panic!("Point.pow | Cot's are not equals")
        };
        PointHlr {
            txid,
            name: String::from("Point.Pow"),
            value: self.value.pow(exp.value as u32),
            status,
            cot,
            timestamp,
        }
    }
}
//
// 
impl PointHlr<f32> {
    ///
    /// Creates `Point<f32>` with given `name` & `value`, taking current timestamp, `Status::Ok`, `Direction::Read`
    pub fn new_real(txid: usize, name: impl Into<String>, value: f32) -> PointHlr<f32> {
        PointHlr {
            txid,
            name: name.into(),
            value,
            status: Status::Ok,
            cot: Cot::default(),
            timestamp: chrono::offset::Utc::now(),
        }
    }
    ///
    /// Returns the Point with the absolute value
    pub fn abs(&self) -> PointHlr<f32> {
        Self {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value.abs(),
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Bool
    pub fn to_bool(&self) -> PointHlr<Bool> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: Bool(self.value > 0.0),
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Int
    pub fn to_int(&self) -> PointHlr<i64> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value.round() as i64,
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Real
    pub fn to_real(&self) -> PointHlr<f32> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value,
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Double
    pub fn to_double(&self) -> PointHlr<f64> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value as f64,
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the String
    pub fn to_string(&self) -> PointHlr<String> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value.to_string(),
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Raises self to the `exp` power.
    pub fn pow(&self, exp: Self) -> Self {
        let status = match self.status.cmp(&exp.status) {
            std::cmp::Ordering::Less => exp.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let (txid, timestamp) = match self.timestamp.cmp(&exp.timestamp) {
            std::cmp::Ordering::Less => (exp.txid, exp.timestamp),
            std::cmp::Ordering::Equal => (self.txid, self.timestamp),
            std::cmp::Ordering::Greater => (self.txid, self.timestamp),
        };
        let cot = if self.cot == exp.cot {
            self.cot
        } else {
            panic!("Point.pow | Cot's are not equals")
        };
        PointHlr {
            txid,
            name: String::from("Point.Pow"),
            value: self.value.powf(exp.value),
            status,
            cot,
            timestamp,
        }
    }
}
//
// 
impl PointHlr<f64> {
    ///
    /// Creates `Point<f64>` with given `name` & `value`, taking current timestamp, `Status::Ok`, `Direction::Read`
    pub fn new_double(txid: usize, name: impl Into<String>, value: f64) -> PointHlr<f64> {
        PointHlr {
            txid,
            name: name.into(),
            value,
            status: Status::Ok,
            cot: Cot::default(),
            timestamp: chrono::offset::Utc::now(),
        }
    }
    ///
    /// Returns the Point with the absolute value
    pub fn abs(&self) -> PointHlr<f64> {
        Self {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value.abs(),
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Bool
    pub fn to_bool(&self) -> PointHlr<Bool> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: Bool(self.value > 0.0),
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Int
    pub fn to_int(&self) -> PointHlr<i64> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value.round() as i64,
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Real
    pub fn to_real(&self) -> PointHlr<f32> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value as f32,
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Double
    pub fn to_double(&self) -> PointHlr<f64> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value,
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the String
    pub fn to_string(&self) -> PointHlr<String> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: self.value.to_string(),
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Raises self to the `exp` power.
    pub fn pow(&self, exp: Self) -> Self {
        let status = match self.status.cmp(&exp.status) {
            std::cmp::Ordering::Less => exp.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let (txid, timestamp) = match self.timestamp.cmp(&exp.timestamp) {
            std::cmp::Ordering::Less => (exp.txid, exp.timestamp),
            std::cmp::Ordering::Equal => (self.txid, self.timestamp),
            std::cmp::Ordering::Greater => (self.txid, self.timestamp),
        };
        let cot = if self.cot == exp.cot {
            self.cot
        } else {
            panic!("Point.pow | Cot's are not equals")
        };
        PointHlr {
            txid,
            name: String::from("Point.Pow"),
            value: self.value.powf(exp.value),
            status,
            cot,
            timestamp,
        }
    }
}
//
// 
impl PointHlr<String> {
    ///
    /// Creates `Point<String>`` with given `name` & `value`, taking current timestamp, `Status::Ok`, `Direction::Read`
    pub fn new_string(txid: usize, name: impl Into<String>, value: impl Into<String>) -> PointHlr<String> {
        PointHlr {
            txid,
            name: name.into(),
            value: value.into(),
            status: Status::Ok,
            cot: Cot::default(),
            timestamp: chrono::offset::Utc::now(),
        }
    }
}
//
// 
impl PointHlr<Vec<u8>> {
    ///
    /// Creates `Point<Bytes>`` with given `name` & `value`, taking current timestamp, `Status::Ok`, `Direction::Read`
    pub fn new_bytes(txid: usize, name: impl Into<String>, value: impl Into<Vec<u8>>) -> PointHlr<Vec<u8>> {
        PointHlr {
            txid,
            name: name.into(),
            value: value.into(),
            status: Status::Ok,
            cot: Cot::default(),
            timestamp: chrono::offset::Utc::now(),
        }
    }
    ///
    /// Returns Point converted to the Bool
    pub fn to_bool(&self) -> PointHlr<Bool> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: Bool(match self.value.first() {
                    Some(value) => *value != 0,
                    None => {
                        panic!("PointHlr({}).to_bool | Error convert to Bool, no bytes found in: '{:?}'", self.name, self.value);
                    }
                }),
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Int
    pub fn to_int(&self) -> PointHlr<i64> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: match self.value[0..8].try_into() {
                Ok(value) => i64::from_be_bytes(value),
                Err(err) => {
                    panic!("PointHlr({}).to_int | Error convert to Int value: {:?}\n\terror: {:#?}", self.name, self.value, err);
                }
            },
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Real
    pub fn to_real(&self) -> PointHlr<f32> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: match self.value[0..4].try_into() {
                Ok(value) => f32::from_be_bytes(value),
                Err(err) => {
                    panic!("PointHlr({}).to_int | Error convert to Int value: {:?}\n\terror: {:#?}", self.name, self.value, err);
                }
            },
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the Double
    pub fn to_double(&self) -> PointHlr<f64> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: match self.value[0..8].try_into() {
                Ok(value) => f64::from_be_bytes(value),
                Err(err) => {
                    panic!("PointHlr({}).to_int | Error convert to Double value: {:?}\n\terror: {:#?}", self.name, self.value, err);
                }
            },
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Returns Point converted to the String
    pub fn to_string(&self) -> PointHlr<String> {
        PointHlr {
            txid: self.txid,
            name: self.name.clone(),
            value: String::from_utf8_lossy(&self.value).into_owned(),
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
}
//
// 
impl<T: std::ops::Add<Output = T>> std::ops::Add for PointHlr<T> {
    type Output = PointHlr<T>;
    fn add(self, rhs: Self) -> Self::Output {
        let status = match self.status.cmp(&rhs.status) {
            std::cmp::Ordering::Less => rhs.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let (txid, timestamp) = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => (rhs.txid, rhs.timestamp),
            std::cmp::Ordering::Equal => (self.txid, self.timestamp),
            std::cmp::Ordering::Greater => (self.txid, self.timestamp),
        };
        let cot = if self.cot == rhs.cot {
            self.cot
        } else {
            panic!("Point.add | Cot's are not equals")
        };
        PointHlr {
            txid,
            name: String::from("Point.Add"),
            value: self.value + rhs.value,
            status,
            cot,
            timestamp,
        }
    }
}
//
//
impl<T: std::ops::Sub<Output = T>> std::ops::Sub for PointHlr<T> {
    type Output = PointHlr<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        let status = match self.status.cmp(&rhs.status) {
            std::cmp::Ordering::Less => rhs.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let (txid, timestamp) = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => (rhs.txid, rhs.timestamp),
            std::cmp::Ordering::Equal => (self.txid, self.timestamp),
            std::cmp::Ordering::Greater => (self.txid, self.timestamp),
        };
        let cot = if self.cot == rhs.cot {
            self.cot
        } else {
            panic!("Point.sub | Cot's are not equals")
        };
        PointHlr {
            txid,
            name: String::from("Point.Sub"),
            value: self.value - rhs.value,
            status,
            cot,
            timestamp,
        }
    }
}
//
//
impl<T: std::ops::Mul<Output = T>> std::ops::Mul for PointHlr<T> {
    type Output = PointHlr<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        let status = match self.status.cmp(&rhs.status) {
            std::cmp::Ordering::Less => rhs.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let (txid, timestamp) = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => (rhs.txid, rhs.timestamp),
            std::cmp::Ordering::Equal => (self.txid, self.timestamp),
            std::cmp::Ordering::Greater => (self.txid, self.timestamp),
        };
        let cot = if self.cot == rhs.cot {
            self.cot
        } else {
            panic!("Point.mul | Cot's are not equals")
        };
        PointHlr {
            txid,
            name: String::from("Point.Mul"),
            value: self.value * rhs.value,
            status,
            cot,
            timestamp,
        }
    }
}
//
//
impl<T: std::ops::Div<Output = T>> std::ops::Div for PointHlr<T> {
    type Output = PointHlr<T>;
    fn div(self, rhs: Self) -> Self::Output {
        let status = match self.status.cmp(&rhs.status) {
            std::cmp::Ordering::Less => rhs.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let (txid, timestamp) = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => (rhs.txid, rhs.timestamp),
            std::cmp::Ordering::Equal => (self.txid, self.timestamp),
            std::cmp::Ordering::Greater => (self.txid, self.timestamp),
        };
        let cot = if self.cot == rhs.cot {
            self.cot
        } else {
            panic!("Point.div | Cot's are not equals")
        };
        PointHlr {
            txid,
            name: String::from("Point.Div"),
            value: self.value / rhs.value,
            status,
            cot,
            timestamp,
        }
    }
}
//
//
impl<T: std::ops::BitOr<Output = T>> std::ops::BitOr for PointHlr<T> {
    type Output = PointHlr<T>;
    fn bitor(self, rhs: Self) -> Self::Output {
        let status = match self.status.cmp(&rhs.status) {
            std::cmp::Ordering::Less => rhs.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let (txid, timestamp) = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => (rhs.txid, rhs.timestamp),
            std::cmp::Ordering::Equal => (self.txid, self.timestamp),
            std::cmp::Ordering::Greater => (self.txid, self.timestamp),
        };
        let cot = if self.cot == rhs.cot {
            self.cot
        } else {
            panic!("Point.bitor | Cot's are not equals")
        };
        PointHlr {
            txid,
            name: String::from("Point.BitOr"),
            value: self.value | rhs.value,
            status,
            cot,
            timestamp,
        }        
    }
}
//
//
impl<T: std::ops::BitAnd<Output = T>> std::ops::BitAnd for PointHlr<T> {
    type Output = PointHlr<T>;
    fn bitand(self, rhs: Self) -> Self::Output {
        let status = match self.status.cmp(&rhs.status) {
            std::cmp::Ordering::Less => rhs.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let (txid, timestamp) = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => (rhs.txid, rhs.timestamp),
            std::cmp::Ordering::Equal => (self.txid, self.timestamp),
            std::cmp::Ordering::Greater => (self.txid, self.timestamp),
        };
        let cot = if self.cot == rhs.cot {
            self.cot
        } else {
            panic!("Point.bitor | Cot's are not equals")
        };
        PointHlr {
            txid,
            name: String::from("Point.BitOr"),
            value: self.value & rhs.value,
            status,
            cot,
            timestamp,
        }        
    }
}
//
//
impl<T: std::cmp::PartialOrd> std::cmp::PartialOrd for PointHlr<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

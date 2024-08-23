use chrono::DateTime;
use crate::services::{
    entity::{cot::Cot, status::status::Status},
    types::bool::Bool,
};
///
/// Holds the unit of the information 
#[derive(Clone, Debug, PartialEq)]
pub struct PointHlr<T> {
    pub tx_id: usize,
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
    pub fn new(tx_id: usize, name: &str, value: T, status: Status, cot: Cot, timestamp: DateTime<chrono::Utc>) -> PointHlr<T> {
        Self {
            tx_id,
            name: name.to_owned(),
            value,
            status,
            cot,
            timestamp,
        }
    }
}
//
// 
impl PointHlr<Bool> {
    ///
    /// creates Point<Bool> with given name & value, taking current timestamp, Status::Ok, Direction::Read
    pub fn new_bool(tx_id: usize, name: &str, value: bool) -> PointHlr<Bool> {
        PointHlr {
            tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
    /// creates Point<i64> with given name & value,
    /// taking current timestamp, Status::Ok, Direction::Read
    pub fn new_int(tx_id: usize, name: &str, value: i64) -> PointHlr<i64> {
        PointHlr {
            tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
            name: self.name.clone(),
            value: self.value.to_string(),
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Raises self to a [factor] power.
    pub fn pow(&self, exp: Self) -> Self {
        let status = match self.status.cmp(&exp.status) {
            std::cmp::Ordering::Less => exp.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let (tx_id, timestamp) = match self.timestamp.cmp(&exp.timestamp) {
            std::cmp::Ordering::Less => (exp.tx_id, exp.timestamp),
            std::cmp::Ordering::Equal => (self.tx_id, self.timestamp),
            std::cmp::Ordering::Greater => (self.tx_id, self.timestamp),
        };
        let cot = if self.cot == exp.cot {
            self.cot
        } else {
            panic!("Point.pow | Cot's are not equals")
        };
        PointHlr {
            tx_id,
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
    /// creates Point<f32> with given name & value, taking current timestamp, Status::Ok, Direction::Read
    pub fn new_real(tx_id: usize, name: &str, value: f32) -> PointHlr<f32> {
        PointHlr {
            tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
            name: self.name.clone(),
            value: self.value.to_string(),
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Raises self to a [factor] power.
    pub fn pow(&self, exp: Self) -> Self {
        let status = match self.status.cmp(&exp.status) {
            std::cmp::Ordering::Less => exp.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let (tx_id, timestamp) = match self.timestamp.cmp(&exp.timestamp) {
            std::cmp::Ordering::Less => (exp.tx_id, exp.timestamp),
            std::cmp::Ordering::Equal => (self.tx_id, self.timestamp),
            std::cmp::Ordering::Greater => (self.tx_id, self.timestamp),
        };
        let cot = if self.cot == exp.cot {
            self.cot
        } else {
            panic!("Point.pow | Cot's are not equals")
        };
        PointHlr {
            tx_id,
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
    /// creates Point<f64> with given name & value, taking current timestamp, Status::Ok, Direction::Read
    pub fn new_double(tx_id: usize, name: &str, value: f64) -> PointHlr<f64> {
        PointHlr {
            tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
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
            tx_id: self.tx_id,
            name: self.name.clone(),
            value: self.value.to_string(),
            status: self.status,
            cot: self.cot,
            timestamp: self.timestamp,
        }
    }
    ///
    /// Raises self to a [factor] power.
    pub fn pow(&self, exp: Self) -> Self {
        let status = match self.status.cmp(&exp.status) {
            std::cmp::Ordering::Less => exp.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let (tx_id, timestamp) = match self.timestamp.cmp(&exp.timestamp) {
            std::cmp::Ordering::Less => (exp.tx_id, exp.timestamp),
            std::cmp::Ordering::Equal => (self.tx_id, self.timestamp),
            std::cmp::Ordering::Greater => (self.tx_id, self.timestamp),
        };
        let cot = if self.cot == exp.cot {
            self.cot
        } else {
            panic!("Point.pow | Cot's are not equals")
        };
        PointHlr {
            tx_id,
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
    /// creates Point<String> with given name & value, taking current timestamp, Status::Ok, Direction::Read
    pub fn new_string(tx_id: usize, name: &str, value: impl Into<String>) -> PointHlr<String> {
        PointHlr {
            tx_id,
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
impl<T: std::ops::Add<Output = T>> std::ops::Add for PointHlr<T> {
    type Output = PointHlr<T>;
    fn add(self, rhs: Self) -> Self::Output {
        let status = match self.status.cmp(&rhs.status) {
            std::cmp::Ordering::Less => rhs.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let (tx_id, timestamp) = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => (rhs.tx_id, rhs.timestamp),
            std::cmp::Ordering::Equal => (self.tx_id, self.timestamp),
            std::cmp::Ordering::Greater => (self.tx_id, self.timestamp),
        };
        let cot = if self.cot == rhs.cot {
            self.cot
        } else {
            panic!("Point.add | Cot's are not equals")
        };
        PointHlr {
            tx_id,
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
        let (tx_id, timestamp) = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => (rhs.tx_id, rhs.timestamp),
            std::cmp::Ordering::Equal => (self.tx_id, self.timestamp),
            std::cmp::Ordering::Greater => (self.tx_id, self.timestamp),
        };
        let cot = if self.cot == rhs.cot {
            self.cot
        } else {
            panic!("Point.sub | Cot's are not equals")
        };
        PointHlr {
            tx_id,
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
        let (tx_id, timestamp) = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => (rhs.tx_id, rhs.timestamp),
            std::cmp::Ordering::Equal => (self.tx_id, self.timestamp),
            std::cmp::Ordering::Greater => (self.tx_id, self.timestamp),
        };
        let cot = if self.cot == rhs.cot {
            self.cot
        } else {
            panic!("Point.mul | Cot's are not equals")
        };
        PointHlr {
            tx_id,
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
        let (tx_id, timestamp) = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => (rhs.tx_id, rhs.timestamp),
            std::cmp::Ordering::Equal => (self.tx_id, self.timestamp),
            std::cmp::Ordering::Greater => (self.tx_id, self.timestamp),
        };
        let cot = if self.cot == rhs.cot {
            self.cot
        } else {
            panic!("Point.div | Cot's are not equals")
        };
        PointHlr {
            tx_id,
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
        let (tx_id, timestamp) = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => (rhs.tx_id, rhs.timestamp),
            std::cmp::Ordering::Equal => (self.tx_id, self.timestamp),
            std::cmp::Ordering::Greater => (self.tx_id, self.timestamp),
        };
        let cot = if self.cot == rhs.cot {
            self.cot
        } else {
            panic!("Point.bitor | Cot's are not equals")
        };
        PointHlr {
            tx_id,
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
        let (tx_id, timestamp) = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => (rhs.tx_id, rhs.timestamp),
            std::cmp::Ordering::Equal => (self.tx_id, self.timestamp),
            std::cmp::Ordering::Greater => (self.tx_id, self.timestamp),
        };
        let cot = if self.cot == rhs.cot {
            self.cot
        } else {
            panic!("Point.bitor | Cot's are not equals")
        };
        PointHlr {
            tx_id,
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

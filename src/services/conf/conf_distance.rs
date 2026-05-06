use std::{str::FromStr, sync::OnceLock};
use log::trace;
use regex::{Regex, RegexBuilder};
use sal_core::error::Error;
use serde::Deserialize;

///
/// Unit of Distance
/// ```ignore
///  -                 | Millimetre (mm) | Centimetre (cm) | Metre (m) | Kilometre (km)
///  :---              | :---            | :---            | :---      | :---
///  1 millimetre (mm) |  1              |  0.1            |    0.001  |  0.000001
///  1 centimetre (cm) |  10             |  1              |    0.01   |  0.00001
///  1 metre (m)       |  1000           |  100            |    1      |  0.001
///  1 kilometre (km)  |  1000000        |  100000         |    1000   |  1
///  1 inch (in)       |  25.4           |  2.54           |    0.0254 |  0.0000254
/// ```
/// - nm - nanometers, 
/// - um - micrometers, 
/// - mm - millimetre, 
/// - cm - centimetres, 
/// - m  - meters, 
/// - km - kilometers, 
/// - in - inches
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum ConfDistanceUnit {
    Nanometer,
    Micrometer,
    Millimeter,
    Centimetre,
    Meter,
    Kilometer,
    Inch,
}
//
// 
impl FromStr for ConfDistanceUnit {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "nm" => Ok(Self::Nanometer),
            "um" => Ok(Self::Micrometer),
            "mm" => Ok(Self::Millimeter),
            "cm"  => Ok(Self::Centimetre),
            "m"  => Ok(Self::Meter),
            "km"  => Ok(Self::Kilometer),
            "in"  => Ok(Self::Inch),
            _ => Err(Error::new("ConfDistanceUnit", "from_str").err(format!("Unknown distance unit: '{}'", input)))
        }
    }
}
//
//
impl std::fmt::Display for ConfDistanceUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unit = match self {
            ConfDistanceUnit::Nanometer => "nm",
            ConfDistanceUnit::Micrometer => "um",
            ConfDistanceUnit::Millimeter => "mm",
            ConfDistanceUnit::Centimetre => "cm",
            ConfDistanceUnit::Meter => "m",
            ConfDistanceUnit::Kilometer => "km",
            ConfDistanceUnit::Inch => "in",
        };
        write!(f, "{unit}")
    }
}

///
/// Distance keyword consists of 2 fields:
/// ```ignore
/// | value  | unit |
/// | ------ | ---- |
/// | requir | opt  |
/// | ------ | ---- |
/// | 111    |  nm  | - 111 nanometers
/// | 12     |  um  | - 12 micrometers
/// | 11     |  cm  | - 11 centimeters
/// | 5      |  m   | - 5 meters
/// | 5      |      | - 5 meters
/// | 3      |  km  | - 3 kilometers
/// | 1      |  in  | - 1 inches
/// ```
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub struct ConfDistance {
    pub value: f64,
    pub unit: ConfDistanceUnit,
}
//
// 
impl ConfDistance {
    ///
    /// New instance if ConfDistance
    pub fn new(value: f64, unit: ConfDistanceUnit) -> Self {
        Self {
            value,
            unit,
        }
    }
    ///
    /// Returns distance in Nanometers (nm)
    pub fn as_nm(&self) -> f64 {
        match self.unit {
            ConfDistanceUnit::Nanometer  => self.value,
            ConfDistanceUnit::Micrometer => self.value * 1_000.0,
            ConfDistanceUnit::Millimeter => self.value * 1e+6,
            ConfDistanceUnit::Centimetre => self.value * 1e+7,
            ConfDistanceUnit::Meter      => self.value * 1e+9,
            ConfDistanceUnit::Kilometer  => self.value * 1e+12,
            ConfDistanceUnit::Inch       => self.value * 2.54e+7,
        }
    }
    ///
    /// Returns distance in Micrometers (um)
    pub fn as_um(&self) -> f64 {
        match self.unit {
            ConfDistanceUnit::Nanometer  => self.value * 0.001,
            ConfDistanceUnit::Micrometer => self.value,
            ConfDistanceUnit::Millimeter => self.value * 1000.0,
            ConfDistanceUnit::Centimetre => self.value * 10_000.0,
            ConfDistanceUnit::Meter      => self.value * 1e+6,
            ConfDistanceUnit::Kilometer  => self.value * 1e+9,
            ConfDistanceUnit::Inch       => self.value * 25400.0,
        }
    }
    ///
    /// Returns distance in Millimeters (mm)
    pub fn as_mm(&self) -> f64 {
        match self.unit {
            ConfDistanceUnit::Nanometer  => self.value * 1e-6,
            ConfDistanceUnit::Micrometer => self.value * 0.001,
            ConfDistanceUnit::Millimeter => self.value,
            ConfDistanceUnit::Centimetre => self.value * 10.0,
            ConfDistanceUnit::Meter      => self.value * 1000.0,
            ConfDistanceUnit::Kilometer  => self.value * 1e+6,
            ConfDistanceUnit::Inch       => self.value * 25.4,
        }
    }
    ///
    /// Returns distance in Centimetres (cm)
    pub fn as_cm(&self) -> f64 {
        match self.unit {
            ConfDistanceUnit::Nanometer  => self.value * 1e-7,
            ConfDistanceUnit::Micrometer => self.value * 1e-4,
            ConfDistanceUnit::Millimeter => self.value * 0.1,
            ConfDistanceUnit::Centimetre => self.value,
            ConfDistanceUnit::Meter      => self.value * 100.0,
            ConfDistanceUnit::Kilometer  => self.value * 100_000.0,
            ConfDistanceUnit::Inch       => self.value * 2.54,
        }
    }
    ///
    /// Returns distance in Meters (m)
    pub fn as_m(&self) -> f64 {
        match self.unit {
            ConfDistanceUnit::Nanometer  => self.value * 1e-9,
            ConfDistanceUnit::Micrometer => self.value * 1e-6,
            ConfDistanceUnit::Millimeter => self.value * 0.001,
            ConfDistanceUnit::Centimetre => self.value * 0.01,
            ConfDistanceUnit::Meter      => self.value,
            ConfDistanceUnit::Kilometer  => self.value * 1000.0,
            ConfDistanceUnit::Inch       => self.value * 0.0254,
        }
    }
    ///
    /// Returns distance in Kilometers (km)
    pub fn as_km(&self) -> f64 {
        match self.unit {
            ConfDistanceUnit::Nanometer  => self.value * 1e-12,
            ConfDistanceUnit::Micrometer => self.value * 1e-9,
            ConfDistanceUnit::Millimeter => self.value * 1e-6,
            ConfDistanceUnit::Centimetre => self.value * 1e-5,
            ConfDistanceUnit::Meter      => self.value * 0.001,
            ConfDistanceUnit::Kilometer  => self.value,
            ConfDistanceUnit::Inch       => self.value * 2.54e-5,
        }
    }
    ///
    /// Returns distance in Inches (in)
    pub fn as_in(&self) -> f64 {
        match self.unit {
            ConfDistanceUnit::Nanometer  => self.value / 2.54e+7,   // На два такта медленнее, но точнее результат
            ConfDistanceUnit::Micrometer => self.value / 25400.0,   // На два такта медленнее, но точнее результат
            ConfDistanceUnit::Millimeter => self.value / 25.4,      // На два такта медленнее, но точнее результат
            ConfDistanceUnit::Centimetre => self.value / 2.54,      // На два такта медленнее, но точнее результат
            ConfDistanceUnit::Meter      => self.value / 0.0254,    // На два такта медленнее, но точнее результат
            ConfDistanceUnit::Kilometer  => self.value / 0.0000254, // На два такта медленнее, но точнее результат
            ConfDistanceUnit::Inch       => self.value,
        }
    }
}
//
// 
static CONF_DISTANCE_RE: OnceLock<Regex> = OnceLock::new();
//
impl FromStr for ConfDistance {
    type Err = Error;
    fn from_str(input: &str) -> Result<ConfDistance, Error> {
        trace!("ConfDistance.from_str | input: {}", input);
        let re = CONF_DISTANCE_RE.get_or_init(|| RegexBuilder::new(
            // r"^[ \t]*([-+]?[\d]*\.?[\d]+)[ \t]*(nm|um|mm|cm|m|km|in){0,1}[ \t]*$"
            r"^[ \t]*([-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?)[ \t]*(nm|um|mm|cm|m|km|in)?[ \t]*$"
        ).multi_line(true).build().unwrap());
        let group_value = 1;
        let group_unit = 2;
        let caps = re.captures(input)
            .ok_or_else(|| Error::new("ConfDistance", "from_str").err(format!("Can't parse distance '{}'", input)))?;
        let first = &caps.get(group_value)
            .ok_or_else(|| Error::new("ConfDistance", "from_str").err(format!("Wrong distance value: '{}'", input)))?;
        let value = first.as_str().parse()
            .map_err(|err: std::num::ParseFloatError| Error::new("ConfDistance", "from_str").pass_with(format!("Can't parse distance '{}'", input), err.to_string()))?;
        let unit = match &caps.get(group_unit) {
            Some(u) => ConfDistanceUnit::from_str(u.as_str()).map_err(|err| Error::new("ConfDistance", "from_str").pass_with(format!("Can't parse distance '{}'", input), err.to_string()))?,
            None => ConfDistanceUnit::Meter,
        };
        Ok(ConfDistance::new(value, unit))
    }
}
//
//
impl std::fmt::Display for ConfDistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}
impl Default for ConfDistanceUnit {
    fn default() -> Self {
        Self::Meter
    }
}
//
//
impl Default for ConfDistance {
    ///
    /// Returns 0.0 Meters
    fn default() -> Self {
        Self {
            value: Default::default(),
            unit: Default::default(),
        }
    }
}
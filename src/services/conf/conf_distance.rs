use std::str::FromStr;
use log::trace;
use regex::RegexBuilder;
use serde::Deserialize;

///
/// Unit of Distance
/// ```ignore
///  -                 | Millimetre (mm) | Centimetre (cm) | Metre (m) | Kilometre (km)
///  ---               | ---             | ---             | ---       | ---
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
#[derive(Debug, Deserialize, PartialEq, Clone)]
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
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "nm" => Ok(Self::Nanometer),
            "um" => Ok(Self::Micrometer),
            "mm" => Ok(Self::Millimeter),
            "cm"  => Ok(Self::Centimetre),
            "m"  => Ok(Self::Meter),
            "km"  => Ok(Self::Kilometer),
            "in"  => Ok(Self::Inch),
            _ => Err(format!("ConfDistanceUnit.from_str | Unknown distance unit: '{}'", input))
        }
    }
}


///
/// keyword konsists of 2 fields:
/// ```ignore
/// | value  |  unit  |
/// | ------ | ------ |
/// | requir | opt    |
/// | ------ | ------ |
/// | 111    |  nm    | - 111 nanometers
/// | 12     |  um    | - 12 micrometers
/// | 11     |  cm    | - 11 centimeters
/// | 5      |  m     | - 5 meters
/// | 5      |        | - 5 meters
/// | 3      |  km    | - 3 kilometers
/// | 1      |  in    | - 1 inches
/// ```
#[derive(Debug, Deserialize, PartialEq)]
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
            ConfDistanceUnit::Nanometer  => self.value / 2.54e+7,
            ConfDistanceUnit::Micrometer => self.value / 25400.0,
            ConfDistanceUnit::Millimeter => self.value / 25.4,
            ConfDistanceUnit::Centimetre => self.value / 2.54,
            ConfDistanceUnit::Meter      => self.value * 39.3701 ,
            ConfDistanceUnit::Kilometer  => self.value * 39370.1,
            ConfDistanceUnit::Inch       => self.value,
        }
    }
}
//
// 
impl FromStr for ConfDistance {
    type Err = String;
    fn from_str(input: &str) -> Result<ConfDistance, String> {
        trace!("ConfDistance.from_str | input: {}", input);
        let re = r#"^[ \t]*(\d+)[ \t]*(nm|um|mm|m|km|in){0,1}[ \t]*$"#;
        let re = RegexBuilder::new(re).multi_line(true).build().unwrap();
        let group_value = 1;
        let group_unit = 2;
        match re.captures(input) {
            Some(caps) => {
                match &caps.get(group_value) {
                    Some(first) => {
                        match first.as_str().parse() {
                            Ok(value) => {
                                let unit = match &caps.get(group_unit) {
                                    Some(u) => match ConfDistanceUnit::from_str(u.as_str()) {
                                        Ok(unit) => Ok(unit),
                                        Err(err) => Err(err),
                                    }
                                    None => Ok(ConfDistanceUnit::Meter),
                                };
                                match unit {
                                    Ok(unit) => Ok(ConfDistance::new(value, unit)),
                                    Err(err) => Err(err),
                                }
                            }
                            Err(err) => Err(format!("ConfDistance.from_str | Error parsing distance value: '{}'\n\terror: {:?}", &input, err)),
                        }
                    }
                    None => Err(format!("ConfDistance.from_str | Error parsing distance value: '{}'", &input)),
                }
            }
            None => {
                Err(format!("ConfDistance.from_str | Error parsing distance value: '{}'", &input))
            }
        }
    }
}

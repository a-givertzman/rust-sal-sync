use std::{str::FromStr, sync::OnceLock};
use regex::{Regex, RegexBuilder};
use sal_core::error::Error;
use serde::Deserialize;

const GRADIANS_TO_RADIANS: f64 = std::f64::consts::PI / 200.0;
const RADIANS_TO_GRADIANS: f64 = 200.0 / std::f64::consts::PI;
const DEGREES_TO_GRADIANS: f64 = 10.0 / 9.0;

///
/// ### Unit of Angle
/// ```ignore
///  -                     | Scale         | Comment
///  ---                   | ---           | ---
///  1 Degrees (°, deg, d) |  1.0          |  Основная единица в UI.
///  2 Radians (rad, r)    |  ~57.2958     |  Основная единица для расчетов Math.sin()
///  3 Gradians (gon)      |  0.9          |  Используется в геодезии (400 град = круг)
///  4 Percent (%)         |  atan(x/100)  |  Для кранов: уклон пути или горизонта
/// ```
/// - deg - Degrees (deg, °), default
/// - rad - Radians (rad, r),
/// - gon - Gradians, 
/// - % - Percent, 
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum ConfAngleUnit {
    Degrees,
    Radians,
    Gradians,
    Percent,
}
//
// 
impl FromStr for ConfAngleUnit {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "deg" => Ok(Self::Degrees),
            "°" =>  Ok(Self::Degrees),
            "rad" => Ok(Self::Radians),
            "gon" => Ok(Self::Gradians),
            "%"  => Ok(Self::Percent),
            _ => Err(Error::new("ConfAngleUnit", "from_str").err(format!("Unknown angle unit: '{}'", input)))
        }
    }
}
//
//
impl std::fmt::Display for ConfAngleUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unit = match self {
            ConfAngleUnit::Degrees => "deg",
            ConfAngleUnit::Radians => "rad",
            ConfAngleUnit::Gradians => "gon",
            ConfAngleUnit::Percent => "%",
        };
        write!(f, "{unit}")
    }
}

///
/// ### Angle keyword consists of 2 fields:
/// ```ignore
/// | value  |  unit  |
/// | ------ | ------ |
/// | requir | opt    |
/// | ------ | ------ |
/// | 111    |  deg   | - 111 Degrees
/// | 12     |  rad   | - 12 Radians
/// | 11     |  %     | - 11 Percent
/// | 5      |  gon   | - 5 Gradians
/// ```
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub struct ConfAngle {
    pub value: f64,
    pub unit: ConfAngleUnit,
}
//
// 
impl ConfAngle {
    ///
    /// ### New instance if ConfAngle
    pub fn new(value: f64, unit: ConfAngleUnit) -> Self {
        Self {
            value,
            unit,
        }
    }
    ///
    /// ### Returns angle in Degrees (deg)
    pub fn as_deg(&self) -> f64 {
        match self.unit {
            ConfAngleUnit::Degrees  => self.value,
            ConfAngleUnit::Radians => self.value.to_degrees(),
            ConfAngleUnit::Gradians => self.value * 0.9,
            ConfAngleUnit::Percent => (self.value * 0.01).atan().to_degrees(),
        }
    }
    ///
    /// ### Returns angle in Radians (rad)
    pub fn as_rad(&self) -> f64 {
        match self.unit {
            ConfAngleUnit::Degrees  => self.value.to_radians(),
            ConfAngleUnit::Radians => self.value,
            ConfAngleUnit::Gradians => self.value * GRADIANS_TO_RADIANS,
            ConfAngleUnit::Percent => (self.value * 0.01).atan(),
        }
    }
    ///
    /// ### Returns angle in Gradians (gon)
    pub fn as_gradians(&self) -> f64 {
        match self.unit {
            ConfAngleUnit::Degrees  => self.value * DEGREES_TO_GRADIANS,
            ConfAngleUnit::Radians => self.value * RADIANS_TO_GRADIANS,
            ConfAngleUnit::Gradians => self.value,
            ConfAngleUnit::Percent => (self.value * 0.01).atan() * RADIANS_TO_GRADIANS,
        }
    }
    ///
    /// ### Returns angle in Percents (%)
    /// - Для кранов: уклон пути или горизонта
    pub fn as_percents(&self) -> f64 {
        match self.unit {
            ConfAngleUnit::Degrees  => self.value.to_radians().tan() * 100.0,
            ConfAngleUnit::Radians => self.value.tan() * 100.0,
            ConfAngleUnit::Gradians => (self.value * GRADIANS_TO_RADIANS).tan() * 100.0,
            ConfAngleUnit::Percent => self.value,
        }
    }
}
//
//
static CONF_ANGLE_RE: OnceLock<Regex> = OnceLock::new();
//
impl FromStr for ConfAngle {
    type Err = Error;
    fn from_str(input: &str) -> Result<ConfAngle, Self::Err> {
        log::trace!("ConfAngle.from_str | input: {}", input);
        let re = CONF_ANGLE_RE.get_or_init(|| RegexBuilder::new(
            // r"^[ \t]*([-+]?[\d]*\.?[\d]+)[ \t]*(deg|rad|gon|%|°){0,1}[ \t]*$"
            r"^[ \t]*([-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?)[ \t]*(deg|rad|gon|%|°)?[ \t]*$"
        ).multi_line(true).build().unwrap());
        let group_value = 1;
        let group_unit = 2;
        let caps = re.captures(input)
                .ok_or_else(|| Error::new("ConfAngle", "from_str").err(format!("Can't parse angle '{}'", input)))?;
        let value = &caps.get(group_value)
            .ok_or_else(|| Error::new("ConfAngle", "from_str").err(format!("Wrong angle value: '{}'", input)))?;
        let value = value.as_str().parse()
            .map_err(|err: std::num::ParseFloatError| Error::new("ConfAngle", "from_str").pass_with(format!("Can't parse angle '{}'", input), err.to_string()))?;
        let unit = match &caps.get(group_unit) {
            Some(u) => ConfAngleUnit::from_str(u.as_str()).map_err(|err| Error::new("ConfAngle", "from_str").pass_with(format!("Can't parse angle '{}'", input), err.to_string()))?,
            None => ConfAngleUnit::Radians,
        };
        Ok(ConfAngle::new(value, unit))
    }
}
//
//
impl std::fmt::Display for ConfAngle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}
impl Default for ConfAngleUnit {
    fn default() -> Self {
        Self::Radians
    }
}
//
//
impl Default for ConfAngle {
    ///
    /// Returns 0.0 Radians
    fn default() -> Self {
        Self {
            value: Default::default(),
            unit: Default::default(),
        }
    }
}
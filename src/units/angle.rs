use super::{HashMap, Lazy};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AngleUnit {
    Degree, // deg
    Radian, // rad
}

static ANGLE_UNITS: Lazy<HashMap<&'static str, AngleUnit>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("deg", AngleUnit::Degree);
    m.insert("rad", AngleUnit::Radian);
    m
});

impl AngleUnit {
    pub fn from_suffix(suffix: &str) -> Option<Self> {
        ANGLE_UNITS.get(suffix).copied()
    }
}

impl AngleUnit {
    pub fn to_radians(&self, value: f64) -> f64 {
        match self {
            AngleUnit::Radian => value,
            AngleUnit::Degree => value * std::f64::consts::PI / 180.0,
        }
    }

    pub fn to_degrees(&self, value: f64) -> f64 {
        match self {
            AngleUnit::Degree => value,
            AngleUnit::Radian => value * 180.0 / std::f64::consts::PI,
        }
    }
}

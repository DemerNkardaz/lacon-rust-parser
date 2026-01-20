#[derive(Debug, Clone, PartialEq)]
pub enum LengthUnit {
    Micrometer, // µm
    Nanometer,  // nm
    Millimeter, // mm
    Centimeter, // cm
    Meter,      // m
    Kilometer,  // km
    Megameter,  // Mm
    Mile,       // mi
    Em,         // em
    Rem,        // rem
    Point,      // pt
    Inch,       // in
    Pixel,      // px
}

#[derive(Debug, Clone, PartialEq)]
pub enum AngleUnit {
    Degree, // deg
    Radian, // rad
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeUnit {
    Nanosecond,  // ns
    Microsecond, // μs
    Millisecond, // ms
    Second,      // sec
    Minute,      // min
    Hour,        // hour
    Day,         // day
    Week,        // week
    Month,       // month
    Year,        // year
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataSizeUnit {
    Kilobyte, // Kb
    Megabyte, // MB
    Gigabyte, // GB
    Terabyte, // TB
}

#[derive(Debug, Clone, PartialEq)]
pub enum FrequencyUnit {
    Hertz,     // Hz
    Kilohertz, // kHz
    Megahertz, // MHz
    Gigahertz, // GHz
    Terahertz, // THz
}

impl LengthUnit {
    pub fn to_pixels(&self, value: f64) -> f64 {
        match self {
            LengthUnit::Pixel => value,
            LengthUnit::Inch => value * 96.0,
            LengthUnit::Point => value * 96.0 / 72.0,
            LengthUnit::Centimeter => value * 96.0 / 2.54,
            LengthUnit::Millimeter => value * 96.0 / 25.4,
            _ => value,
        }
    }

    pub fn to_meters(&self, value: f64) -> f64 {
        match self {
            LengthUnit::Meter => value,
            LengthUnit::Kilometer => value * 1000.0,
            LengthUnit::Centimeter => value / 100.0,
            LengthUnit::Millimeter => value / 1000.0,
            LengthUnit::Nanometer => value / 1_000_000_000.0,
            LengthUnit::Mile => value * 1609.344,
            LengthUnit::Inch => value * 0.0254,
            _ => value,
        }
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

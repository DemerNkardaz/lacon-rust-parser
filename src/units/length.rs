use crate::units::common::*;

#[derive(Debug, Clone, Copy, PartialEq)]
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

static LENGTH_UNITS: Lazy<HashMap<&'static str, LengthUnit>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("µm", LengthUnit::Micrometer);
    m.insert("um", LengthUnit::Micrometer);
    m.insert("nm", LengthUnit::Nanometer);
    m.insert("mm", LengthUnit::Millimeter);
    m.insert("cm", LengthUnit::Centimeter);
    m.insert("m", LengthUnit::Meter);
    m.insert("km", LengthUnit::Kilometer);
    m.insert("Mm", LengthUnit::Megameter);
    m.insert("mi", LengthUnit::Mile);
    m.insert("em", LengthUnit::Em);
    m.insert("rem", LengthUnit::Rem);
    m.insert("pt", LengthUnit::Point);
    m.insert("in", LengthUnit::Inch);
    m.insert("px", LengthUnit::Pixel);
    m
});

impl LengthUnit {
    pub fn from_suffix(suffix: &str) -> Option<Self> {
        LENGTH_UNITS.get(suffix).copied()
    }
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

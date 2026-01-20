use crate::units::common::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrequencyUnit {
    Hertz,     // Hz
    Kilohertz, // kHz
    Megahertz, // MHz
    Gigahertz, // GHz
    Terahertz, // THz
}

static FREQUENCY_UNITS: Lazy<HashMap<&'static str, FrequencyUnit>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Hz", FrequencyUnit::Hertz);
    m.insert("hz", FrequencyUnit::Hertz);
    m.insert("kHz", FrequencyUnit::Kilohertz);
    m.insert("khz", FrequencyUnit::Kilohertz);
    m.insert("MHz", FrequencyUnit::Megahertz);
    m.insert("mhz", FrequencyUnit::Megahertz);
    m.insert("GHz", FrequencyUnit::Gigahertz);
    m.insert("ghz", FrequencyUnit::Gigahertz);
    m.insert("THz", FrequencyUnit::Terahertz);
    m.insert("thz", FrequencyUnit::Terahertz);
    m
});

impl FrequencyUnit {
    pub fn from_suffix(suffix: &str) -> Option<Self> {
        FREQUENCY_UNITS.get(suffix).copied()
    }
}

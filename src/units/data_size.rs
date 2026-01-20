use super::{HashMap, Lazy};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataSizeUnit {
    Kilobyte, // Kb
    Megabyte, // MB
    Gigabyte, // GB
    Terabyte, // TB
}

static DATA_SIZE_UNITS: Lazy<HashMap<&'static str, DataSizeUnit>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("KB", DataSizeUnit::Kilobyte);
    m.insert("Kb", DataSizeUnit::Kilobyte);
    m.insert("kb", DataSizeUnit::Kilobyte);
    m.insert("MB", DataSizeUnit::Megabyte);
    m.insert("Mb", DataSizeUnit::Megabyte);
    m.insert("mb", DataSizeUnit::Megabyte);
    m.insert("GB", DataSizeUnit::Gigabyte);
    m.insert("Gb", DataSizeUnit::Gigabyte);
    m.insert("gb", DataSizeUnit::Gigabyte);
    m.insert("TB", DataSizeUnit::Terabyte);
    m.insert("Tb", DataSizeUnit::Terabyte);
    m.insert("tb", DataSizeUnit::Terabyte);
    m
});

impl DataSizeUnit {
    pub fn from_suffix(suffix: &str) -> Option<Self> {
        DATA_SIZE_UNITS.get(suffix).copied()
    }
}

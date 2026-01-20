use crate::units::common::*;

#[derive(Debug, Clone, Copy, PartialEq)]
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

static TIME_UNITS: Lazy<HashMap<&'static str, TimeUnit>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("ns", TimeUnit::Nanosecond);
    m.insert("μs", TimeUnit::Microsecond);
    m.insert("us", TimeUnit::Microsecond);
    m.insert("ms", TimeUnit::Millisecond);
    m.insert("sec", TimeUnit::Second);
    m.insert("s", TimeUnit::Second);
    m.insert("min", TimeUnit::Minute);
    m.insert("hour", TimeUnit::Hour);
    m.insert("h", TimeUnit::Hour);
    m.insert("day", TimeUnit::Day);
    m.insert("d", TimeUnit::Day);
    m.insert("week", TimeUnit::Week);
    m.insert("w", TimeUnit::Week);
    m.insert("month", TimeUnit::Month);
    m.insert("year", TimeUnit::Year);
    m.insert("y", TimeUnit::Year);
    m
});

impl TimeUnit {
    pub fn from_suffix(suffix: &str) -> Option<Self> {
        TIME_UNITS.get(suffix).copied()
    }
}

use crate::{AngleUnit, DataSizeUnit, FrequencyUnit, LaCoNValue, LengthUnit, TimeUnit};

pub fn parse_string_literal(input: &str) -> LaCoNValue {
    LaCoNValue::StringLiteral(input.to_string())
}

pub fn parse_typed_value(input: &str) -> Option<LaCoNValue> {
    let input = input.trim();
    let input_lower = input.to_lowercase();

    // Специальные значения
    match input_lower.as_str() {
        "auto" => return Some(LaCoNValue::Auto),
        "none" => return Some(LaCoNValue::None),
        "true" => return Some(LaCoNValue::Bool(true)),
        "false" => return Some(LaCoNValue::Bool(false)),
        _ => {}
    }

    // Единицы измерения
    if let Some(val) = parse_length(input) {
        return Some(val);
    }
    if let Some(val) = parse_angle(input) {
        return Some(val);
    }
    if let Some(val) = parse_time(input) {
        return Some(val);
    }
    if let Some(val) = parse_data_size(input) {
        return Some(val);
    }
    if let Some(val) = parse_frequency(input) {
        return Some(val);
    }
    if let Some(val) = parse_percentage(input) {
        return Some(val);
    }
    if let Some(val) = parse_fraction(input) {
        return Some(val);
    }

    if let Ok(i) = input.parse::<i64>() {
        return Some(LaCoNValue::Int(i));
    }
    if let Ok(f) = input.parse::<f64>() {
        return Some(LaCoNValue::Float(f));
    }

    None
}

pub fn parse_value(input: &str) -> Option<LaCoNValue> {
    parse_typed_value(input).or(Some(LaCoNValue::String(input.to_string())))
}

pub fn parse_length(input: &str) -> Option<LaCoNValue> {
    let input = input.trim();

    let units = [
        ("nm", LengthUnit::Nanometer),
        ("mm", LengthUnit::Millimeter),
        ("cm", LengthUnit::Centimeter),
        ("Mm", LengthUnit::Megameter),
        ("km", LengthUnit::Kilometer),
        ("mi", LengthUnit::Mile),
        ("em", LengthUnit::Em),
        ("rem", LengthUnit::Rem),
        ("pt", LengthUnit::Point),
        ("in", LengthUnit::Inch),
        ("px", LengthUnit::Pixel),
        ("m", LengthUnit::Meter),
    ];

    for (suffix, unit) in units {
        if let Some(value_str) = input.strip_suffix(suffix) {
            if let Ok(num) = value_str.parse::<f64>() {
                return Some(LaCoNValue::Length(num, unit));
            }
        }
    }

    None
}

pub fn parse_angle(input: &str) -> Option<LaCoNValue> {
    let input = input.trim();

    let units = [("deg", AngleUnit::Degree), ("rad", AngleUnit::Radian)];

    for (suffix, unit) in units {
        if let Some(value_str) = input.strip_suffix(suffix) {
            if let Ok(num) = value_str.parse::<f64>() {
                return Some(LaCoNValue::Angle(num, unit));
            }
        }
    }

    None
}

pub fn parse_time(input: &str) -> Option<LaCoNValue> {
    let input = input.trim();

    let units = [
        ("sec", TimeUnit::Second),
        ("min", TimeUnit::Minute),
        ("hour", TimeUnit::Hour),
        ("day", TimeUnit::Day),
    ];

    for (suffix, unit) in units {
        if let Some(value_str) = input.strip_suffix(suffix) {
            if let Ok(num) = value_str.parse::<f64>() {
                return Some(LaCoNValue::Time(num, unit));
            }
        }
    }

    None
}

pub fn parse_data_size(input: &str) -> Option<LaCoNValue> {
    let input = input.trim();

    let units = [
        ("KB", DataSizeUnit::Kilobyte),
        ("MB", DataSizeUnit::Megabyte),
        ("GB", DataSizeUnit::Gigabyte),
        ("TB", DataSizeUnit::Terabyte),
        ("Kb", DataSizeUnit::Kilobyte),
        ("Mb", DataSizeUnit::Megabyte),
        ("Gb", DataSizeUnit::Gigabyte),
        ("Tb", DataSizeUnit::Terabyte),
    ];

    for (suffix, unit) in units {
        if let Some(value_str) = input.strip_suffix(suffix) {
            if let Ok(num) = value_str.parse::<f64>() {
                return Some(LaCoNValue::DataSize(num, unit));
            }
        }
    }

    None
}

pub fn parse_frequency(input: &str) -> Option<LaCoNValue> {
    let input = input.trim();

    let units = [
        ("kHz", FrequencyUnit::Kilohertz),
        ("Hz", FrequencyUnit::Hertz),
    ];

    for (suffix, unit) in units {
        if let Some(value_str) = input.strip_suffix(suffix) {
            if let Ok(num) = value_str.parse::<f64>() {
                return Some(LaCoNValue::Frequency(num, unit));
            }
        }
    }

    None
}

pub fn parse_percentage(input: &str) -> Option<LaCoNValue> {
    let input = input.trim();

    if let Some(value_str) = input.strip_suffix('%') {
        if let Ok(num) = value_str.parse::<f64>() {
            return Some(LaCoNValue::Percentage(num));
        }
    }

    None
}

pub fn parse_fraction(input: &str) -> Option<LaCoNValue> {
    let input = input.trim();

    if let Some(value_str) = input.strip_suffix("fr") {
        if let Ok(num) = value_str.parse::<f64>() {
            return Some(LaCoNValue::Fraction(num));
        }
    }

    None
}

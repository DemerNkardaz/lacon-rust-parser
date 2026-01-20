use crate::units::*;
use std::collections::HashMap;

// src/value.rs
#[derive(Debug, Clone, PartialEq)]
pub enum LaCoNValue {
    // Типизированные значения (без кавычек в исходнике)
    Int(i64),
    Float(f64),
    Bool(bool),
    Array(Vec<LaCoNValue>),
    Dictionary(HashMap<String, LaCoNValue>),
    Length(f64, LengthUnit),
    Angle(f64, AngleUnit),
    Time(f64, TimeUnit),
    DataSize(f64, DataSizeUnit),
    Frequency(f64, FrequencyUnit),
    Percentage(f64),
    Fraction(f64),
    Auto,
    None,

    StringLiteral(String),

    String(String),
}

impl LaCoNValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            LaCoNValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            LaCoNValue::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            LaCoNValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            LaCoNValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<LaCoNValue>> {
        match self {
            LaCoNValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_dict(&self) -> Option<&HashMap<String, LaCoNValue>> {
        match self {
            LaCoNValue::Dictionary(dict) => Some(dict),
            _ => None,
        }
    }

    pub fn is_auto(&self) -> bool {
        matches!(self, LaCoNValue::Auto)
    }

    pub fn is_none(&self) -> bool {
        matches!(self, LaCoNValue::None)
    }
}

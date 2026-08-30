#[derive(Debug)]
pub enum ValueType {
    UInt8,
    Int8,
    UInt16,
    Int16,
    UInt32,
    Int32,
    Float32,
    Bool,
    String,
    Array,
    UInt64,
    Int64,
    Float64,
}

impl ValueType {
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(ValueType::UInt8),
            1 => Some(ValueType::Int8),
            2 => Some(ValueType::UInt16),
            3 => Some(ValueType::Int16),
            4 => Some(ValueType::UInt32),
            5 => Some(ValueType::Int32),
            6 => Some(ValueType::Float32),
            7 => Some(ValueType::Bool),
            8 => Some(ValueType::String),
            9 => Some(ValueType::Array),
            10 => Some(ValueType::UInt64),
            11 => Some(ValueType::Int64),
            12 => Some(ValueType::Float64),
            _ => None,
        }
    }
}
use std::ops::Range;

use bevy::prelude::Color;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AttributeKind {
    Color,
    String,
    StaticStr,
    Uint16,
    Uint32,
    Uint64,
    Sint16,
    Sint32,
    Sint64,
    Float32,
    Float64,
    RangeU16,
    RangeU32,
    RangeI16,
    RangeI32,
    RangeF32,
    StaticStrX6,
    Uint32X6,
    Sint32X6,
    Float32X6,
}

#[derive(Debug, Clone)]
pub enum AttributeValue {
    Color(Color),
    String(String),
    StaticStr(&'static str),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Sint16(i16),
    Sint32(i32),
    Sint64(i64),
    Float32(f32),
    Float64(f64),
    RangeU16(Range<u16>),
    RangeU32(Range<u32>),
    RangeI16(Range<i16>),
    RangeI32(Range<i32>),
    RangeF32(Range<f32>),
    StaticStrX6([&'static str; 6]),
    Uint32X6([u32; 6]),
    Sint32X6([i32; 6]),
    Float32X6([f32; 6]),
}

impl From<&AttributeValue> for AttributeKind {
    fn from(value: &AttributeValue) -> Self {
        match value {
            AttributeValue::Color(_) => AttributeKind::Color,
            AttributeValue::String(_) => AttributeKind::String,
            AttributeValue::StaticStr(_) => AttributeKind::StaticStr,
            AttributeValue::Uint16(_) => AttributeKind::Uint16,
            AttributeValue::Uint32(_) => AttributeKind::Uint32,
            AttributeValue::Uint64(_) => AttributeKind::Uint64,
            AttributeValue::Sint16(_) => AttributeKind::Sint16,
            AttributeValue::Sint32(_) => AttributeKind::Sint32,
            AttributeValue::Sint64(_) => AttributeKind::Sint64,
            AttributeValue::Float32(_) => AttributeKind::Float32,
            AttributeValue::Float64(_) => AttributeKind::Float64,
            AttributeValue::RangeU16(_) => AttributeKind::RangeU16,
            AttributeValue::RangeU32(_) => AttributeKind::RangeU32,
            AttributeValue::RangeI16(_) => AttributeKind::RangeI16,
            AttributeValue::RangeI32(_) => AttributeKind::RangeI32,
            AttributeValue::RangeF32(_) => AttributeKind::RangeF32,
            AttributeValue::StaticStrX6(_) => AttributeKind::StaticStrX6,
            AttributeValue::Uint32X6(_) => AttributeKind::Uint32X6,
            AttributeValue::Sint32X6(_) => AttributeKind::Sint32X6,
            AttributeValue::Float32X6(_) => AttributeKind::Float32X6,
        }
    }
}

// TODO: Reduce amount of TryFrom impls. Maybe with metaprogramming?

impl TryFrom<AttributeValue> for Range<u16> {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::RangeU16(value) => Ok(value),
            AttributeValue::RangeU32(value) => Ok(value.start as u16..value.end as u16),
            AttributeValue::RangeI16(value) => Ok(value.start as u16..value.end as u16),
            AttributeValue::RangeI32(value) => Ok(value.start as u16..value.end as u16),
            AttributeValue::RangeF32(value) => Ok(value.start as u16..value.end as u16),
            _ => Err(()),
        }
    }
}

impl TryFrom<AttributeValue> for Range<u32> {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::RangeU32(value) => Ok(value),
            AttributeValue::RangeU16(value) => Ok(value.start as u32..value.end as u32),
            AttributeValue::RangeI16(value) => Ok(value.start as u32..value.end as u32),
            AttributeValue::RangeI32(value) => Ok(value.start as u32..value.end as u32),
            AttributeValue::RangeF32(value) => Ok(value.start as u32..value.end as u32),
            _ => Err(()),
        }
    }
}

impl TryFrom<AttributeValue> for Range<i16> {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::RangeI16(value) => Ok(value),
            AttributeValue::RangeU16(value) => Ok(value.start as i16..value.end as i16),
            AttributeValue::RangeU32(value) => Ok(value.start as i16..value.end as i16),
            AttributeValue::RangeI32(value) => Ok(value.start as i16..value.end as i16),
            AttributeValue::RangeF32(value) => Ok(value.start as i16..value.end as i16),
            _ => Err(()),
        }
    }
}

impl TryFrom<AttributeValue> for Range<i32> {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::RangeI32(value) => Ok(value),
            AttributeValue::RangeU16(value) => Ok(value.start as i32..value.end as i32),
            AttributeValue::RangeU32(value) => Ok(value.start as i32..value.end as i32),
            AttributeValue::RangeI16(value) => Ok(value.start as i32..value.end as i32),
            AttributeValue::RangeF32(value) => Ok(value.start as i32..value.end as i32),
            _ => Err(()),
        }
    }
}

impl TryFrom<AttributeValue> for Range<f32> {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::RangeF32(value) => Ok(value),
            AttributeValue::RangeU16(value) => Ok(value.start as f32..value.end as f32),
            AttributeValue::RangeU32(value) => Ok(value.start as f32..value.end as f32),
            AttributeValue::RangeI16(value) => Ok(value.start as f32..value.end as f32),
            AttributeValue::RangeI32(value) => Ok(value.start as f32..value.end as f32),
            _ => Err(()),
        }
    }
}

impl TryFrom<AttributeValue> for Color {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::Color(value) => Ok(value),
            _ => Err(())
        }
    }
}

impl TryFrom<AttributeValue> for String {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::String(value) => Ok(value),
            _ => Err(())
        }
    }
}

impl TryFrom<AttributeValue> for &'static str {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::StaticStr(value) => Ok(value),
            _ => Err(())
        }
    }
}

impl TryFrom<AttributeValue> for u16 {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::Uint16(value) => Ok(value),
            AttributeValue::Uint32(value) => Ok(value as u16),
            AttributeValue::Uint64(value) => Ok(value as u16),
            AttributeValue::Sint16(value) => Ok(value as u16),
            AttributeValue::Sint32(value) => Ok(value as u16),
            AttributeValue::Sint64(value) => Ok(value as u16),
            AttributeValue::Float32(value) => Ok(value as u16),
            AttributeValue::Float64(value) => Ok(value as u16),
            _ => Err(()),
        }
    }
}

impl TryFrom<AttributeValue> for u32 {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::Uint32(value) => Ok(value),
            AttributeValue::Uint16(value) => Ok(value as u32),
            AttributeValue::Uint64(value) => Ok(value as u32),
            AttributeValue::Sint16(value) => Ok(value as u32),
            AttributeValue::Sint32(value) => Ok(value as u32),
            AttributeValue::Sint64(value) => Ok(value as u32),
            AttributeValue::Float32(value) => Ok(value as u32),
            AttributeValue::Float64(value) => Ok(value as u32),
            _ => Err(()),
        }
    }
}

impl TryFrom<AttributeValue> for u64 {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::Uint64(value) => Ok(value),
            AttributeValue::Uint16(value) => Ok(value as u64),
            AttributeValue::Uint32(value) => Ok(value as u64),
            AttributeValue::Sint16(value) => Ok(value as u64),
            AttributeValue::Sint32(value) => Ok(value as u64),
            AttributeValue::Sint64(value) => Ok(value as u64),
            AttributeValue::Float32(value) => Ok(value as u64),
            AttributeValue::Float64(value) => Ok(value as u64),
            _ => Err(()),
        }
    }
}

impl TryFrom<AttributeValue> for i16 {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::Sint16(value) => Ok(value),
            AttributeValue::Uint16(value) => Ok(value as i16),
            AttributeValue::Uint32(value) => Ok(value as i16),
            AttributeValue::Uint64(value) => Ok(value as i16),
            AttributeValue::Sint32(value) => Ok(value as i16),
            AttributeValue::Sint64(value) => Ok(value as i16),
            AttributeValue::Float32(value) => Ok(value as i16),
            AttributeValue::Float64(value) => Ok(value as i16),
            _ => Err(()),
        }
    }
}

impl TryFrom<AttributeValue> for i32 {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::Sint32(value) => Ok(value),
            AttributeValue::Uint16(value) => Ok(value as i32),
            AttributeValue::Uint32(value) => Ok(value as i32),
            AttributeValue::Uint64(value) => Ok(value as i32),
            AttributeValue::Sint16(value) => Ok(value as i32),
            AttributeValue::Sint64(value) => Ok(value as i32),
            AttributeValue::Float32(value) => Ok(value as i32),
            AttributeValue::Float64(value) => Ok(value as i32),
            _ => Err(()),
        }
    }
}

impl TryFrom<AttributeValue> for i64 {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::Sint64(value) => Ok(value),
            AttributeValue::Uint16(value) => Ok(value as i64),
            AttributeValue::Uint32(value) => Ok(value as i64),
            AttributeValue::Uint64(value) => Ok(value as i64),
            AttributeValue::Sint16(value) => Ok(value as i64),
            AttributeValue::Sint32(value) => Ok(value as i64),
            AttributeValue::Float32(value) => Ok(value as i64),
            AttributeValue::Float64(value) => Ok(value as i64),
            _ => Err(()),
        }
    }
}

impl TryFrom<AttributeValue> for f32 {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::Float32(value) => Ok(value),
            AttributeValue::Uint16(value) => Ok(value as f32),
            AttributeValue::Uint32(value) => Ok(value as f32),
            AttributeValue::Uint64(value) => Ok(value as f32),
            AttributeValue::Sint16(value) => Ok(value as f32),
            AttributeValue::Sint32(value) => Ok(value as f32),
            AttributeValue::Sint64(value) => Ok(value as f32),
            AttributeValue::Float64(value) => Ok(value as f32),
            _ => Err(()),
        }
    }
}

impl TryFrom<AttributeValue> for f64 {
    type Error = ();

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::Float64(value) => Ok(value),
            AttributeValue::Uint16(value) => Ok(value as f64),
            AttributeValue::Uint32(value) => Ok(value as f64),
            AttributeValue::Uint64(value) => Ok(value as f64),
            AttributeValue::Sint16(value) => Ok(value as f64),
            AttributeValue::Sint32(value) => Ok(value as f64),
            AttributeValue::Sint64(value) => Ok(value as f64),
            AttributeValue::Float32(value) => Ok(value as f64),
            _ => Err(()),
        }
    }
}
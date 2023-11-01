use std::fmt::Display;

use serde::Deserialize;

/// Separators for processing strings into content identifiers.
pub static ID_SEPARATORS: &[char] = &[':', '/'];

/// The engine's reserved content package name.
pub(crate) const ENGINE_ID: IdentifierSegment = IdentifierSegment::StaticStr("engine");

/// A segment of a [`ContentIdentifier`] value.
#[derive(Debug, Clone, Hash, PartialOrd, Ord)]
pub enum IdentifierSegment {
    StaticStr(&'static str),
    BoxedStr(Box<str>),
}

impl PartialEq for IdentifierSegment {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::StaticStr(l0), Self::StaticStr(r0)) => l0 == r0,
            (Self::StaticStr(l0), Self::BoxedStr(r0)) => l0.as_bytes() == r0.as_bytes(),
            (Self::BoxedStr(l0), Self::BoxedStr(r0)) => l0 == r0,
            (Self::BoxedStr(l0), Self::StaticStr(r0)) => l0.as_bytes() == r0.as_bytes(),
            _ => false,
        }
    }
}

impl Eq for IdentifierSegment {}

impl Display for IdentifierSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentifierSegment::StaticStr(v) => f.write_str(v),
            IdentifierSegment::BoxedStr(v) => f.write_str(v),
        }
    }
}

impl From<&str> for IdentifierSegment {
    /// Creates an `Identifier` value from a string slice by cloning.
    fn from(value: &str) -> Self {
        Self::BoxedStr(value.into())
    }
}

impl<'de> Deserialize<'de> for IdentifierSegment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct SegmentVisitor;
        impl <'de> serde::de::Visitor<'de> for SegmentVisitor {
            type Value = IdentifierSegment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string value")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where E: serde::de::Error, {
                Ok(IdentifierSegment::BoxedStr(v.into()))
            }
        }

        deserializer.deserialize_str(SegmentVisitor)
    }
}

/// An object used to identify content, like blocks or item IDs.
/// 
/// For example, if two content packages added 'copper',
/// they would have the same `identifier` value,
/// but a different `namespace`, therefore being distinct.
///
/// A [`ContentIdentifier`] also has a `variant` field.
/// Normal `PartialEq`/`Eq` comparisons will **ignore** this field.
/// The `eq_variant` method can be used to compare all fields.
#[derive(Debug, Hash, Clone, PartialOrd, Ord)]
pub struct ContentIdentifier {
    pub namespace: IdentifierSegment,
    pub identifier: IdentifierSegment,
    pub variant: Option<IdentifierSegment>,
}

impl ContentIdentifier {
    /// Compares two [`ContentIdentifier`]s, but also compares the `variant` field.
    pub fn eq_variant(&self, other: &Self) -> bool {
        if !self.eq(other) { return false }
        self.variant.eq(&other.variant)
    }
}

impl PartialEq for ContentIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.namespace == other.namespace && self.identifier == other.identifier
    }
}

impl Eq for ContentIdentifier {}

impl Display for ContentIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.variant {
            Some(variant) => f.write_str(&format!("{}:{}/{}", self.namespace, self.identifier, variant)),
            None => f.write_str(&format!("{}:{}", self.namespace, self.identifier)),
        }
    }
}

impl TryFrom<&str> for ContentIdentifier {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(ID_SEPARATORS);
        let namespace = if let Some(next) = split.next() { IdentifierSegment::from(next) } else { return Err(()) };
        let identifier = if let Some(next) = split.next() { IdentifierSegment::from(next) } else { return Err(()) };
        let variant = match split.next() {
            Some(v) => Some(IdentifierSegment::from(v)),
            None => None,
        };

        Ok(Self {
            namespace,
            identifier,
            variant
        })
    }
}

impl<'de> Deserialize<'de> for ContentIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        todo!()
    }
}
use std::fmt::Display;

/// The engine's reserved content package name.
pub(crate) const ENGINE_ID: Identifier = Identifier::StaticStr("engine");

/// An identifier value, used in [ContentIdentifier].
/// 
/// Implements `PartialEq` and `Eq`, with special behavior.
/// `StaticStr` and `BoxedStr` are equal to themselves and eachother, but `Integer` is only equal to itself.
#[derive(Debug, Clone, Hash, PartialOrd, Ord)]
pub enum Identifier {
    StaticStr(&'static str),
    BoxedStr(Box<str>),
    Integer(i64),
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::StaticStr(l0), Self::StaticStr(r0)) => l0 == r0,
            (Self::StaticStr(l0), Self::BoxedStr(r0)) => l0.as_bytes() == r0.as_bytes(),
            (Self::BoxedStr(l0), Self::BoxedStr(r0)) => l0 == r0,
            (Self::BoxedStr(l0), Self::StaticStr(r0)) => l0.as_bytes() == r0.as_bytes(),
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Identifier {}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Identifier::StaticStr(v) => f.write_str(v),
            Identifier::BoxedStr(v) => f.write_str(v),
            Identifier::Integer(v) => f.write_str(&format!("{v}")),
        }
    }
}

/// An object used to identify content, like blocks or item IDs.
/// 
/// For example, if two content packages added 'copper',
/// they would have the same `identifier` value,
/// but a different `namespace`, therefore being distinct.
///
/// A [NamespacedIdentifier] also has a `variant` field.
/// Normal `PartialEq`/`Eq` comparisons will **ignore** this field.
/// The `eq_variant` method can be used to compare all fields.
#[derive(Debug, Hash, PartialOrd, Ord)]
pub struct ContentIdentifier {
    pub namespace: Identifier,
    pub identifier: Identifier,
    pub variant: Option<Identifier>,
}

impl ContentIdentifier {
    /// Compares two [NamespacedIdentifier]s, but also compares the `variant` field.
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
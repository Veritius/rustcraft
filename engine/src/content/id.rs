use std::fmt::Display;

/// The engine's reserved content package name.
pub(crate) const ENGINE_ID: Identifier = Identifier::StaticStr("engine");

/// A segment of a [`ContentIdentifier`] value.
/// 
/// Implements `PartialEq` and `Eq`, with special behavior.
/// `StaticStr` and `BoxedStr` are equal to themselves and eachother, but `Integer` is only equal to itself.
/// Note that when using `From<&str>`, the `Integer` type will be prioritised.
#[derive(Debug, Clone, Hash, PartialOrd, Ord)]
pub enum Identifier {
    StaticStr(&'static str),
    BoxedStr(Box<str>),
    Integer(i32),
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

impl From<i32> for Identifier {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl From<&str> for Identifier {
    /// Creates an `Identifier` value from a string slice.
    /// 
    /// This will try to create an `Integer` variant first by parsing the input as an `i32`, and if that doesn't work, the `BoxedStr` variant will be used.
    fn from(value: &str) -> Self {
        match value.parse::<i32>() {
            Ok(i) => Self::Integer(i),
            Err(_) => Self::BoxedStr(value.into()),
        }
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
    pub namespace: Identifier,
    pub identifier: Identifier,
    pub variant: Option<Identifier>,
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
        // Read a ContentIdentifier from the following form:
        // namespace:identifier/variant

        if !value.contains(':') { return Err(()) }

        let mut colon_split = value.split(':');

        // Namespace
        let namespace = colon_split.next().unwrap();
        if namespace.len() == 0 { return Err(()) }
        let namespace = Identifier::from(namespace);

        // Identifier and variant
        let right_segment = colon_split.next().unwrap();
        if right_segment.len() == 0 { return Err(()) }
        if right_segment.contains('/') {
            let mut slash_split = right_segment.split('/');
            let identifier = slash_split.next().unwrap();
            let variant = slash_split.next().unwrap();

            if identifier.len() == 0 { return Err(()) }

            // Identifier and variant
            return Ok(Self {
                namespace,
                identifier: Identifier::from(identifier),
                variant: Some(Identifier::from(variant))
            })
        } else {
            // Just an identifier
            return Ok(Self {
                namespace,
                identifier: Identifier::from(right_segment),
                variant: None,
            })
        }
    }
}
use std::collections::BTreeMap;
use crate::content::id::ContentIdentifier;
use super::value::Attribute;

#[derive(Debug)]
pub struct Attributes {
    map: BTreeMap<ContentIdentifier, Attribute>,
}

impl Attributes {
    pub fn new() -> Self {
        Self { map: BTreeMap::default() }
    }

    pub fn insert(&mut self, identifier: ContentIdentifier, attribute: Attribute) {
        self.map.insert(identifier, attribute);
    }

    pub fn get(&self, identifier: &ContentIdentifier) -> Option<&Attribute> {
        self.map.get(identifier)
    }
}

impl FromIterator<(ContentIdentifier, Attribute)> for Attributes {
    fn from_iter<T: IntoIterator<Item = (ContentIdentifier, Attribute)>>(iter: T) -> Self {
        Self {
            map: BTreeMap::from_iter(iter),
        }
    }
}
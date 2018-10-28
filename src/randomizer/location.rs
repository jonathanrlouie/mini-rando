use super::item::{LabelledItem};
use std::hash::{Hash, Hasher};
use std::fmt;

pub struct IsAccessible(pub Box<Fn(&[LabelledItem]) -> bool>);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocId(pub u64);

impl fmt::Display for LocId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Location(pub LocId, pub IsAccessible);

impl PartialEq for Location {
    fn eq(&self, other: &Location) -> bool {
        let Location(loc_id, _) = *self;
        let Location(other_loc_id, _) = *other;
        loc_id == other_loc_id
    }
}

impl Eq for Location {}

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Location(loc_id, _) = *self;
        loc_id.hash(state);
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Location(loc_id, _) = *self;
        write!(f, "Location {{ loc_id: {} }}", loc_id)
    }
}

pub fn has_item(items: &[LabelledItem], item: LabelledItem) -> bool {
    items.iter().any(|&assumed_item| assumed_item == item)
}
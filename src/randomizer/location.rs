use super::item::{LabelledItem};
use std::hash::{Hash, Hasher};
use std::fmt;

pub struct IsAccessible<F: Fn(&[LabelledItem]) -> bool>(pub fn() -> F);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LocId {
    Loc0,
    Loc1,
    Loc2,
    Loc3,
    Loc4,
    Loc5
}

pub struct Location<F: Fn(&[LabelledItem]) -> bool>(pub LocId, pub IsAccessible<F>);

impl<F: Fn(&[LabelledItem]) -> bool> PartialEq for Location<F> {
    fn eq(&self, other: &Location<F>) -> bool {
        let &Location(loc_id, _) = self;
        let &Location(other_loc_id, _) = other;
        loc_id == other_loc_id
    }
}

impl<F: Fn(&[LabelledItem]) -> bool> Eq for Location<F> {}

impl<F: Fn(&[LabelledItem]) -> bool> Hash for Location<F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let &Location(loc_id, _) = self;
        loc_id.hash(state);
    }
}

impl<F: Fn(&[LabelledItem]) -> bool> fmt::Debug for Location<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &Location(loc_id, _) = self;
        write!(f, "Location {{ loc_id: {:?} }}", loc_id)
    }
}

pub fn has_item(items: &[LabelledItem], item: LabelledItem) -> bool {
    items.iter().any(|&assumed_item| assumed_item == item)
}
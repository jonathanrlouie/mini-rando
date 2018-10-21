use super::item::{Item, LabelledItem};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LocId(pub u32);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IsAccessible<'a>(pub &'a Fn(&[LabelledItem]) -> bool);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Location<'a>(pub LocId, pub IsAccessible<'a>);

/*
impl<F> Location<F> {


    // items is the list of assumed accessible items
    /*
    pub fn is_accessible(&self, items: &[LabelledItem]) -> bool {
        match self {
            Location::Location0 => has_item(items, LabelledItem::Progression(Item::Item0)),
            Location::Location1 => {
                has_item(items, LabelledItem::Progression(Item::Item0)) &&
                has_item(items, LabelledItem::Progression(Item::Item1))
            },
            Location::Location2 => true,
            Location::Location3 => true,
            Location::Location4 => true,
            Location::Location5 => true
        }
    }*/
}*/

pub fn has_item(items: &[LabelledItem], item: LabelledItem) -> bool {
    items.iter().any(|&assumed_item| assumed_item == item)
}
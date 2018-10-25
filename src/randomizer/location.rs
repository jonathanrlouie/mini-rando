use super::item::{Item, LabelledItem};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Location {
    Location0,
    Location1,
    Location2,
    Location3,
    Location4,
    Location5
}

impl Location {
    // items is the list of assumed accessible items
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
    }
}

pub fn has_item(items: &[LabelledItem], item: LabelledItem) -> bool {
    items.iter().any(|&assumed_item| assumed_item == item)
}
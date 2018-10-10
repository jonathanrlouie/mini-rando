use super::item::{LabelledItem};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Location{
    Location0,
    Location1,
    Location2
}

impl Location {
    pub fn is_accessible(&self, items: &[LabelledItem]) -> bool {
        match self {
            Location::Location0 => true,
            Location::Location1 => true,
            Location::Location2 => true
        }
    }
}
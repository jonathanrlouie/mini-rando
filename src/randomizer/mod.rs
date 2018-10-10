pub mod item;
pub mod location;
pub mod filler;

#[derive(Debug)]
pub struct FilledLocation(pub item::LabelledItem, pub location::Location);
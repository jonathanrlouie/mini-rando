#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Item {
    Item0,
    Item1,
    Item2,
    Item3
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LabelledItem {
    Progression(Item),
    Nice(Item),
    Junk(Item)
}
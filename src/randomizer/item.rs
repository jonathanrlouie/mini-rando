#[derive(Copy, Clone, Debug)]
pub enum Item {
    Item0,
    Item1,
    Item2
}

#[derive(Copy, Clone, Debug)]
pub enum LabelledItem {
    Progression(Item),
    Nice(Item),
    Junk(Item)
}
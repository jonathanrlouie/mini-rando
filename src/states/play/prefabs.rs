use amethyst::{
    ecs::prelude::{Entity, Component, WriteStorage, VecStorage},
    assets::{PrefabData, PrefabError, ProgressCounter}
};

#[derive(Deserialize, Serialize, PrefabData)]
pub struct ItemLocation {
    pub was_checked: WasChecked
}

#[derive(Default, Deserialize, Serialize, Clone, PrefabData)]
#[prefab(Component)]
pub struct WasChecked {
    pub was_checked: bool
}

impl Component for WasChecked {
    type Storage = VecStorage<Self>;
}
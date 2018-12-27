use amethyst::{
    ecs::prelude::{Entity, Component, WriteStorage, DenseVecStorage},
    assets::{PrefabData, PrefabError, ProgressCounter}
};

#[derive(Default, Deserialize, Serialize, Clone, PrefabData)]
#[prefab(Component)]
pub struct WasChecked {
    was_checked: bool
}

impl Component for WasChecked {
    type Storage = DenseVecStorage<Self>;
}
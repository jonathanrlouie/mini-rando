use amethyst::{
    prelude::*,
    ecs::{
        Component, DenseVecStorage,
        prelude::Entity,
        error::Error
    },
    core::specs::{
        error::Error as PrefabError,
        WriteStorage
    },
    assets::PrefabData
};

#[derive(Copy, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct WasChecked {
    was_checked: bool
}

impl Component for WasChecked {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct LocationPrefabData {
    was_checked: Option<WasChecked>
}

// TODO: remove this when derive macro lands in next version of Amethyst
impl<'a> PrefabData<'a> for LocationPrefabData {
    type SystemData = <Option<WasChecked> as PrefabData<'a>>::SystemData;

    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        entities: &[Entity],
    ) -> Result<(), Error> {
        let was_checked = system_data;
        self.was_checked.add_to_entity(entity, was_checked, entities)?;
        Ok(())
    }
}

impl<'a> PrefabData<'a> for WasChecked {
    type SystemData = WriteStorage<'a, Self>;

    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        storage: &mut Self::SystemData,
        entities: &[Entity],
    ) -> Result<(), PrefabError> {
        storage.insert(entity, self.clone()).map(|_| ())
    }
}
use amethyst::{
    prelude::*,
    ecs::prelude::Entity,
    ui::{UiTransform}
};
use super::super::game_data::MiniRandoGameData;

pub trait ButtonTrans {
    fn button_click_trans<'a, 'b>(
        &self,
        world: &mut World,
        target: Entity
    ) -> Trans<MiniRandoGameData<'a, 'b>, StateEvent> {
        let button_id = get_button_id(world, target);
        button_id
            .map(|id| self.get_trans_for_id(world, id.as_str()))
            .unwrap_or_else(|| Trans::None)
    }

    fn get_trans_for_id<'a, 'b>(&self, world: &mut World, button_id: &str) -> Trans<MiniRandoGameData<'a, 'b>, StateEvent>;
}

fn get_button_id(world: &World, target: Entity) -> Option<String> {
    let transform_storage = world.read_storage::<UiTransform>();
    let o_button = transform_storage.get(target);
    o_button.map(|button| button.id.to_string())
}
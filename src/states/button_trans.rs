use amethyst::{
    prelude::*,
    ecs::prelude::Entity,
    ui::{UiCreator, UiEvent, UiEventType, UiButton, UiTransform}
};
use super::super::game_data::MiniRandoGameData;

pub trait ButtonTrans {
    fn button_click_trans<'a, 'b>(
        &self,
        world: &mut World,
        target: Entity
    ) -> Trans<MiniRandoGameData<'a, 'b>, StateEvent> {
        // TODO: See how this works with NLLs
        let button_id = {
            let transform_storage = world.read_storage::<UiTransform>();
            let o_button = transform_storage.get(target);
            o_button.map(|button| button.id.to_string())
        };

        if let Some(id) = button_id {
            self.get_trans_for_id(world, id.as_str())
        } else {
            Trans::None
        }
    }

    fn get_trans_for_id<'a, 'b>(&self, world: &mut World, button_id: &str) -> Trans<MiniRandoGameData<'a, 'b>, StateEvent>;
}
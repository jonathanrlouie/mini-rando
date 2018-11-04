use amethyst::{
    prelude::*,
    ecs::prelude::{Entity, System, Write},
    shrev::{EventChannel, ReaderId},
    ui::{UiCreator, UiEvent, UiEventType, UiButton, UiTransform}
};
use super::seeded::Seeded;

pub struct MainMenu;

impl MainMenu {
    fn button_click_trans<'a, 'b>(
        &self,
        world: &World,
        target: Entity,
        button_id: &str,
        trans: SimpleTrans<'a, 'b>
    ) -> SimpleTrans<'a, 'b> {
        let transform_storage = world.read_storage::<UiTransform>();
        if let Some(button) = transform_storage.get(target) {
            if button.id == button_id {
                trans
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

impl<'a, 'b> SimpleState<'a, 'b> for MainMenu {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;
        world.exec(|mut creator: UiCreator| {
            creator.create("main_menu_ui.ron", ());
        });
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans<'a, 'b> {
        let StateData { world, .. } = data;
        if let StateEvent::Ui(ev) = &event {
            match ev.event_type {
                UiEventType::ClickStop => {
                    self.button_click_trans(
                        &world,
                        ev.target,
                        "seeded_game_button",
                        Trans::Push(Box::new(Seeded))
                    )
                },
                _ => Trans::None
            }
        } else {
            Trans::None
        }
    }



    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans<'a, 'b> {
        Trans::None
    }
}
use amethyst::{
    prelude::*,
    ecs::prelude::{Entity, System, Write},
    shrev::{EventChannel, ReaderId},
    ui::{UiCreator, UiTransform, UiEvent, UiEventType}
};
use super::super::{
    main_menu::MainMenu,
    play::Play,
    button_trans::ButtonTrans
};
use super::super::super::game_data::{MiniRandoGameData, StateDispatcher};

pub struct CustomGame;

impl ButtonTrans for CustomGame {
    fn get_trans_for_id<'a, 'b>(&self, _world: &mut World, button_id: &str) -> Trans<MiniRandoGameData<'a, 'b>, StateEvent> {
        match button_id {
            "back_button" => Trans::Push(Box::new(MainMenu)),
            "start_game_button" => Trans::Switch(Box::new(Play)),
            _ => Trans::None
        }
    }
}

impl<'a, 'b> State<MiniRandoGameData<'a, 'b>, StateEvent> for CustomGame {
    fn on_start(&mut self, data: StateData<MiniRandoGameData>) {
        let StateData { world, .. } = data;
        world.exec(|mut creator: UiCreator| {
            creator.create("custom_game_ui.ron", ());
        });
    }

    fn handle_event(
        &mut self,
        data: StateData<MiniRandoGameData>,
        event: StateEvent
    ) -> Trans<MiniRandoGameData<'a, 'b>, StateEvent> {
        let StateData { mut world, .. } = data;
        if let StateEvent::Ui(ev) = &event {
            match ev.event_type {
                UiEventType::ClickStop => {
                    self.button_click_trans(&mut world, ev.target)
                },
                _ => Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<MiniRandoGameData>) -> Trans<MiniRandoGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, StateDispatcher::CustomGame);
        Trans::None
    }
}
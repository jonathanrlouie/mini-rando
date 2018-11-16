use amethyst::{
    prelude::*,
    ui::{UiCreator, UiEventType}
};
use super::{
    custom_game::custom_game::CustomGame,
    play::Play,
    button_trans::ButtonTrans
};
use super::super::game_data::{MiniRandoGameData, StateDispatcher};

pub struct MainMenu;

impl ButtonTrans for MainMenu {
    fn get_trans_for_id<'a, 'b>(&self, world: &mut World, button_id: &str) -> Trans<MiniRandoGameData<'a, 'b>, StateEvent> {
        match button_id {
            "custom_game_button" => {
                world.delete_all();
                Trans::Push(Box::new(CustomGame))
            },
            "start_game_button" => Trans::Switch(Box::new(Play)),
            _ => Trans::None
        }
    }
}

impl<'a, 'b> State<MiniRandoGameData<'a, 'b>, StateEvent> for MainMenu {
    fn on_start(&mut self, data: StateData<MiniRandoGameData>) {
        let StateData { world, .. } = data;
        world.exec(|mut creator: UiCreator| {
            creator.create("main_menu_ui.ron", ());
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
        data.data.update(&data.world, StateDispatcher::MainMenu);
        Trans::None
    }
}
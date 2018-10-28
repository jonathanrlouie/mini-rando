use amethyst::{
    prelude::*,
    ui::UiCreator
};

pub struct MainMenu;

impl<'a, 'b> SimpleState<'a, 'b> for MainMenu {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;
        world.exec(|mut creator: UiCreator| {
            creator.create("ui/example.ron", ());
        });
    }
}
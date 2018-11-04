use amethyst::{
    prelude::*,
    ecs::prelude::{Entity, System, Write},
    shrev::{EventChannel, ReaderId},
    ui::{UiCreator, UiEvent, UiEventType}
};

pub struct Seeded;

impl<'a, 'b> SimpleState<'a, 'b> for Seeded {
    fn on_start(&mut self, data: StateData<GameData>) {
        println!("hi");
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans<'a, 'b> {
        Trans::None
    }
}

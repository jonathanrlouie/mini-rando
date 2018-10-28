use amethyst::{
    prelude::*,
    ecs::prelude::{Entity, System, Write},
    shrev::{EventChannel, ReaderId},
    ui::{UiCreator, UiEvent}
};

pub struct MainMenu;

impl<'a, 'b> SimpleState<'a, 'b> for MainMenu {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;
        world.exec(|mut creator: UiCreator| {
            creator.create("main_menu_ui.ron", ());
        });
    }
}

pub struct UiEventHandlerSystem {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl UiEventHandlerSystem {
    pub fn new() -> Self {
        UiEventHandlerSystem { reader_id: None }
    }
}

impl<'a> System<'a> for UiEventHandlerSystem {
    type SystemData = Write<'a, EventChannel<UiEvent>>;

    fn run(&mut self, mut events: Self::SystemData) {
        if self.reader_id.is_none() {
            self.reader_id = Some(events.register_reader());
        }

        // Reader id was just initialized above if empty
        for ev in events.read(self.reader_id.as_mut().unwrap()) {
            println!("[SYSTEM] You just interacted with a ui element: {:?}", ev);
        }
    }
}
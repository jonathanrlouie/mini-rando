use amethyst::{
    ecs::prelude::{Write, System, ReadStorage},
    shrev::{ReaderId, EventChannel},
    ui::{UiEvent, UiTransform, UiEventType}
};

pub struct UiEventHandlerSystem {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl UiEventHandlerSystem {
    pub fn new() -> Self {
        UiEventHandlerSystem { reader_id: None }
    }
}

impl UiEventHandlerSystem {
    fn handle_event(&self, transform: &UiTransform, event: &UiEvent) {
        match transform.id.as_str() {
            "generate_seed_button" => {
                match event.event_type {
                    UiEventType::ClickStop => {
                        println!("clicked gen seed");
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }
}

impl<'a> System<'a> for UiEventHandlerSystem {
    type SystemData = (
        Write<'a, EventChannel<UiEvent>>,
        ReadStorage<'a, UiTransform>
    );

    fn run(&mut self, (mut events, transforms): Self::SystemData) {
        if self.reader_id.is_none() {
            self.reader_id = Some(events.register_reader())
        }

        for ev in events.read(self.reader_id.as_mut().unwrap()) {
            if let Some(transform) = transforms.get(ev.target) {
                self.handle_event(&transform, &ev)
            }
        }
    }
}
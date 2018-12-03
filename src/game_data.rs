use std::{
    marker::Send
};
use amethyst::{
    ecs::prelude::{System, Dispatcher, DispatcherBuilder, World},
    core::{ArcThreadPool, SystemBundle},
    DataInit,
    Result as AmethystResult,
    Error as AmethystError
};

pub enum StateDispatcher {
    MainMenu,
    CustomGame,
    Play
}

pub struct MiniRandoGameData<'a, 'b> {
    core_dispatcher: Dispatcher<'a, 'b>,
    main_menu_dispatcher: Dispatcher<'a, 'b>,
    custom_game_dispatcher: Dispatcher<'a, 'b>,
    play_dispatcher: Dispatcher<'a, 'b>
}

impl<'a, 'b> MiniRandoGameData<'a, 'b> {
    pub fn update(&mut self, world: &World, state_dispatcher: StateDispatcher) {
        match state_dispatcher {
            StateDispatcher::MainMenu => self.main_menu_dispatcher.dispatch(&world.res),
            StateDispatcher::CustomGame => self.custom_game_dispatcher.dispatch(&world.res),
            StateDispatcher::Play => self.play_dispatcher.dispatch(&world.res)
        }
        self.core_dispatcher.dispatch(&world.res);
    }
}

pub struct MiniRandoGameDataBuilder<'a, 'b> {
    pub core: DispatcherBuilder<'a, 'b>,
    pub main_menu: DispatcherBuilder<'a, 'b>,
    pub custom_game: DispatcherBuilder<'a, 'b>,
    pub play: DispatcherBuilder<'a, 'b>
}

impl<'a, 'b> Default for MiniRandoGameDataBuilder<'a, 'b> {
    fn default() -> Self {
        MiniRandoGameDataBuilder::new()
    }
}

impl<'a, 'b> MiniRandoGameDataBuilder<'a, 'b> {
    fn new() -> Self {
        MiniRandoGameDataBuilder {
            core: DispatcherBuilder::new(),
            main_menu: DispatcherBuilder::new(),
            custom_game: DispatcherBuilder::new(),
            play: DispatcherBuilder::new()
        }
    }

    pub fn with_base_bundle<B>(mut self, bundle: B) -> AmethystResult<Self>
    where
        B: SystemBundle<'a, 'b>
    {
        bundle
            .build(&mut self.core)
            .map_err(|err| AmethystError::Core(err))?;
        Ok(self)
    }

    pub fn with_main_menu<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a
    {
        self.main_menu.add(system, name, dependencies);
        self
    }

    pub fn with_custom_game<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a
    {
        self.custom_game.add(system, name, dependencies);
        self
    }

    pub fn with_play<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a
    {
        self.play.add(system, name, dependencies);
        self
    }
}

impl<'a, 'b> DataInit<MiniRandoGameData<'a, 'b>> for MiniRandoGameDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> MiniRandoGameData<'a, 'b> {
        let pool = world.read_resource::<ArcThreadPool>().clone();

        let mut core_dispatcher = self.core.with_pool(pool.clone()).build();
        let mut main_menu_dispatcher = self.main_menu.with_pool(pool.clone()).build();
        let mut custom_game_dispatcher = self.custom_game.with_pool(pool.clone()).build();
        let mut play_dispatcher = self.play.with_pool(pool.clone()).build();

        core_dispatcher.setup(&mut world.res);
        main_menu_dispatcher.setup(&mut world.res);
        custom_game_dispatcher.setup(&mut world.res);
        play_dispatcher.setup(&mut world.res);

        MiniRandoGameData {
            core_dispatcher,
            main_menu_dispatcher,
            custom_game_dispatcher,
            play_dispatcher
        }
    }
}
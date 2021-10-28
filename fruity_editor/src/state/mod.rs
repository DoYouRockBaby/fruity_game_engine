use crate::state::theme::update_theme;
use crate::state::theme::ThemeMessage;
use crate::state::theme::ThemeState;
use crate::state::world::update_world;
use crate::state::world::WorldMessage;
use crate::state::world::WorldState;
use crate::World;

pub mod theme;
pub mod world;

#[derive(Debug)]
pub struct State {
    pub theme: ThemeState,
    pub world: WorldState,
}

impl State {
    pub fn new(world: &World) -> Self {
        State {
            theme: ThemeState::default(),
            world: WorldState::new(world),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Empty,
    Theme(ThemeMessage),
    World(WorldMessage),
}

pub fn update_state(state: &mut State, message: Message) {
    match message {
        Message::Empty => (),
        Message::Theme(theme) => update_theme(&mut state.theme, theme),
        Message::World(theme) => update_world(&mut state.world, theme),
    }
}

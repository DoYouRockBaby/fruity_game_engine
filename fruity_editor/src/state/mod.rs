use crate::state::theme::update_theme;
use crate::state::world::update_world;
use crate::state::world::WorldMessage;
use crate::state::theme::ThemeMessage;
use crate::state::world::WorldState;
use crate::state::theme::ThemeState;

pub mod world;
pub mod theme;

#[derive(Debug, Default)]
pub struct State {
    pub theme: ThemeState,
    pub world: WorldState,
}

#[derive(Debug, Clone)]
pub enum Message {
    Theme(ThemeMessage),
    World(WorldMessage),
}

pub fn update_state(state: &mut State, message: Message) {
    match message {
        Message::Theme(theme) => update_theme(&mut state.theme, theme),
        Message::World(theme) => update_world(&mut state.world, theme),
    }
}
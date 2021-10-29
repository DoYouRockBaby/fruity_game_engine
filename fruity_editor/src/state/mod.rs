use crate::components::panes::PanesMessage;
use crate::state::entity::update_entity;
use crate::state::entity::EntityMessage;
use crate::state::entity::EntityState;
use crate::state::theme::update_theme;
use crate::state::theme::ThemeMessage;
use crate::state::theme::ThemeState;
use crate::state::world::update_world;
use crate::state::world::WorldMessage;
use crate::state::world::WorldState;
use crate::World;

pub mod entity;
pub mod theme;
pub mod world;

#[derive(Debug)]
pub struct State {
    pub theme: ThemeState,
    pub world: WorldState,
    pub entity: EntityState,
}

impl State {
    pub fn new(world: &World) -> Self {
        State {
            theme: ThemeState::default(),
            world: WorldState::new(world),
            entity: EntityState::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Empty,
    Theme(ThemeMessage),
    World(WorldMessage),
    Panes(PanesMessage),
    Entity(EntityMessage),
}

pub fn update_state(state: &mut State, message: Message) {
    match message {
        Message::Theme(theme) => update_theme(&mut state.theme, theme),
        Message::World(theme) => update_world(&mut state.world, theme),
        Message::Entity(entity) => update_entity(&mut state.entity, entity),
        _ => (),
    }
}

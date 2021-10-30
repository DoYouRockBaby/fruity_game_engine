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
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

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

#[derive(Clone)]
pub enum Message {
    Empty,
    Theme(ThemeMessage),
    World(WorldMessage),
    Panes(PanesMessage),
    Entity(EntityMessage),
    Callback(Arc<dyn Fn() + Send + Sync>),
    StringChanged(Arc<dyn Fn(&str) + Send + Sync>, String),
    BoolChanged(Arc<dyn Fn(bool) + Send + Sync>, bool),
    IntegerChanged(Arc<dyn Fn(i64) + Send + Sync>, i64),
    FloatChanged(Arc<dyn Fn(f64) + Send + Sync>, f64),
    AnyChanged(
        Arc<dyn Fn(&dyn Any) + Send + Sync>,
        Arc<dyn Any + Send + Sync>,
    ),
}

impl Debug for Message {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

pub fn update_state(state: &mut State, message: Message) {
    match message {
        Message::Theme(theme) => update_theme(&mut state.theme, theme),
        Message::World(theme) => update_world(&mut state.world, theme),
        Message::Entity(entity) => update_entity(&mut state.entity, entity),
        Message::Callback(callback) => callback(),
        Message::StringChanged(callback, value) => callback(&value),
        Message::BoolChanged(callback, value) => callback(value),
        Message::IntegerChanged(callback, value) => callback(value),
        Message::FloatChanged(callback, value) => callback(value),
        Message::AnyChanged(callback, value) => callback(&value),
        _ => (),
    }
}

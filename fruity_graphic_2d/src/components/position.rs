use fruity_any::*;
use fruity_core::*;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
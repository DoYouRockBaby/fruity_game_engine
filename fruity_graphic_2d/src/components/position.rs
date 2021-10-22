use fruity_any::*;
use fruity_core::*;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

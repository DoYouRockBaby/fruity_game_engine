use fruity_any::*;
use fruity_core::*;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }
}

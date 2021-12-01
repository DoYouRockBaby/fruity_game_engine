use fruity_any::*;
use fruity_ecs::*;
use fruity_graphic::math::material::Material;

#[derive(Debug, Clone, Default, Component, FruityAny)]
pub struct Sprite {
    pub material: Material,
    pub z_index: usize,
}

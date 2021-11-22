use fruity_any::*;
use fruity_ecs::*;
use fruity_graphic::math::matrix3::Matrix3;

#[derive(Debug, Clone, Component, Default, FruityAny)]
pub struct Transform2d {
    pub transform: Matrix3,
}

use fruity_any::*;
use fruity_ecs::*;

#[derive(Debug, Default, Clone, Component, FruityAny)]
pub struct KinematicRigidBody {
    pub handle: Option<(u32, u32)>,
}

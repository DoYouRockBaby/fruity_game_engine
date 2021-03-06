use fruity_any::*;
use fruity_ecs::*;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct DynamicRigidBody {
    pub handle: Option<(u32, u32)>,
    pub gravity_scale: f32,
    pub can_sleep: bool,
    pub ccd_enabled: bool,
}

impl Default for DynamicRigidBody {
    fn default() -> Self {
        Self {
            handle: None,
            gravity_scale: 1.0,
            can_sleep: true,
            ccd_enabled: false,
        }
    }
}

use fruity_any::*;
use fruity_ecs::resource::resource::Resource;
use fruity_ecs::*;
use fruity_introspect::*;
use std::sync::Arc;

#[derive(Debug, Clone, Component, IntrospectFields, FruityAny)]
pub struct Size {
    pub texture: Arc<dyn Resource>,
}

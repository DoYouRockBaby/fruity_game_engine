use fruity_any::*;
use fruity_core::*;
use fruity_graphic::resources::material_resource::MaterialResource;
use std::sync::Arc;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct Sprite {
    pub material: Option<Arc<MaterialResource>>,
}

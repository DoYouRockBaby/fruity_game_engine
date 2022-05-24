use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use rapier2d::prelude::ColliderSet;
use rapier2d::prelude::RigidBodySet;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(FruityAny)]
pub struct Rapier2dService {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
}

impl Debug for Rapier2dService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl Rapier2dService {
    pub fn new(_resource_container: Arc<ResourceContainer>) -> Self {
        Self {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
        }
    }
}

impl IntrospectObject for Rapier2dService {
    fn get_class_name(&self) -> String {
        "Rapier2dService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for Rapier2dService {}

use crate::component::component_reference::ComponentReference;
use crate::entity::archetype::ArchetypeArcRwLock;
use crate::entity::entity_guard::EntityReadGuard;
use crate::entity::entity_guard::EntityWriteGuard;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::utils::introspect::cast_introspect_ref;
use fruity_core::RwLockReadGuard;
use fruity_core::RwLockWriteGuard;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

/// A reference over an entity stored into an Archetype
#[derive(Clone, FruityAny)]
pub struct EntityReference {
    pub(crate) entity_id: usize,
    pub(crate) archetype: ArchetypeArcRwLock,
}

impl EntityReference {
    /// Get a read access to the entity
    pub fn read(&self) -> EntityReadGuard {
        let archetype_reader = self.archetype.read();
        let guard = archetype_reader
            .lock_array
            .get(self.entity_id)
            .unwrap()
            .read();

        // TODO: Find a way to remove it
        let guard =
            unsafe { std::mem::transmute::<RwLockReadGuard<()>, RwLockReadGuard<()>>(guard) };

        EntityReadGuard {
            entity_id: self.entity_id,
            _guard: Rc::new(guard),
            archetype_reader: Rc::new(archetype_reader),
        }
    }

    /// Get a write access to the entity
    pub fn write(&self) -> EntityWriteGuard {
        let archetype_reader = self.archetype.read();
        let guard = archetype_reader
            .lock_array
            .get(self.entity_id)
            .unwrap()
            .write();

        // TODO: Find a way to remove it
        let guard =
            unsafe { std::mem::transmute::<RwLockWriteGuard<()>, RwLockWriteGuard<()>>(guard) };

        EntityWriteGuard {
            entity_id: self.entity_id,
            _guard: Rc::new(guard),
            archetype_reader: Rc::new(archetype_reader),
        }
    }

    /// Get all components
    pub fn get_components(&self) -> Vec<ComponentReference> {
        self.archetype.clone().get_entity_components(self.entity_id)
    }

    /// Get components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn get_components_by_type_identifier(
        &self,
        component_identifier: &str,
    ) -> Vec<ComponentReference> {
        self.archetype
            .clone()
            .get_entity_components_from_type(self.entity_id, component_identifier)
    }
}

impl Debug for EntityReference {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl IntrospectObject for EntityReference {
    fn get_class_name(&self) -> String {
        "EntityReference".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "get_entity_id".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<EntityReference>(this);
                    let this = this.read();
                    let result = this.get_entity_id();

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "get_name".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<EntityReference>(this);
                    let this = this.read();
                    let result = this.get_name();

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "is_enabled".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<EntityReference>(this);
                    let this = this.read();
                    let result = this.is_enabled();

                    Ok(Some(result.fruity_into()))
                })),
            },
            // TODO: Complete that
        ]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl SerializableObject for EntityReference {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl FruityInto<Serialized> for EntityReference {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}

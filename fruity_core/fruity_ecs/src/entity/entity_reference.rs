use crate::component::component_reference::ComponentReference;
use crate::entity::archetype::InnerArchetype;
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
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// A reference over an entity stored into an Archetype
#[derive(Clone, FruityAny)]
pub struct EntityReference {
    pub(crate) index: usize,
    pub(crate) inner_archetype: Arc<InnerArchetype>,
}

impl EntityReference {
    /// Get a read access to the entity
    pub fn read(&self) -> EntityReadGuard {
        let lock_array = self.inner_archetype.lock_array.read().unwrap();
        let guard = lock_array.get(self.index).unwrap().read().unwrap();

        // TODO: Find a way to remove it
        let guard =
            unsafe { std::mem::transmute::<RwLockReadGuard<()>, RwLockReadGuard<()>>(guard) };

        EntityReadGuard {
            _guard: guard,
            index: self.index,
            inner_archetype: self.inner_archetype.clone(),
        }
    }

    /// Get a write access to the entity
    pub fn write(&self) -> EntityWriteGuard {
        let lock_array = self.inner_archetype.lock_array.read().unwrap();
        let guard = lock_array.get(self.index).unwrap().write().unwrap();

        // TODO: Find a way to remove it
        let guard =
            unsafe { std::mem::transmute::<RwLockWriteGuard<()>, RwLockWriteGuard<()>>(guard) };

        EntityWriteGuard {
            _guard: guard,
            index: self.index,
            inner_archetype: self.inner_archetype.clone(),
        }
    }

    /// Iter over all components
    pub fn iter_all_component(&self) -> impl Iterator<Item = ComponentReference> + '_ {
        self.inner_archetype
            .component_arrays
            .iter()
            .map(|(key, component_array)| {
                let component_count = {
                    let component_array = component_array.read().unwrap();
                    component_array.get_component_count()
                };

                (0..component_count)
                    .into_iter()
                    .map(|index| ComponentReference {
                        entity: self.clone(),
                        component_identifier: key.clone(),
                        component_index: index,
                    })
            })
            .flatten()
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

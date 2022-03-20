use crate::component::component::Component;
use crate::entity::archetype::InnerArchetype;
use crate::entity::entity_guard::EntityReadGuard;
use crate::entity::entity_guard::EntityWriteGuard;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
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

/// A reference over an entity stored into an Archetype
#[derive(Clone, FruityAny)]
pub struct ComponentReference {
    pub(crate) entity: EntityReference,
    pub(crate) component_identifier: String,
    pub(crate) component_index: usize,
}

impl ComponentReference {
    /// Get a read access to the entity
    pub fn read(&self) -> &dyn Component {
        let entity = self.entity.read();
        let component = entity
            .read_components(&self.component_identifier)
            .remove(self.component_index);

        // TODO: Find a way to remove it
        let component = unsafe { std::mem::transmute::<&dyn Component, &dyn Component>(component) };

        component
    }

    /// Get a write access to the entity
    pub fn write(&self) -> &mut dyn Component {
        let entity = self.entity.write();
        let component = entity
            .write_components(&self.component_identifier)
            .remove(self.component_index);

        // TODO: Find a way to remove it
        let component =
            unsafe { std::mem::transmute::<&mut dyn Component, &mut dyn Component>(component) };

        component
    }
}

impl Debug for ComponentReference {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl IntrospectObject for ComponentReference {
    fn get_class_name(&self) -> String {
        let component = self.read();
        component.get_class_name()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        let component = self.read();
        component
            .get_method_infos()
            .into_iter()
            .map(|method_info| MethodInfo {
                name: method_info.name,
                call: match method_info.call {
                    MethodCaller::Const(call) => {
                        MethodCaller::Const(Arc::new(move |this, args| {
                            let this = this.downcast_ref::<ComponentReference>().unwrap();
                            let component = this.read();
                            call(component.as_any_ref(), args)
                        }))
                    }
                    MethodCaller::Mut(call) => MethodCaller::Mut(Arc::new(move |this, args| {
                        let this = this.downcast_mut::<ComponentReference>().unwrap();
                        let component = this.write();
                        call(component.as_any_mut(), args)
                    })),
                },
            })
            .collect::<Vec<_>>()
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        let component = self.read();
        component
            .get_field_infos()
            .into_iter()
            .map(|field_info| FieldInfo {
                name: field_info.name,
                serializable: field_info.serializable,
                getter: Arc::new(move |this| {
                    let this = this.downcast_ref::<ComponentReference>().unwrap();
                    let component = this.read();
                    (field_info.getter)(component.as_any_ref())
                }),
                setter: match field_info.setter {
                    SetterCaller::Const(call) => {
                        SetterCaller::Const(Arc::new(move |this, args| {
                            let this = this.downcast_ref::<ComponentReference>().unwrap();
                            let component = this.read();
                            call(component.as_any_ref(), args)
                        }))
                    }
                    SetterCaller::Mut(call) => SetterCaller::Mut(Arc::new(move |this, args| {
                        let this = this.downcast_mut::<ComponentReference>().unwrap();
                        let component = this.write();
                        call(component.as_any_mut(), args)
                    })),
                    SetterCaller::None => SetterCaller::None,
                },
            })
            .collect::<Vec<_>>()
    }
}

impl SerializableObject for ComponentReference {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

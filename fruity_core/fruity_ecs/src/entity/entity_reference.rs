use crate::component::component::Component;
use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;
use crate::component::component_guard::TypedComponentReadGuard;
use crate::component::component_guard::TypedComponentWriteGuard;
use crate::component::component_reference::ComponentReference;
use crate::entity::archetype::InnerArchetype;
use crate::entity::entity::EntityId;
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

/// A reference over an entity stored into an Archetype
#[derive(Clone, FruityAny)]
pub struct EntityReference {
    pub(crate) index: usize,
    pub(crate) inner_archetype: Arc<InnerArchetype>,
}

impl EntityReference {
    /// Get the entity id
    pub fn get_entity_id(&self) -> EntityId {
        let entity_id_array = self.inner_archetype.entity_id_array.read().unwrap();
        *entity_id_array.get(self.index).unwrap()
    }

    /// Get the entity name
    pub fn get_name(&self) -> String {
        let name_array = self.inner_archetype.name_array.read().unwrap();
        name_array.get(self.index).unwrap().clone()
    }

    /// Set the entity name
    ///
    /// # Arguments
    /// * `value` - The name value
    ///
    pub fn set_name(&self, value: &str) {
        let mut name_array = self.inner_archetype.name_array.write().unwrap();
        let name = name_array.get_mut(self.index).unwrap();
        *name = value.to_string();
    }

    /// Is the entity enabled
    pub fn is_enabled(&self) -> bool {
        let enabled_array = self.inner_archetype.enabled_array.read().unwrap();
        *enabled_array.get(self.index).unwrap()
    }

    /// Set the entity enabled state
    ///
    /// # Arguments
    /// * `value` - Is the entity enabled
    ///
    pub fn set_enabled(&self, value: bool) {
        let mut enabled_array = self.inner_archetype.enabled_array.write().unwrap();
        let enabled = enabled_array.get_mut(self.index).unwrap();
        *enabled = value;
    }

    /// Get a specific component
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn get_component(&self, component_identifier: &str) -> Option<ComponentReference> {
        self.inner_archetype
            .component_arrays
            .get(component_identifier)
            .map(|components_array| {
                let components_array = components_array.read().unwrap();
                components_array.get(&self.index)
            })
    }

    /// Read a specific component
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn read_component<T: Component>(
        &self,
        component_identifier: &str,
    ) -> Option<TypedComponentReadGuard<'_, T>> {
        let component = self.get_component(component_identifier)?;

        // TODO: Find a way to remove it
        let reader = unsafe {
            std::mem::transmute::<ComponentReadGuard, ComponentReadGuard>(component.read())
        };

        reader.downcast::<T>()
    }

    /// Write a specific component
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn write_component<T: Component>(
        &self,
        component_identifier: &str,
    ) -> Option<TypedComponentWriteGuard<'_, T>> {
        let component = self.get_component(component_identifier)?;

        // TODO: Find a way to remove it
        let writer = unsafe {
            std::mem::transmute::<ComponentWriteGuard, ComponentWriteGuard>(component.write())
        };

        writer.downcast::<T>()
    }

    /// Iter over all components
    pub fn iter_all_components(&self) -> impl Iterator<Item = ComponentReference> + '_ {
        self.inner_archetype
            .component_arrays
            .iter()
            .map(|(_, components_array)| {
                let components_array = components_array.read().unwrap();
                components_array.get(&self.index)
            })
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
                    let result = this.get_entity_id();

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "get_name".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<EntityReference>(this);
                    let result = this.get_name();

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "is_enabled".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<EntityReference>(this);
                    let result = this.is_enabled();

                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "iter_all_components".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<EntityReference>(this);

                    let result = Serialized::Array(
                        this.iter_all_components()
                            .map(|component| Serialized::NativeObject(Box::new(component)))
                            .collect::<Vec<_>>(),
                    );

                    Ok(Some(result.fruity_into()))
                })),
            },
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

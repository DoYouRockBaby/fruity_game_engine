use crate::component::component::Component;
use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;
use crate::component::component_guard::TypedComponentReadGuard;
use crate::component::component_guard::TypedComponentWriteGuard;
use crate::entity::entity_guard::EntityReadGuard;
use crate::entity::entity_guard::EntityWriteGuard;
use crate::entity::entity_reference::EntityReference;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::serialize::serialized::SerializableObject;
use std::any::TypeId;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

/// A reference over an entity stored into an Archetype
#[derive(Clone, FruityAny)]
pub struct ComponentReference {
    pub(crate) entity: EntityReference,
    pub(crate) component_identifier: String,
    pub(crate) component_index: usize,
}

impl ComponentReference {
    /// Get a read access to the component
    pub fn read(&self) -> ComponentReadGuard<'_> {
        ComponentReadGuard {
            entity_reader: self.entity.read(),
            component_identifier: self.component_identifier.clone(),
            component_index: self.component_index,
        }
    }

    /// Get a write access to the component
    pub fn write(&self) -> ComponentWriteGuard<'_> {
        ComponentWriteGuard {
            entity_writer: self.entity.write(),
            component_identifier: self.component_identifier.clone(),
            component_index: self.component_index,
        }
    }

    /// Get a read access to the component
    pub fn read_typed<T: Component>(&self) -> Option<TypedComponentReadGuard<'_, T>> {
        let component_type_id = {
            let entity_reader = self.entity.read();

            let component = entity_reader
                .read_components(&self.component_identifier)
                .remove(self.component_index);

            component.as_any_ref().type_id()
        };

        if component_type_id == TypeId::of::<T>() {
            Some(TypedComponentReadGuard {
                entity_reader: self.entity.read(),
                component_identifier: self.component_identifier.clone(),
                component_index: self.component_index,
                phantom: PhantomData {},
            })
        } else {
            None
        }
    }

    /// Get a write access to the component
    pub fn write_typed<T: Component>(&self) -> Option<TypedComponentWriteGuard<'_, T>> {
        let component_type_id = {
            let entity_reader = self.entity.read();

            let component = entity_reader
                .read_components(&self.component_identifier)
                .remove(self.component_index);

            component.as_any_ref().type_id()
        };

        if component_type_id == TypeId::of::<T>() {
            Some(TypedComponentWriteGuard {
                entity_writer: self.entity.write(),
                component_identifier: self.component_identifier.clone(),
                component_index: self.component_index,
                phantom: PhantomData {},
            })
        } else {
            None
        }
    }

    /// Get a read access to the entity
    pub fn read_entity(&self) -> EntityReadGuard<'_> {
        self.entity.read()
    }

    /// Get a write access to the entity
    pub fn write_entity(&self) -> EntityWriteGuard<'_> {
        self.entity.write()
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
                        let mut component = this.write();
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
                        let mut component = this.write();
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

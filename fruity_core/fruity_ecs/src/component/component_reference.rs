use crate::component::component::Component;
use crate::component::component::StaticComponent;
use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;
use crate::component::component_guard::InternalReadGuard;
use crate::component::component_guard::TypedComponentReadGuard;
use crate::component::component_guard::TypedComponentWriteGuard;
use crate::entity::entity_guard::EntityReadGuard;
use crate::entity::entity_guard::EntityWriteGuard;
use crate::entity::entity_reference::EntityReference;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use std::any::TypeId;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

/// A reference over an entity stored into an Archetype
#[derive(Clone, FruityAny)]
pub struct ComponentReference {
    pub(crate) entity_reference: EntityReference,
    pub(crate) component_identifier: String,
    pub(crate) component_index: usize,
}

impl ComponentReference {
    /// Get a read access to the component
    pub fn read(&self) -> ComponentReadGuard<'_> {
        ComponentReadGuard {
            _guard: InternalReadGuard::Read(self.entity_reference.read()._guard.clone()),
            archetype_reader: Rc::new(self.entity_reference.archetype.read()),
            component_identifier: self.component_identifier.clone(),
            component_index: self.component_index,
        }
    }

    /// Get a write access to the component
    pub fn write(&self) -> ComponentWriteGuard<'_> {
        ComponentWriteGuard {
            _guard: self.entity_reference.write()._guard.clone(),
            archetype_reader: Rc::new(self.entity_reference.archetype.read()),
            component_identifier: self.component_identifier.clone(),
            component_index: self.component_index,
        }
    }

    /// Get a read access to the component
    pub fn read_typed<T: Component + StaticComponent>(
        &self,
    ) -> Option<TypedComponentReadGuard<'_, T>> {
        let component_reader = self.read();
        let component_type_id = component_reader.as_any_ref().type_id();

        if component_type_id == TypeId::of::<T>() {
            Some(TypedComponentReadGuard {
                component_reader,
                phantom: PhantomData {},
            })
        } else {
            None
        }
    }

    /// Get a write access to the component
    pub fn write_typed<T: Component + StaticComponent>(
        &self,
    ) -> Option<TypedComponentWriteGuard<'_, T>> {
        let component_writer = self.write();
        let component_type_id = component_writer.as_any_ref().type_id();

        if component_type_id == TypeId::of::<T>() {
            Some(TypedComponentWriteGuard {
                component_writer,
                phantom: PhantomData {},
            })
        } else {
            None
        }
    }

    /// Get a read access to the entity
    pub fn read_entity(&self) -> EntityReadGuard<'_> {
        self.entity_reference.read()
    }

    /// Get a write access to the entity
    pub fn write_entity(&self) -> EntityWriteGuard<'_> {
        self.entity_reference.write()
    }
}

impl Debug for ComponentReference {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl FruityInto<Serialized> for ComponentReference {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
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

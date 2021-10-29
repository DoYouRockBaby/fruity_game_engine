use crate::component::component_list_rwlock::ComponentListRwLock;
use crate::component::component_rwlock::ComponentRwLock;
use crate::entity::archetype::encode_entity::decode_components;
use crate::entity::archetype::encode_entity::decode_components_mut;
use crate::entity::archetype::encode_entity::decode_entity_head;
use crate::entity::archetype::encode_entity::decode_entity_head_mut;
use crate::entity::archetype::get_type_identifier;
use crate::entity::archetype::inner_archetype::InnerArchetype;
use crate::entity::archetype::Component;
use crate::entity::archetype::EntityCellHead;
use crate::entity::archetype::EntityTypeIdentifier;
use crate::service::utils::cast_service;
use crate::service::utils::ArgumentCaster;
use fruity_any::*;
use fruity_introspect::serializable_object::SerializableObject;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use fruity_introspect::SetterCaller;
use itertools::Itertools;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// A weak over an entity RwLock, this is the handle that will be used by the extern user to access datas
/// This can be clone and works like an Arc but over a reference that it don't own and have access to the
/// reference RwLock functionalities
#[derive(FruityAny, Clone)]
pub struct EntitySharedRwLock {
    inner_archetype: Arc<RwLock<InnerArchetype>>,
    buffer_index: usize,
}

impl EntitySharedRwLock {
    /// Returns an EntitySharedRwLock
    pub(crate) fn new(
        inner_archetype: Arc<RwLock<InnerArchetype>>,
        buffer_index: usize,
    ) -> EntitySharedRwLock {
        EntitySharedRwLock {
            inner_archetype,
            buffer_index,
        }
    }

    /// Create a read guard over the entity RwLock
    pub fn read(&self) -> EntityReadGuard {
        EntityReadGuard::new(&self.inner_archetype, self.buffer_index)
    }

    /// Create a write guard over the entity RwLock
    pub fn write(&self) -> EntityWriteGuard {
        EntityWriteGuard::new(&self.inner_archetype, self.buffer_index)
    }

    /// Get a component rwlock
    ///
    /// # Arguments
    /// * `component_type` - The component type
    ///
    pub fn get_component(&self, component_type: String) -> Option<ComponentRwLock> {
        let reader = self.read();
        reader
            .iter()
            .enumerate()
            .find(|(_index, component)| component.get_component_type() == component_type)
            .map(|(index, _component)| ComponentRwLock::new(self.clone(), index))
    }

    /// Check if the entity contains the given component types
    ///
    /// # Arguments
    /// * `component_types` - The component types
    ///
    pub fn contains(&self, component_types: &EntityTypeIdentifier) -> bool {
        let reader = self.read();
        let entity_type_identifier = get_type_identifier(&reader);
        entity_type_identifier.contains(component_types)
    }

    /// Get components count
    pub fn len(&self) -> usize {
        let reader = self.read();
        reader.len()
    }

    /// Get collections of components list reader
    /// Cause an entity can contain multiple component of the same type, can returns multiple readers
    /// All components are mapped to the provided component identifiers in the same order
    ///
    /// # Arguments
    /// * `type_identifiers` - The identifier list of the components, components will be returned with the same order
    ///
    pub fn iter_components(
        &self,
        target_identifier: &EntityTypeIdentifier,
    ) -> impl Iterator<Item = ComponentListRwLock> {
        let inner_archetype = self.inner_archetype.read().unwrap();
        let intern_identifier = inner_archetype.get_type_identifier().clone();
        std::mem::drop(inner_archetype);

        // Get a collection of indexes, this contains the component indexes ordered
        // in the same order of the given identifier
        let component_indexes = target_identifier
            .clone()
            .0
            .into_iter()
            .map(|type_identifier| {
                intern_identifier
                    .0
                    .iter()
                    .enumerate()
                    .filter_map(|(index, component_type)| {
                        if *component_type == type_identifier {
                            Some(index)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .multi_cartesian_product()
            .map(|vec| Vec::from(vec));

        let weak = self.clone();
        component_indexes.map(move |component_indexes| {
            ComponentListRwLock::new(weak.clone(), component_indexes.clone())
        })
    }
}

impl Deref for EntitySharedRwLock {
    type Target = EntityCellHead;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        let archetype_reader = self.inner_archetype.read().unwrap();

        // TODO: Try a way to remove that (ignore the fact that archetype reader is local)
        let archetype_ref = unsafe { &*(&archetype_reader as *const _) } as &InnerArchetype;

        decode_entity_head(archetype_ref, self.buffer_index)
    }
}

impl DerefMut for EntitySharedRwLock {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        let mut archetype_writer = self.inner_archetype.write().unwrap();

        // TODO: Try a way to remove that (ignore the fact that archetype reader is local)
        let archetype_mut =
            unsafe { &mut *(&mut archetype_writer as *mut _) } as &mut InnerArchetype;

        decode_entity_head_mut(archetype_mut, self.buffer_index)
    }
}

impl Debug for EntitySharedRwLock {
    fn fmt(&self, formater: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let reader = self.read();
        reader.deref().fmt(formater)
    }
}

/// An entity guard that can be used to access an entity without mutability
pub struct EntityReadGuard<'a> {
    components: Vec<&'a dyn Component>,
    _archetype_reader: RwLockReadGuard<'a, InnerArchetype>,
    _entity_reader: RwLockReadGuard<'a, ()>,
}

impl<'a> EntityReadGuard<'a> {
    pub(crate) fn new(
        inner_archetype: &'a Arc<RwLock<InnerArchetype>>,
        buffer_index: usize,
    ) -> EntityReadGuard<'a> {
        let archetype_reader = inner_archetype.read().unwrap();

        // TODO: Try a way to remove that (ignore the fact that archetype reader is local)
        let archetype_ref = unsafe { &*(&archetype_reader as *const _) } as &InnerArchetype;

        let entity_head = decode_entity_head(archetype_ref, buffer_index);
        let components = decode_components(archetype_ref, entity_head);

        EntityReadGuard {
            components,
            _archetype_reader: archetype_reader,
            _entity_reader: entity_head.lock.read().unwrap(),
        }
    }
}

impl<'a> Deref for EntityReadGuard<'a> {
    type Target = [&'a dyn Component];

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.components
    }
}

impl<'a> Debug for EntityReadGuard<'a> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.deref().fmt(formatter)
    }
}

/// An entity guard that can be used to access an entity with mutability
pub struct EntityWriteGuard<'a> {
    entity_head: &'a EntityCellHead,
    components: Vec<&'a mut dyn Component>,
    _archetype_reader: RwLockReadGuard<'a, InnerArchetype>,
    entity_writer: Option<RwLockWriteGuard<'a, ()>>,
}

impl<'a> EntityWriteGuard<'a> {
    pub(crate) fn new(
        inner_archetype: &'a Arc<RwLock<InnerArchetype>>,
        buffer_index: usize,
    ) -> EntityWriteGuard<'a> {
        let archetype_reader = inner_archetype.read().unwrap();

        // TODO: Try a way to remove that (ignore the fact that archetype reader is local)
        let archetype_ref = unsafe { &*(&archetype_reader as *const _) } as &InnerArchetype;

        let entity_head = decode_entity_head(archetype_ref, buffer_index);
        let components = decode_components_mut(archetype_ref, entity_head);

        EntityWriteGuard {
            entity_head,
            components,
            _archetype_reader: archetype_reader,
            entity_writer: Some(entity_head.lock.write().unwrap()),
        }
    }
}

impl<'a> Drop for EntityWriteGuard<'a> {
    fn drop(&mut self) {
        std::mem::drop(self.entity_writer.take());
        self.entity_head.on_updated.notify(());
    }
}

impl<'a> Deref for EntityWriteGuard<'a> {
    type Target = [&'a mut dyn Component];

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.components
    }
}

impl<'s> DerefMut for EntityWriteGuard<'s> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.components
    }
}

impl<'a> Debug for EntityWriteGuard<'a> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.deref().fmt(formatter)
    }
}

impl IntrospectObject for EntitySharedRwLock {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "get_component".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = cast_service::<EntitySharedRwLock>(this);

                    let mut caster = ArgumentCaster::new("get_component", args);
                    let arg1 = caster.cast_next::<String>()?;

                    let result = this.get_component(arg1);

                    Ok(result.map(|result| Serialized::NativeObject(Box::new(result))))
                })),
            },
            MethodInfo {
                name: "contains".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = cast_service::<EntitySharedRwLock>(this);

                    let mut caster = ArgumentCaster::new("contains", args);
                    let arg1 = caster.cast_next::<Vec<String>>()?;

                    let result = this.contains(&EntityTypeIdentifier(arg1));

                    Ok(Some(Serialized::Bool(result)))
                })),
            },
            MethodInfo {
                name: "len".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, _args| {
                    let this = cast_service::<EntitySharedRwLock>(this);
                    let result = this.len();

                    Ok(Some(Serialized::USize(result)))
                })),
            },
        ]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![
            FieldInfo {
                name: "id".to_string(),
                getter: Arc::new(|this| {
                    let this = cast_service::<EntitySharedRwLock>(this);
                    this.entity_id.into()
                }),
                setter: SetterCaller::None,
            },
            FieldInfo {
                name: "on_updated".to_string(),
                getter: Arc::new(|this| {
                    let this = cast_service::<EntitySharedRwLock>(this);
                    this.on_updated.clone().into()
                }),
                setter: SetterCaller::None,
            },
        ]
    }
}

impl SerializableObject for EntitySharedRwLock {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl Into<Serialized> for EntitySharedRwLock {
    fn into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self.clone()))
    }
}

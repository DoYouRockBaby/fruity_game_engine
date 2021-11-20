use crate::component::component_list_rwlock::ComponentListRwLock;
use crate::component::component_rwlock::ComponentRwLock;
use crate::entity::archetype::encode_entity::decode_components;
use crate::entity::archetype::encode_entity::decode_components_mut;
use crate::entity::archetype::encode_entity::decode_entity_head;
use crate::entity::archetype::encode_entity::decode_entity_head_mut;
use crate::entity::archetype::get_type_identifier;
use crate::entity::archetype::Component;
use crate::entity::archetype::EntityCellHead;
use crate::entity::archetype::EntityTypeIdentifier;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::signal::Signal;
use fruity_core::utils::introspect::cast_introspect_ref;
use fruity_core::utils::introspect::ArgumentCaster;
use itertools::Itertools;
use std::any::TypeId;
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
    buffer: Arc<RwLock<Vec<u8>>>,
    buffer_index: usize,
    identifier: EntityTypeIdentifier,
    components_per_entity: usize,
    entity_size: usize,
}

impl EntitySharedRwLock {
    /// Returns an EntitySharedRwLock
    pub(crate) fn new(
        buffer: Arc<RwLock<Vec<u8>>>,
        buffer_index: usize,
        identifier: EntityTypeIdentifier,
        components_per_entity: usize,
        entity_size: usize,
    ) -> EntitySharedRwLock {
        EntitySharedRwLock {
            buffer,
            buffer_index,
            identifier,
            components_per_entity,
            entity_size,
        }
    }

    /// Create a read guard over the entity RwLock
    pub fn read(&self) -> EntityReadGuard {
        let buffer_reader = self.buffer.read().unwrap();

        // TODO: Try a way to remove that (ignore the fact that buffer reader is local)
        let buffer = unsafe { &*(&buffer_reader as *const _) } as &[u8];

        EntityReadGuard::new(
            buffer,
            self.buffer_index,
            self.components_per_entity,
            self.entity_size,
        )
    }

    /// Create a write guard over the entity RwLock
    pub fn write(&self) -> EntityWriteGuard {
        let buffer_reader = self.buffer.read().unwrap();

        // TODO: Try a way to remove that (ignore the fact that buffer reader is local)
        let buffer = unsafe { &*(&buffer_reader as *const _) } as &[u8];

        EntityWriteGuard::new(
            buffer,
            self.buffer_index,
            self.components_per_entity,
            self.entity_size,
        )
    }

    /// Get a component rwlock
    ///
    /// # Arguments
    /// * `component_type` - The component type
    ///
    pub fn get_component(&self, component_type: &str) -> Option<ComponentRwLock> {
        let reader = self.read();
        reader
            .get_components()
            .iter()
            .enumerate()
            .find(|(_index, component)| component.get_class_name() == component_type)
            .map(|(index, _component)| ComponentRwLock::new(self.clone(), index))
    }

    /// Check if the entity contains the given component types
    ///
    /// # Arguments
    /// * `component_types` - The component types
    ///
    pub fn contains(&self, component_types: &EntityTypeIdentifier) -> bool {
        let reader = self.read();
        let entity_type_identifier = get_type_identifier(&reader.get_components());
        entity_type_identifier.contains(component_types)
    }

    /// Get components count
    pub fn len(&self) -> usize {
        let reader = self.read();
        reader.get_components().len()
    }

    /// Get an iterator over all components list rwlock
    pub fn iter_all_components(&self) -> impl Iterator<Item = ComponentRwLock> {
        let this = self.clone();
        (0..self.len()).map(move |index| ComponentRwLock::new(this.clone(), index))
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
        // Get a collection of indexes, this contains the component indexes ordered
        // in the same order of the given identifier
        let component_indexes = target_identifier
            .clone()
            .0
            .into_iter()
            .map(|type_identifier| {
                self.identifier
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

impl Debug for EntitySharedRwLock {
    fn fmt(&self, formater: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let reader = self.read();
        reader.get_components().fmt(formater)
    }
}

/// An entity guard that can be used to access an entity without mutability
pub struct EntityReadGuard<'a> {
    entity_head: &'a EntityCellHead,
    components: Vec<&'a dyn Component>,
    _entity_reader: RwLockReadGuard<'a, ()>,
}

impl<'a> EntityReadGuard<'a> {
    pub(crate) fn new(
        buffer: &'a [u8],
        buffer_index: usize,
        components_per_entity: usize,
        entity_size: usize,
    ) -> EntityReadGuard<'a> {
        // TODO: Try a way to remove that (ignore the fact that archetype reader is local)
        let buffer = unsafe { &*(&buffer as *const _) } as &[u8];

        let entity_head = decode_entity_head(buffer, buffer_index);
        let components = decode_components(entity_head, components_per_entity, entity_size);

        EntityReadGuard {
            entity_head,
            components,
            _entity_reader: entity_head.lock.read().unwrap(),
        }
    }

    /// Get the list of the components in the entity
    pub fn get_components(&self) -> &[&'a dyn Component] {
        &self.components
    }

    /// Get a component rwlock
    ///
    /// # Arguments
    /// * `component_type` - The component type
    ///
    pub fn get_component<T: Component>(&self, component_type: &str) -> Option<&T> {
        match self
            .get_components()
            .iter()
            .find(|component| component.get_class_name() == component_type)
        {
            Some(component) => match component.as_any_ref().downcast_ref::<T>() {
                Some(component) => Some(component),
                None => None,
            },
            None => None,
        }
    }
}

impl<'a> Deref for EntityReadGuard<'a> {
    type Target = EntityCellHead;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.entity_head
    }
}

impl<'a> Debug for EntityReadGuard<'a> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.get_components().fmt(formatter)
    }
}

/// An entity guard that can be used to access an entity with mutability
pub struct EntityWriteGuard<'a> {
    entity_head: &'a mut EntityCellHead,
    components: Vec<&'a mut dyn Component>,
    _entity_writer: RwLockWriteGuard<'a, ()>,
}

impl<'a> EntityWriteGuard<'a> {
    pub(crate) fn new(
        buffer: &'a [u8],
        buffer_index: usize,
        components_per_entity: usize,
        entity_size: usize,
    ) -> EntityWriteGuard<'a> {
        // TODO: Try a way to remove that (ignore the fact that archetype reader is local)
        let buffer = unsafe { &*(&buffer as *const _) } as &[u8];
        let buffer = unsafe { &mut *(&buffer as &[u8] as *const [u8] as *mut [u8]) } as &mut [u8];

        let mut entity_head = decode_entity_head_mut(buffer, buffer_index);
        let entity_head_2 = unsafe { &mut *(&mut entity_head as *mut _) } as &mut EntityCellHead;
        let entity_head_3 = unsafe { &mut *(&mut entity_head as *mut _) } as &mut EntityCellHead;
        let components = decode_components_mut(entity_head_2, components_per_entity, entity_size);

        EntityWriteGuard {
            entity_head,
            components,
            _entity_writer: entity_head_3.lock.write().unwrap(),
        }
    }

    /// Get the list of the components in the entity
    pub fn get_components(&self) -> &[&'a mut dyn Component] {
        &self.components
    }

    /// Get the list of the components in the entity with mutability
    pub fn get_components_mut(&mut self) -> &mut [&'a mut dyn Component] {
        &mut self.components
    }

    /// Get a component rwlock
    ///
    /// # Arguments
    /// * `component_type` - The component type
    ///
    pub fn get_component_mut<T: Component>(&mut self, component_type: &str) -> Option<&mut T> {
        match self
            .get_components_mut()
            .iter_mut()
            .find(|component| component.get_class_name() == component_type)
        {
            Some(component) => match component.as_any_mut().downcast_mut::<T>() {
                Some(component) => Some(component),
                None => None,
            },
            None => None,
        }
    }
}

impl<'a> Deref for EntityWriteGuard<'a> {
    type Target = EntityCellHead;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.entity_head
    }
}

impl<'a> DerefMut for EntityWriteGuard<'a> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        &mut self.entity_head
    }
}

impl<'a> Debug for EntityWriteGuard<'a> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.get_components().fmt(formatter)
    }
}

impl IntrospectObject for EntitySharedRwLock {
    fn get_class_name(&self) -> String {
        "EntitySharedRwLock".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "get_component".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = cast_introspect_ref::<EntitySharedRwLock>(this);

                    let mut caster = ArgumentCaster::new("get_component", args);
                    let arg1 = caster.cast_next::<String>()?;

                    let result = this.get_component(&arg1);

                    Ok(result.map(|result| Serialized::NativeObject(Box::new(result))))
                })),
            },
            MethodInfo {
                name: "contains".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = cast_introspect_ref::<EntitySharedRwLock>(this);

                    let mut caster = ArgumentCaster::new("contains", args);
                    let arg1 = caster.cast_next::<Vec<String>>()?;

                    let result = this.contains(&EntityTypeIdentifier(arg1));

                    Ok(Some(Serialized::Bool(result)))
                })),
            },
            MethodInfo {
                name: "len".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, _args| {
                    let this = cast_introspect_ref::<EntitySharedRwLock>(this);
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
                ty: TypeId::of::<u64>(),
                serializable: false,
                getter: Arc::new(|this| {
                    let this = cast_introspect_ref::<EntitySharedRwLock>(this);
                    let reader = this.read();

                    reader.entity_id.fruity_into()
                }),
                setter: SetterCaller::None,
            },
            FieldInfo {
                name: "on_deleted".to_string(),
                ty: TypeId::of::<Signal<()>>(),
                serializable: false,
                getter: Arc::new(|this| {
                    let this = cast_introspect_ref::<EntitySharedRwLock>(this);
                    let reader = this.read();

                    reader.on_deleted.clone().fruity_into()
                }),
                setter: SetterCaller::None,
            },
            FieldInfo {
                name: "components".to_string(),
                ty: TypeId::of::<ComponentRwLock>(),
                serializable: true,
                getter: Arc::new(|this| {
                    let this = cast_introspect_ref::<EntitySharedRwLock>(this);

                    Serialized::Array(
                        this.iter_all_components()
                            .map(|component| Serialized::NativeObject(Box::new(component)))
                            .collect::<Vec<_>>(),
                    )
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

impl FruityInto<Serialized> for EntitySharedRwLock {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self.clone()))
    }
}

impl FruityTryFrom<Serialized> for EntitySharedRwLock {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                match value.clone().as_any_box().downcast::<EntitySharedRwLock>() {
                    Ok(value) => Ok(*value),
                    Err(_) => Err(format!(
                        "Couldn't convert a EntitySharedRwLock to native object"
                    )),
                }
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

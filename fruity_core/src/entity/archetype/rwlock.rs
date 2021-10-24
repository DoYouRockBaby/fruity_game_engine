use crate::component::component_list_rwlock::ComponentListRwLock;
use crate::entity::archetype::Archetype;
use crate::entity::archetype::Component;
use crate::entity::archetype::ComponentDecodingInfos;
use crate::entity::archetype::EntityTypeIdentifier;
use crate::service::utils::cast_service;
use fruity_any::*;
use fruity_introspect::serialize::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use itertools::Itertools;
use std::any::Any;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::mem::size_of;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

/// This is an rwlock for an entity, is intended to be used by archetype
/// Extern users are not supposed to have access to that
#[derive(FruityAny)]
pub struct EntityRwLock {
    archetype_ptr: AtomicPtr<Archetype>,
    // If it's a writer that handle the lock, the reader_count is maximal cause we cannot create more
    // Otherwise, this is incremented by one per reader added
    reader_count: AtomicUsize,
    arc_count: AtomicUsize,
}

impl EntityRwLock {
    /// Returns a EntityRwLock
    pub fn new(archetype: &mut Archetype) -> EntityRwLock {
        let test = AtomicPtr::new(archetype as *mut Archetype);

        EntityRwLock {
            archetype_ptr: test,
            reader_count: AtomicUsize::new(0),
            arc_count: AtomicUsize::new(0),
        }
    }

    /// This create a new RwLock weak, it will be used by the extern user to access datas
    pub(crate) fn create_new_weak(&self) -> EntityRwLockWeak {
        self.arc_count.fetch_add(1, Ordering::SeqCst);

        EntityRwLockWeak {
            entity_rwlock_ptr: AtomicPtr::new(self as *const _ as *mut EntityRwLock),
        }
    }

    /// Create a read guard over the entity RwLock
    pub fn read(&self) -> EntityReadGuard {
        let archetype = unsafe { &*self.archetype_ptr.load(Ordering::SeqCst) };
        EntityReadGuard::new(archetype, self)
    }

    /// Create a write guard over the entity RwLock
    pub fn write(&self) -> EntityWriteGuard {
        let archetype = unsafe { &*self.archetype_ptr.load(Ordering::SeqCst) };
        EntityWriteGuard::new(archetype, self)
    }
}

impl Debug for EntityRwLock {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let reader = self.read();
        reader.fmt(formatter)
    }
}

/// A weak over an entity RwLock, this is the handle that will be used by the extern user to access datas
/// This can be clone and works like an Arc but over a reference that it don't own and have access to the
/// reference RwLock functionalities
#[derive(FruityAny, Debug)]
pub struct EntityRwLockWeak {
    entity_rwlock_ptr: AtomicPtr<EntityRwLock>,
}

impl EntityRwLockWeak {
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
        let archetype = unsafe { &*self.archetype_ptr.load(Ordering::SeqCst) };
        let intern_identifier = archetype.get_type_identifier();

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

impl Drop for EntityRwLockWeak {
    fn drop(&mut self) {
        // Decrement the lock weak counter
        let entity_rwlock = unsafe { &*self.entity_rwlock_ptr.load(Ordering::SeqCst) };
        entity_rwlock.arc_count.fetch_sub(1, Ordering::SeqCst);
    }
}

impl Clone for EntityRwLockWeak {
    fn clone(&self) -> Self {
        let entity_rwlock = unsafe { &*self.entity_rwlock_ptr.load(Ordering::SeqCst) };
        entity_rwlock.create_new_weak()
    }
}

impl Deref for EntityRwLockWeak {
    type Target = EntityRwLock;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &*self.entity_rwlock_ptr.load(Ordering::SeqCst) }
    }
}

/// An entity guard that can be used to access an entity without mutability
pub struct EntityReadGuard<'a> {
    entity_lock: &'a EntityRwLock,
    components: Vec<&'a dyn Component>,
}

impl<'a> EntityReadGuard<'a> {
    pub(crate) fn new(archetype: &Archetype, entity_lock: &'a EntityRwLock) -> EntityReadGuard<'a> {
        // Wait that every write locker is done
        while entity_lock.reader_count.load(Ordering::SeqCst) == usize::MAX {}

        // Poison the entity cell in the archetype
        entity_lock.reader_count.fetch_add(1, Ordering::SeqCst);

        // Build the component ref vector
        let components = Self::build_component_ref_vector(archetype, entity_lock);

        EntityReadGuard {
            entity_lock,
            components,
        }
    }

    fn build_component_ref_vector<'b>(
        archetype: &Archetype,
        rwlock: &'b EntityRwLock,
    ) -> Vec<&'b dyn Component> {
        let (_, component_infos_buffer, components_buffer) = get_entry_buffers(archetype, rwlock);

        // Get component decoding infos
        let component_decoding_infos = get_component_decoding_infos(component_infos_buffer);

        // Deserialize every components
        let components = component_decoding_infos
            .iter()
            .map(move |decoding_info| {
                let components_buffer = unsafe { &*(components_buffer as *const _) } as &[u8];

                let component_buffer_index = decoding_info.relative_index;
                let component_buffer_end = component_buffer_index + decoding_info.size;
                let component_buffer =
                    &components_buffer[component_buffer_index..component_buffer_end];

                (decoding_info.decoder)(component_buffer)
            })
            .collect::<Vec<_>>();

        components
    }
}

impl<'a> Deref for EntityReadGuard<'a> {
    type Target = [&'a dyn Component];

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.components
    }
}

impl<'a> Drop for EntityReadGuard<'a> {
    fn drop(&mut self) {
        // Decrement the lock guard counter
        self.entity_lock.reader_count.fetch_sub(1, Ordering::SeqCst);
    }
}

impl<'a> Debug for EntityReadGuard<'a> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.deref().fmt(formatter)
    }
}

/// An entity guard that can be used to access an entity with mutability
pub struct EntityWriteGuard<'a> {
    entity_lock: &'a EntityRwLock,
    components: Vec<&'a mut dyn Component>,
}

impl<'a> EntityWriteGuard<'a> {
    pub(crate) fn new(
        archetype: &Archetype,
        entity_lock: &'a EntityRwLock,
    ) -> EntityWriteGuard<'a> {
        // Wait that every write locker is done
        while entity_lock.reader_count.load(Ordering::SeqCst) == 0 {}

        // Poison the entity cell in the archetype
        entity_lock.reader_count.store(usize::MAX, Ordering::SeqCst);

        // Build the component ref vector
        let components = Self::build_component_ref_vector(archetype, entity_lock);

        EntityWriteGuard {
            entity_lock,
            components,
        }
    }

    fn build_component_ref_vector<'b>(
        archetype: &Archetype,
        rwlock: &'a EntityRwLock,
    ) -> Vec<&'a mut dyn Component> {
        let (_, component_infos_buffer, components_buffer) =
            get_entry_buffers_mut(archetype, rwlock);

        // Get component decoding infos
        let component_decoding_infos = get_component_decoding_infos(component_infos_buffer);

        // Deserialize every components
        let components = component_decoding_infos
            .into_iter()
            .map(|decoding_info| {
                let components_buffer =
                    unsafe { &mut *(components_buffer as *mut _) } as &mut [u8];

                let component_buffer_index = decoding_info.relative_index;
                let component_buffer_end = component_buffer_index + decoding_info.size;
                let component_buffer =
                    &mut components_buffer[component_buffer_index..component_buffer_end];

                (decoding_info.decoder_mut)(component_buffer)
            })
            .collect::<Vec<_>>();

        components
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

impl<'a> Drop for EntityWriteGuard<'a> {
    fn drop(&mut self) {
        // Decrement the lock guard counter
        self.entity_lock.reader_count.store(0, Ordering::SeqCst);
    }
}

impl IntrospectObject for EntityRwLock {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![MethodInfo {
            name: "len".to_string(),
            call: MethodCaller::Const(Arc::new(move |this, _args| {
                let this = unsafe { &*(this as *const _) } as &dyn Any;
                let this = cast_service::<EntityRwLock>(this);
                let this = this.read();

                let result = this.len();

                Ok(Some(Serialized::USize(result)))
            })),
        }]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

// Get the entry buffer and split it
// Split the entity buffer into three other ones, one for the lock, one
// for the encoding infos and one for the component datas
fn get_entry_buffers<'a>(
    archetype: &Archetype,
    rwlock: &'a EntityRwLock,
) -> (&'a [u8], &'a [u8], &'a [u8]) {
    // Get the whole entity buffer
    let components_per_entity = archetype.components_per_entity;
    let entity_size = archetype.entity_size;
    let entity_buffer = unsafe {
        std::slice::from_raw_parts((&*rwlock as *const EntityRwLock) as *const u8, entity_size)
    };

    // Split the entity buffer into three other one
    let (rwlock_buffer, rest) = entity_buffer.split_at(size_of::<EntityRwLock>());
    let (component_infos_buffer, components_buffer) =
        rest.split_at(components_per_entity * size_of::<ComponentDecodingInfos>());

    (rwlock_buffer, component_infos_buffer, components_buffer)
}

// Get the entry buffer with mutability and split it
// Split the entity buffer into three other ones, one for the lock, one
// for the encoding infos and one for the component datas
fn get_entry_buffers_mut<'a>(
    archetype: &Archetype,
    rwlock: &'a EntityRwLock,
) -> (&'a mut [u8], &'a mut [u8], &'a mut [u8]) {
    // Get the whole entity buffer
    let components_per_entity = archetype.components_per_entity;
    let entity_size = archetype.entity_size;
    let entity_buffer = unsafe {
        std::slice::from_raw_parts_mut(
            (&*rwlock as *const EntityRwLock as *mut EntityRwLock) as *mut u8,
            entity_size,
        )
    };

    // Split the entity buffer into three other one
    let (rwlock_buffer, rest) = entity_buffer.split_at_mut(size_of::<EntityRwLock>());
    let (component_infos_buffer, components_buffer) =
        rest.split_at_mut(components_per_entity * size_of::<ComponentDecodingInfos>());

    (rwlock_buffer, component_infos_buffer, components_buffer)
}

// Get the components decoding infos for an entity in the archetype
fn get_component_decoding_infos(entity_bufer: &[u8]) -> &[ComponentDecodingInfos] {
    let (_head, body, _tail) = unsafe { entity_bufer.align_to::<ComponentDecodingInfos>() };
    body
}

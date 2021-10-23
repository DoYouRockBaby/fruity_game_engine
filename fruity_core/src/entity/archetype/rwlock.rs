use crate::component::component_list_guard::ComponentListReadGuard;
use crate::component::component_list_guard::ComponentListWriteGuard;
use crate::entity::archetype::Archetype;
use crate::entity::archetype::Component;
use crate::entity::archetype::ComponentDecodingInfos;
use crate::entity::archetype::EntityLockCell;
use crate::entity::archetype::EntityTypeIdentifier;
use itertools::Itertools;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::mem::size_of;
use std::ops::Deref;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;

pub struct EntityRwLock {
    archetype: AtomicPtr<Archetype>,
    buffer_index: usize,
}

impl EntityRwLock {
    pub(crate) fn new(archetype: &Archetype, buffer_index: usize) -> EntityRwLock {
        EntityRwLock {
            archetype: AtomicPtr::new(archetype as *const _ as *mut Archetype),
            buffer_index,
        }
    }

    pub fn read(&self) -> EntityReadGuard {
        let archetype = &*self.archetype.load(Ordering::SeqCst);
        EntityReadGuard::new(archetype, self.buffer_index)
    }

    pub fn write(&self) -> EntityWriteGuard {
        let archetype = &*self.archetype.load(Ordering::SeqCst);
        EntityWriteGuard::new(archetype, self.buffer_index)
    }
}

impl Debug for EntityRwLock {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let reader = self.read();
        reader.fmt(formatter)
    }
}

pub struct EntityReadGuard<'a> {
    entity_lock: &'a EntityLockCell,
    components: Vec<&'a dyn Component>,
}

impl<'a> EntityReadGuard<'a> {
    pub(crate) fn new(archetype: &Archetype, buffer_index: usize) -> EntityReadGuard {
        // Wait that every write locker is done
        let entity_lock = get_lock_in_archetype(archetype as &Archetype, buffer_index);
        while entity_lock.reader_count.load(Ordering::SeqCst) == usize::MAX {}

        // Poison the entity cell in the archetype
        entity_lock.reader_count.fetch_add(1, Ordering::SeqCst);

        // Build the component ref vector
        let components = Self::build_component_ref_vector(archetype, buffer_index);

        EntityReadGuard {
            entity_lock,
            components,
        }
    }

    fn build_component_ref_vector(
        archetype: &Archetype,
        buffer_index: usize,
    ) -> Vec<&dyn Component> {
        // Get component decoding infos
        let component_decoding_infos =
            get_component_decoding_infos_in_archetype(archetype, buffer_index);

        // Get the buffer for all the components
        let all_components_buffer_index = buffer_index
            + size_of::<EntityLockCell>()
            + archetype.components_per_entity * size_of::<ComponentDecodingInfos>();
        let all_components_buffer = &archetype.buffer[all_components_buffer_index..];

        // Deserialize every components
        let components = component_decoding_infos
            .iter()
            .map(|decoding_info| {
                let component_buffer_index =
                    all_components_buffer_index + decoding_info.relative_index;
                let component_buffer_end = component_buffer_index + decoding_info.size;
                let component_buffer =
                    &archetype.buffer[component_buffer_index..component_buffer_end];

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

pub struct EntityWriteGuard<'a> {
    entity_lock: &'a EntityLockCell,
    components: Vec<&'a mut dyn Component>,
}

impl<'a> EntityWriteGuard<'a> {
    pub(crate) fn new(archetype: &Archetype, buffer_index: usize) -> EntityWriteGuard {
        // Wait that every write locker is done
        let entity_lock = get_lock_in_archetype(archetype, buffer_index);
        while entity_lock.reader_count.load(Ordering::SeqCst) == 0 {}

        // Poison the entity cell in the archetype
        entity_lock.reader_count.store(usize::MAX, Ordering::SeqCst);

        // Build the component ref vector
        let components = Self::build_component_ref_vector(archetype, buffer_index);

        EntityWriteGuard {
            entity_lock,
            components,
        }
    }

    fn build_component_ref_vector(
        archetype: &Archetype,
        buffer_index: usize,
    ) -> Vec<&mut dyn Component> {
        // Get component decoding infos
        let component_decoding_infos =
            get_component_decoding_infos_in_archetype(archetype, buffer_index);

        // Get the buffer for all the components
        let all_components_buffer_index = buffer_index
            + size_of::<EntityLockCell>()
            + archetype.components_per_entity * size_of::<ComponentDecodingInfos>();
        let all_components_buffer = &archetype.buffer[all_components_buffer_index..];

        // Deserialize every components
        let components = component_decoding_infos
            .iter()
            .map(|decoding_info| {
                let component_buffer_index =
                    all_components_buffer_index + decoding_info.relative_index;
                let component_buffer_end = component_buffer_index + decoding_info.size;
                let component_buffer =
                    &mut archetype.buffer[component_buffer_index..component_buffer_end];

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

// Get the lock cell for an entity in the archetype
fn get_lock_in_archetype(archetype: &Archetype, buffer_index: usize) -> &EntityLockCell {
    let buffer_end = buffer_index + size_of::<EntityLockCell>();
    let entity_lock_buffer = &archetype.buffer[buffer_index..buffer_end];
    let (_head, body, _tail) = unsafe { entity_lock_buffer.align_to::<EntityLockCell>() };
    &body[0]
}

// Get the components decoding infos for an entity in the archetype
fn get_component_decoding_infos_in_archetype(
    archetype: &Archetype,
    buffer_index: usize,
) -> &[ComponentDecodingInfos] {
    let components_decoding_infos_buffer_index = buffer_index + size_of::<EntityLockCell>();
    let components_decoding_infos_buffer_end = buffer_index + archetype.entity_size;
    let components_decoding_infos_buffer = &archetype.buffer
        [components_decoding_infos_buffer_index..components_decoding_infos_buffer_end];
    let (_head, body, _tail) =
        unsafe { components_decoding_infos_buffer.align_to::<ComponentDecodingInfos>() };
    body
}

impl EntityRwLock {
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
    ) -> impl Iterator<Item = ComponentListReadGuard> {
        let archetype = &*self.archetype.load(Ordering::SeqCst);
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

        component_indexes.map(move |component_indexes| {
            ComponentListReadGuard::new(self.read(), component_indexes.clone())
        })
    }

    /// Get collections of components list writer
    /// Cause an entity can contain multiple component of the same type, can returns multiple writers
    /// All components are mapped to the provided component identifiers in the same order
    ///
    /// # Arguments
    /// * `type_identifiers` - The identifier list of the components, components will be returned with the same order
    ///
    pub fn iter_components_mut(
        &self,
        target_identifier: &EntityTypeIdentifier,
    ) -> impl Iterator<Item = ComponentListWriteGuard> {
        let archetype = &*self.archetype.load(Ordering::SeqCst);
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

        component_indexes.map(move |component_indexes| {
            ComponentListWriteGuard::new(self.write(), component_indexes.clone())
        })
    }
}

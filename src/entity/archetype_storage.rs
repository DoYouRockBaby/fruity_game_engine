use std::sync::Mutex;
use std::sync::Arc;
use std::fmt::Debug;
use hashbrown::HashMap;
use hashbrown::hash_map::Iter as HashMapIter;
use rayon::prelude::*;
use crate::entity::entity_manager::RemoveEntityError;
use crate::component::component::Component;
use crate::entity::entity::EntityId;
use crate::entity::archetype::ArchetypeComponentType;

#[derive(Clone)]
struct ArchetypeStorageDatas {
    component_types: Vec<ArchetypeComponentType>,
    entity_size: usize,
    buffer: Vec<u8>,
}

#[derive(Clone)]
pub struct ArchetypeStorage {
    entity_indexes: HashMap<EntityId, usize>,
    datas: ArchetypeStorageDatas,
}

impl ArchetypeStorage  {
    pub fn new(entity_size: usize, component_types: Vec<ArchetypeComponentType>) -> ArchetypeStorage {
        ArchetypeStorage {
            entity_indexes: HashMap::new(),
            datas: ArchetypeStorageDatas::new(entity_size, component_types),
        }
    }

    pub fn get(&self, entity_id: EntityId) -> Option<Vec<&dyn Component>> {
        match self.entity_indexes.get(&entity_id) {
            Some(index) => self.datas.get_by_index(*index),
            None => None,
        }
    }

    pub fn get_mut(&mut self, entity_id: EntityId) -> Option<Vec<&mut dyn Component>> {
        match self.entity_indexes.get(&entity_id) {
            Some(index) => self.datas.get_mut_by_index(*index),
            None => None,
        }
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            storage: &self,
            entity_indexes_iterator: self.entity_indexes.iter(),
        }
    }

    pub fn add(&mut self, entity_id: EntityId, components: &[&mut dyn Component]) {
        let index = self.datas.add(ArchetypeStorage::encode_entity(components));
        self.entity_indexes.insert(entity_id, index);
    }

    pub fn remove(&mut self, entity_id: EntityId) -> Result<(), RemoveEntityError> {
        // Remove old stored id
        match self.entity_indexes.remove(&entity_id) {
            Some(index) => {
                // Remove associated binary datas
                self.datas.remove(index);

                // Gap all existing indexes
                self.entity_indexes
                    .iter_mut()
                    .for_each(|(_, index_2)| {
                        if *index_2 > index {
                            *index_2 -= 1;
                        }
                    });
                
                Ok(())
            },
            None => {
                Err(RemoveEntityError::NotFound)
            },
        }
    }

    pub fn for_each<F: Fn(&mut [&mut dyn Component]) + Send + Sync>(&mut self, callback: F) {
        /*let datas = Arc::new(Mutex::new(&mut self.datas));

        self.entity_indexes
            .iter()
            .par_bridge()
            .for_each(|(_, index)| {
                match datas.clone().lock().unwrap().get_mut_by_index(*index) {
                    Some(mut components) => {
                        callback(&mut components[..]);
                    },
                    None => (),
                }
            });*/

        for (_, index) in self.entity_indexes.iter() {
            match self.datas.get_mut_by_index(*index) {
                Some(mut components) => {
                    callback(&mut components[..]);
                },
                None => (),
            }
        }
    }

    fn encode_entity(components: &[&mut dyn Component]) -> Vec<u8> {
        components
            .iter()
            .map(|component| component.encode())
            .flatten()
            .collect()
    }
}

impl ArchetypeStorageDatas {
    pub fn new(entity_size: usize, component_types: Vec<ArchetypeComponentType>) -> ArchetypeStorageDatas {
        ArchetypeStorageDatas {
            component_types,
            entity_size,
            buffer: Vec::new(),
        }
    }

    pub fn get_by_index(&self, index: usize) -> Option<Vec<&dyn Component>> {
        let mut components: Vec<&dyn Component> = Vec::new();
        let entity_slices = self.buffer.chunks(self.entity_size);
        
        match entity_slices.skip(index).next() {
            Some(mut entity_slice) => {
                for component_type in &self.component_types {
                    let (component_slice, rest_slice) = entity_slice.split_at(component_type.size);
                    entity_slice = rest_slice;
                    components.push((component_type.decoder)(component_slice));
                }

                Some(components)
            },
            None => None,
        }
    }

    pub fn get_mut_by_index(&mut self, index: usize) -> Option<Vec<&mut dyn Component>> {
        let mut components: Vec<&mut dyn Component> = Vec::new();
        let entity_slices = self.buffer.chunks_mut(self.entity_size);
        
        match entity_slices.skip(index).next() {
            Some(mut entity_slice) => {
                for component_type in &self.component_types {
                    let (component_slice, rest_slice) = entity_slice.split_at_mut(component_type.size);
                    entity_slice = rest_slice;
                    components.push((component_type.decoder_mut)(component_slice));
                }

                Some(components)
            },
            None => None,
        }
    }

    pub fn add(&mut self, mut datas: Vec<u8>) -> usize {
        let index = self.buffer.len() / self.entity_size;
        self.buffer.append(&mut datas);
        index
    }

    pub fn remove(&mut self, index: usize) {
        let data_index = index * self.entity_size;
        let data_end = data_index + self.entity_size;
        self.buffer.drain(data_index .. data_end);
    }
}

impl Debug for ArchetypeStorage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let fmt_error = self.iter().find_map(|elem| {
            match elem.fmt(formatter) {
                Ok(()) => None,
                Err(err) => Some(err),
            }
        });

        match fmt_error {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }
}

pub struct Iter<'s> {
    storage: &'s ArchetypeStorage,
    entity_indexes_iterator: HashMapIter<'s, EntityId, usize>,
}

impl<'s> Iterator for Iter<'s> {
    type Item = (EntityId, Vec<&'s dyn Component>);

    fn next(&mut self) -> Option<(EntityId, Vec<&'s dyn Component>)> {
        match self.entity_indexes_iterator.next() {
            Some((entity_id, _)) => match self.storage.get(*entity_id) {
                Some(components) => Some((*entity_id, components)),
                None => None,
            },
            None => None,
        }
    }
}
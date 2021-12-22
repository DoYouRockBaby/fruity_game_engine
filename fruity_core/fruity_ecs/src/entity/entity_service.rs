use crate::component::component::AnyComponent;
use crate::component::component_reference::ComponentReference;
use crate::entity::archetype::entity::Entity;
use crate::entity::archetype::Archetype;
use crate::entity::entity::get_type_identifier_by_any;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_query::QueryInject;
use crate::ResourceContainer;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::serialize::yaml::deserialize_yaml;
use fruity_core::serialize::Deserialize;
use fruity_core::serialize::Serialize;
use fruity_core::utils::introspect::cast_introspect_ref;
use fruity_core::utils::introspect::ArgumentCaster;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::sync::Arc;
use std::sync::RwLock;

/// An error over entity deletion
pub enum RemoveEntityError {
    /// The entity don't exists in any archetype storage
    NotFound,
}

struct InnerEntityService {
    id_incrementer: u64,
    index_map: HashMap<EntityId, (usize, usize)>,
    archetypes: Vec<Archetype>,
}

/// A storage for every entities, use [’Archetypes’] to store entities of different types
#[derive(FruityAny)]
pub struct EntityService {
    inner: Arc<RwLock<InnerEntityService>>,
    object_factory_service: ResourceReference<ObjectFactoryService>,
}

/// A save for the entities stored in an [’EntityService’]
#[derive(Clone, Debug)]
pub struct EntityServiceSnapshot(pub Serialized);

impl Debug for EntityService {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl EntityService {
    /// Returns an EntityService
    pub fn new(resource_container: Arc<ResourceContainer>) -> EntityService {
        EntityService {
            inner: Arc::new(RwLock::new(InnerEntityService {
                id_incrementer: 0,
                index_map: HashMap::new(),
                archetypes: Vec::new(),
            })),
            object_factory_service: resource_container.require::<ObjectFactoryService>(),
        }
    }

    /// Get an entity specific components
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `component_identifier` - The component identifiers
    ///
    pub fn get_entity(
        &self,
        entity_id: EntityId,
        component_identifier: EntityTypeIdentifier,
    ) -> Option<Vec<ComponentReference>> {
        let inner = self.inner.read().unwrap();
        inner
            .index_map
            .get(&entity_id)
            .map(|(archetype_index, entity_index)| {
                inner.archetypes[*archetype_index]
                    .get_components(*entity_index, component_identifier)
            })
    }

    /// Get an entity all components
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get_full_entity(&self, entity_id: EntityId) -> Option<Vec<ComponentReference>> {
        let inner = self.inner.read().unwrap();
        inner
            .index_map
            .get(&entity_id)
            .map(|(archetype_index, entity_index)| {
                inner.archetypes[*archetype_index].get_full_entity(*entity_index)
            })
    }

    /// Iterate over all entities
    pub fn iter_all_entities(&self) -> impl Iterator<Item = Vec<ComponentReference>> {
        let inner = self.inner.read().unwrap();
        let archetypes = unsafe { &*(&inner.archetypes as *const _) } as &Vec<Archetype>;
        archetypes
            .iter()
            .map(|archetype| archetype.iter_all_components())
            .flatten()
    }

    /// Iterate over all entities with a specific archetype type
    /// Use every entity that contains the provided entity type
    ///
    /// # Arguments
    /// * `entity_identifier` - The entity type identifier
    ///
    pub fn iter_components(
        &self,
        entity_identifier: EntityTypeIdentifier,
    ) -> impl Iterator<Item = Vec<ComponentReference>> {
        let inner = self.inner.read().unwrap();
        let archetypes = unsafe { &*(&inner.archetypes as *const _) } as &Vec<Archetype>;

        let entity_identifier_2 = entity_identifier.clone();
        archetypes
            .iter()
            .filter(move |archetype| archetype.get_type_identifier().contains(&entity_identifier))
            .map(move |archetype| archetype.iter(entity_identifier_2.clone()))
            .flatten()
    }

    /// Execute a closure over all entities with a specific archetype type
    /// Use every entity that contains the provided entity type
    /// Also map components to the order of provided entity type
    /// identifier
    ///
    /// # Arguments
    /// * `entity_identifier` - The entity type identifier
    /// * `callback` - The closure to execute
    ///
    pub fn for_each<'a>(
        &self,
        entity_identifier: EntityTypeIdentifier,
        callback: impl QueryInject,
    ) {
        self.iter_components(entity_identifier)
            .par_bridge()
            .for_each(|components| {
                let callback = callback.duplicate();
                (callback.inject())(components)
            });
    }

    /// Add a new entity in the storage
    /// Create the archetype if it don't exists
    /// Returns the newly created entity id
    ///
    /// # Arguments
    /// * `name` - The name of the entity
    /// * `components` - The components that will be added
    ///
    pub fn create(&self, name: &str, components: Vec<AnyComponent>) -> EntityId {
        let entity_id = {
            let mut inner = self.inner.write().unwrap();
            inner.id_incrementer += 1;
            inner.id_incrementer
        };

        self.create_with_id(entity_id, name, components)
    }

    /// Add a new entity in the storage
    /// Create the archetype if it don't exists
    /// Returns the newly created entity id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `name` - The name of the entity
    /// * `components` - The components that will be added
    ///
    pub fn create_with_id(
        &self,
        entity_id: EntityId,
        name: &str,
        mut components: Vec<AnyComponent>,
    ) -> EntityId {
        let entity_id = {
            let mut inner = self.inner.write().unwrap();
            inner.id_incrementer = u64::max(entity_id + 1, inner.id_incrementer);
            entity_id
        };

        components.sort_by(|a, b| a.get_class_name().cmp(&b.get_class_name()));
        let entity_identifier = get_type_identifier_by_any(&components);

        let indexes = match self.archetype_by_identifier(entity_identifier) {
            Some((archetype_index, archetype)) => {
                let archetype_entity_index = archetype.len();
                archetype.add(entity_id, name, components);

                (archetype_index, archetype_entity_index)
            }
            None => {
                let mut inner = self.inner.write().unwrap();
                let archetype_index = inner.archetypes.len();
                let archetype = Archetype::new(entity_id, name, components);
                inner.archetypes.push(archetype);
                (archetype_index, 0)
            }
        };

        let mut inner = self.inner.write().unwrap();
        inner.index_map.insert(entity_id, indexes);

        // self.on_entity_created.notify(self.get(entity_id).unwrap());
        entity_id
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn remove(&self, entity_id: EntityId) -> Result<(), RemoveEntityError> {
        let indexes = {
            let mut inner = self.inner.write().unwrap();
            inner.index_map.remove(&entity_id)
        };

        if let Some(indexes) = indexes {
            let inner = self.inner.read().unwrap();

            let archetype = inner.archetypes.get(indexes.0).unwrap();
            archetype.remove(indexes.1);

            Ok(())
        } else {
            Err(RemoveEntityError::NotFound)
        }
    }

    /// Add components to an entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `component_index` - The component index, is based on alphabetical number of the component type name
    ///
    pub fn add_component(
        &self,
        entity_id: EntityId,
        mut components: Vec<AnyComponent>,
    ) -> Result<(), RemoveEntityError> {
        let indexes = {
            let mut inner = self.inner.write().unwrap();
            inner.index_map.remove(&entity_id)
        };

        if let Some(indexes) = indexes {
            let (old_entity, mut old_components) = {
                let inner = self.inner.read().unwrap();

                let archetype = inner.archetypes.get(indexes.0).unwrap();
                archetype.remove(indexes.1)
            };

            old_components.append(&mut components);

            self.create_with_id(entity_id, &old_entity.name, old_components);

            Ok(())
        } else {
            Err(RemoveEntityError::NotFound)
        }
    }

    /// Remove a component from an entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `component_index` - The component index, is based on alphabetical number of the component type name
    ///
    pub fn remove_component(
        &self,
        entity_id: EntityId,
        component_index: usize,
    ) -> Result<(), RemoveEntityError> {
        let indexes = {
            let mut inner = self.inner.write().unwrap();
            inner.index_map.remove(&entity_id)
        };

        if let Some(indexes) = indexes {
            let (old_entity, mut old_components) = {
                let inner = self.inner.read().unwrap();

                let archetype = inner.archetypes.get(indexes.0).unwrap();
                archetype.remove(indexes.1)
            };

            // propagate the deleted signal
            old_entity.on_deleted.notify(());

            old_components.remove(component_index);

            self.create_with_id(entity_id, &old_entity.name, old_components);

            Ok(())
        } else {
            Err(RemoveEntityError::NotFound)
        }
    }

    fn archetype_by_identifier(
        &self,
        entity_identifier: EntityTypeIdentifier,
    ) -> Option<(usize, &Archetype)> {
        let inner = self.inner.read().unwrap();

        inner
            .archetypes
            .iter()
            .enumerate()
            .find(|(_index, archetype)| *archetype.get_type_identifier() == entity_identifier)
            .map(|(index, archetype)| {
                (index, unsafe {
                    std::mem::transmute::<&Archetype, &Archetype>(archetype)
                })
            })
    }

    /// Clear all the entities
    pub fn clear(&self) {
        // Clear all entities
        let mut inner = self.inner.write().unwrap();
        inner.id_incrementer = 0;
        inner.archetypes.clear();
    }

    /// Create a snapshot over all the entities
    pub fn snapshot(&self) -> EntityServiceSnapshot {
        let serialized_entities = self
            .iter_all_entities()
            .filter_map(|components| {
                let serialized_components = Serialized::Array(
                    components
                        .iter()
                        .filter_map(|component| component.serialize())
                        .collect::<Vec<_>>(),
                );

                Some(serialized_components)
            })
            .collect::<Vec<_>>();

        EntityServiceSnapshot(Serialized::Array(serialized_entities))
    }

    /// Restore an entity snapshot from a file
    ///
    /// # Arguments
    /// * `filepath` - The file path
    ///
    pub fn restore_from_file(&self, filepath: &str) {
        if let Ok(mut reader) = File::open(&filepath) {
            if let Some(snapshot) = deserialize_yaml(&mut reader) {
                self.restore(&EntityServiceSnapshot(snapshot));
            }
        }
    }

    /// Restore an entity snapshot
    ///
    /// # Arguments
    /// * `snapshot` - The snapshot
    ///
    pub fn restore(&self, snapshot: &EntityServiceSnapshot) {
        self.clear();

        if let Serialized::Array(entities) = &snapshot.0 {
            entities
                .iter()
                .for_each(|serialized_entity| self.restore_entity(serialized_entity));
        }
    }

    fn restore_entity(&self, serialized_entity: &Serialized) {
        let object_factory_service = self.object_factory_service.read();

        let (entity_properties, components) =
            if let Serialized::Array(components) = serialized_entity {
                let entity_properties = components.iter().find_map(|serialized_component| {
                    Entity::deserialize(serialized_component, &object_factory_service)
                });

                if let Some(entity_properties) = entity_properties {
                    let components = components
                        .iter()
                        .filter_map(|serialized_component| {
                            AnyComponent::deserialize(serialized_component, &object_factory_service)
                        })
                        .collect::<Vec<_>>();

                    (entity_properties, components)
                } else {
                    return;
                }
            } else {
                return;
            };

        let entity_id = self.create(&entity_properties.name, components);
        let mut components = self
            .get_entity(entity_id, EntityTypeIdentifier(vec!["Entity".to_string()]))
            .unwrap();
        let entity = components.get_mut(0).unwrap();
        let mut entity = entity.write();
        let mut entity = entity.as_any_mut().downcast_mut::<Entity>().unwrap();
        entity.enabled = entity_properties.enabled;
    }
}

impl IntrospectObject for EntityService {
    fn get_class_name(&self) -> String {
        "EntityService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "get_entity".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = cast_introspect_ref::<EntityService>(this);

                    let mut caster = ArgumentCaster::new("get_entity", args);
                    let arg1 = caster.cast_next::<EntityId>()?;
                    let arg2 = caster.cast_next::<Vec<String>>()?;

                    let result =
                        this.get_entity(arg1, EntityTypeIdentifier(arg2))
                            .map(|components| {
                                Serialized::Array(
                                    components
                                        .into_iter()
                                        .map(|component| {
                                            Serialized::NativeObject(Box::new(component))
                                        })
                                        .collect::<Vec<_>>(),
                                )
                            });

                    Ok(result)
                })),
            },
            MethodInfo {
                name: "iter_components".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = cast_introspect_ref::<EntityService>(this);

                    let mut caster = ArgumentCaster::new("iter_components", args);
                    let arg1 = caster.cast_next::<Vec<String>>()?;

                    let iterator =
                        this.iter_components(EntityTypeIdentifier(arg1))
                            .map(|components| {
                                Serialized::Array(
                                    components
                                        .into_iter()
                                        .map(|component| {
                                            Serialized::NativeObject(Box::new(component))
                                        })
                                        .collect::<Vec<_>>(),
                                )
                            });

                    Ok(Some(Serialized::Iterator(Arc::new(RwLock::new(iterator)))))
                })),
            },
            MethodInfo {
                name: "create".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = cast_introspect_ref::<EntityService>(this);

                    let mut caster = ArgumentCaster::new("create", args);
                    let arg1 = caster.cast_next::<String>()?;
                    let arg2 = caster.cast_next::<Vec<AnyComponent>>()?;
                    let id = this.create(&arg1, arg2);

                    Ok(Some(id.fruity_into()))
                })),
            },
            MethodInfo {
                name: "remove".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = cast_introspect_ref::<EntityService>(this);

                    let mut caster = ArgumentCaster::new("remove", args);
                    let arg1 = caster.cast_next::<EntityId>()?;
                    let result = this.remove(arg1);
                    if let Err(_) = result {
                        log::error!(
                            "Trying to delete an unregistered entity with entity id {:?}",
                            arg1
                        );
                    }

                    Ok(None)
                })),
            },
        ]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for EntityService {}

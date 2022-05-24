use crate::component::component::AnyComponent;
use crate::entity::archetype::Archetype;
use crate::entity::entity::get_type_identifier_by_any;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_query::serialized::SerializedQuery;
use crate::entity::entity_query::Query;
use crate::entity::entity_query::QueryParam;
use std::marker::PhantomData;
// use crate::entity::entity_query_inject::QueryInject;
use crate::entity::entity_reference::EntityReference;
use crate::ResourceContainer;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::serialize::yaml::deserialize_yaml;
use fruity_core::serialize::Deserialize;
use fruity_core::serialize::Serialize;
use fruity_core::signal::Signal;
use fruity_core::utils::introspect::cast_introspect_ref;
use fruity_core::utils::introspect::ArgumentCaster;
use maplit::hashmap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

/// An error over entity deletion
pub enum RemoveEntityError {
    /// The entity don't exists in any archetype storage
    NotFound,
}

/// A storage for every entities, use [’Archetypes’] to store entities of different types
#[derive(FruityAny)]
pub struct EntityService {
    id_incrementer: Mutex<u64>,
    index_map: RwLock<HashMap<EntityId, (usize, usize)>>,
    archetypes: Arc<RwLock<Vec<Arc<Archetype>>>>,
    object_factory_service: ResourceReference<ObjectFactoryService>,

    /// Signal notified when an entity is deleted
    pub on_deleted: Signal<EntityId>,
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
            id_incrementer: Mutex::new(0),
            index_map: RwLock::new(HashMap::new()),
            archetypes: Arc::new(RwLock::new(Vec::new())),
            object_factory_service: resource_container.require::<ObjectFactoryService>(),
            on_deleted: Signal::new(),
        }
    }

    /// Get an entity specific components
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `component_identifier` - The component identifiers
    ///
    pub fn get_entity(&self, entity_id: EntityId) -> Option<EntityReference> {
        let index_map = self.index_map.read().unwrap();
        index_map
            .get(&entity_id)
            .map(|(archetype_index, entity_id)| {
                let archetypes = self.archetypes.read().unwrap();
                archetypes[*archetype_index].clone().get(*entity_id)
            })
    }

    /// Iterate over all entities
    pub fn iter_all_entities(&self) -> impl Iterator<Item = EntityReference> + '_ {
        let archetypes = self.archetypes.read().unwrap();
        let archetypes = unsafe {
            std::mem::transmute::<&Vec<Arc<Archetype>>, &Vec<Arc<Archetype>>>(&archetypes)
        };

        archetypes
            .iter()
            .map(|archetype| {
                let archetype = archetype.clone();
                archetype.iter()
            })
            .flatten()
    }

    /// Create a query over entities
    ///
    /// # Arguments
    /// * `entity_identifier` - The entity type identifier
    /// * `callback` - The closure to execute
    ///
    pub fn query<'a, T: QueryParam<'a> + 'static>(&self) -> Query<T> {
        Query::<T> {
            archetypes: self.archetypes.clone(),
            _param_phantom: PhantomData {},
        }
    }

    /// Add a new entity in the storage
    /// Create the archetype if it don't exists
    /// Returns the newly created entity id
    ///
    /// # Arguments
    /// * `name` - The name of the entity
    /// * `enabled` - Is the entity active
    /// * `components` - The components that will be added
    ///
    pub fn create(&self, name: &str, enabled: bool, components: Vec<AnyComponent>) -> EntityId {
        let entity_id = {
            let mut id_incrementer = self.id_incrementer.lock().unwrap();
            *id_incrementer += 1;
            *id_incrementer
        };

        self.create_with_id(entity_id, name, enabled, components)
    }

    /// Add a new entity in the storage
    /// Create the archetype if it don't exists
    /// Returns the newly created entity id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `name` - The name of the entity
    /// * `enabled` - Is the entity active
    /// * `components` - The components that will be added
    ///
    pub fn create_with_id(
        &self,
        entity_id: EntityId,
        name: &str,
        enabled: bool,
        mut components: Vec<AnyComponent>,
    ) -> EntityId {
        let entity_id = {
            let mut id_incrementer = self.id_incrementer.lock().unwrap();
            *id_incrementer = u64::max(entity_id + 1, *id_incrementer);
            entity_id
        };

        components.sort_by(|a, b| a.get_class_name().cmp(&b.get_class_name()));
        let entity_identifier = get_type_identifier_by_any(&components);

        let indexes = match self.archetype_by_identifier(entity_identifier) {
            Some((archetype_index, archetype)) => {
                let archetype_entity_id = archetype.len();
                archetype.add(entity_id, name, enabled, components);

                (archetype_index, archetype_entity_id)
            }
            None => {
                let mut archetypes = self.archetypes.write().unwrap();
                let archetype_index = archetypes.len();
                let archetype = Archetype::new(entity_id, name, enabled, components);
                archetypes.push(Arc::new(archetype));
                (archetype_index, 0)
            }
        };

        let mut index_map = self.index_map.write().unwrap();
        index_map.insert(entity_id, indexes);

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
            let mut index_map = self.index_map.write().unwrap();
            index_map.remove(&entity_id)
        };

        if let Some(indexes) = indexes {
            // Delete the entity
            {
                let archetypes = self.archetypes.read().unwrap();
                let archetype = archetypes.get(indexes.0).unwrap();
                archetype.remove(indexes.1);
            }

            // Propagate the deleted signal
            self.on_deleted.notify(entity_id);

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
            let mut index_map = self.index_map.write().unwrap();
            index_map.remove(&entity_id)
        };

        if let Some(indexes) = indexes {
            let (old_entity, mut old_components) = {
                let archetypes = self.archetypes.read().unwrap();
                let archetype = archetypes.get(indexes.0).unwrap();
                archetype.remove(indexes.1)
            };

            old_components.append(&mut components);

            self.create_with_id(
                entity_id,
                &old_entity.name,
                old_entity.enabled,
                old_components,
            );

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
            let mut index_map = self.index_map.write().unwrap();
            index_map.remove(&entity_id)
        };

        if let Some(indexes) = indexes {
            let (old_entity, mut old_components) = {
                let archetypes = self.archetypes.read().unwrap();
                let archetype = archetypes.get(indexes.0).unwrap();
                archetype.remove(indexes.1)
            };

            old_components.remove(component_index);

            self.create_with_id(
                entity_id,
                &old_entity.name,
                old_entity.enabled,
                old_components,
            );

            Ok(())
        } else {
            Err(RemoveEntityError::NotFound)
        }
    }

    fn archetype_by_identifier(
        &self,
        entity_identifier: EntityTypeIdentifier,
    ) -> Option<(usize, &Archetype)> {
        let archetypes = self.archetypes.read().unwrap();
        archetypes
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
        let mut index_map = self.index_map.write().unwrap();
        let mut id_incrementer = self.id_incrementer.lock().unwrap();
        let mut archetypes = self.archetypes.write().unwrap();

        index_map.clear();
        *id_incrementer = 0;
        archetypes.clear();
    }

    /// Create a snapshot over all the entities
    pub fn snapshot(&self) -> EntityServiceSnapshot {
        let serialized_entities = self
            .iter_all_entities()
            .filter_map(|entity| {
                let entity = entity.read();
                let serialized_components = Serialized::Array(
                    entity
                        .read_all_components()
                        .into_iter()
                        .filter_map(|component| component.deref().serialize())
                        .collect::<Vec<_>>(),
                );

                let serialized_entity = Serialized::SerializedObject {
                    class_name: "Entity".to_string(),
                    fields: hashmap! {
                        "entity_id".to_string() => Serialized::U64(entity.get_entity_id()),
                        "name".to_string() => Serialized::String(entity.get_name()),
                        "enabled".to_string() => Serialized::Bool(entity.is_enabled()),
                        "components".to_string() => serialized_components,
                    },
                };

                Some(serialized_entity)
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

        if let Serialized::SerializedObject { fields, .. } = serialized_entity {
            let entity_id =
                if let Ok(entity_id) = EntityId::fruity_try_from(fields.get("entity_id")) {
                    entity_id
                } else {
                    return;
                };

            let name = if let Ok(name) = String::fruity_try_from(fields.get("name")) {
                name
            } else {
                return;
            };

            let enabled = if let Ok(enabled) = bool::fruity_try_from(fields.get("enabled")) {
                enabled
            } else {
                return;
            };

            let components = if let Some(Serialized::Array(components)) = fields.get("components") {
                components
                    .iter()
                    .filter_map(|serialized_component| {
                        AnyComponent::deserialize(serialized_component, &object_factory_service)
                    })
                    .collect::<Vec<_>>()
            } else {
                return;
            };

            self.create_with_id(entity_id, &name, enabled, components);
        }
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

                    let result = this.get_entity(arg1);
                    Ok(Some(result.fruity_into()))
                })),
            },
            MethodInfo {
                name: "query".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, _args| {
                    let this = cast_introspect_ref::<EntityService>(this);

                    let query = SerializedQuery {
                        archetypes: this.archetypes.clone(),
                        params: vec![],
                    };

                    Ok(Some(query.fruity_into()))
                })),
            },
            MethodInfo {
                name: "create".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = cast_introspect_ref::<EntityService>(this);

                    let mut caster = ArgumentCaster::new("create", args);
                    let arg1 = caster.cast_next::<String>()?;
                    let arg2 = caster.cast_next::<bool>()?;
                    let arg3 = caster.cast_next::<Vec<AnyComponent>>()?;
                    let id = this.create(&arg1, arg2, arg3);

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
        vec![FieldInfo {
            name: "on_deleted".to_string(),
            serializable: false,
            getter: Arc::new(|this| {
                this.downcast_ref::<EntityService>()
                    .unwrap()
                    .on_deleted
                    .clone()
                    .fruity_into()
            }),
            setter: SetterCaller::None,
        }]
    }
}

impl Resource for EntityService {}

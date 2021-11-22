use crate::component::component::AnyComponent;
use crate::component::component_list_rwlock::ComponentListRwLock;
use crate::entity::archetype::rwlock::EntitySharedRwLock;
use crate::entity::archetype::Archetype;
use crate::entity::entity::get_type_identifier_by_any;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_query::EntityQueryReadCallback;
use crate::entity::entity_query::EntityQueryWriteCallback;
use crate::ResourceContainer;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::serialize::Deserialize;
use fruity_core::serialize::Serialize;
use fruity_core::signal::Signal;
use fruity_core::utils::introspect::cast_introspect_ref;
use fruity_core::utils::introspect::ArgumentCaster;
use maplit::hashmap;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::any::TypeId;
use std::sync::Arc;
use std::sync::RwLock;

/// An error over entity deletion
pub enum RemoveEntityError {
    /// The entity don't exists in any archetype storage
    NotFound,
}

#[derive(Debug)]
struct InnerEntityService {
    id_incrementer: u64,
    archetypes: Vec<Archetype>,
}

/// A storage for every entities, use [’Archetypes’] to store entities of different types
#[derive(Debug, FruityAny)]
pub struct EntityService {
    inner: Arc<RwLock<InnerEntityService>>,
    object_factory_service: ResourceReference<ObjectFactoryService>,

    /// Signal propagated when a new entity is inserted into the collection
    pub on_entity_created: Signal<EntitySharedRwLock>,

    /// Signal propagated when a new entity is removed from the collection
    pub on_entity_removed: Signal<EntityId>,
}

/// A save for the entities stored in an [’EntityService’]
#[derive(Clone, Debug)]
pub struct EntityServiceSnapshot(pub Serialized);

impl EntityService {
    /// Returns an EntityService
    pub fn new(resource_container: Arc<ResourceContainer>) -> EntityService {
        EntityService {
            inner: Arc::new(RwLock::new(InnerEntityService {
                id_incrementer: 0,
                archetypes: Vec::new(),
            })),
            object_factory_service: resource_container.require::<ObjectFactoryService>(),
            on_entity_created: Signal::new(),
            on_entity_removed: Signal::new(),
        }
    }

    /// Get a specific entity by it's id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get_entity(&self, entity_id: EntityId) -> Option<EntitySharedRwLock> {
        self.iter_all_entities().find(|entity| {
            let entity = entity.read();
            entity.entity_id == entity_id
        })
    }

    /// Get a locked entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get(&self, entity_id: EntityId) -> Option<EntitySharedRwLock> {
        let inner = self.inner.read().unwrap();
        inner
            .archetypes
            .iter()
            .find_map(|archetype| archetype.get(entity_id))
    }

    /// Iterate over all entities
    pub fn iter_all_entities(&self) -> impl Iterator<Item = EntitySharedRwLock> {
        let inner = self.inner.read().unwrap();
        let archetypes = unsafe { &*(&inner.archetypes as *const _) } as &Vec<Archetype>;
        archetypes
            .iter()
            .map(|archetype| archetype.iter())
            .flatten()
    }

    /// Iterate over all entities with a specific archetype type
    /// Use every entity that contains the provided entity type
    ///
    /// # Arguments
    /// * `entity_identifier` - The entity type identifier
    ///
    pub fn iter_entities(
        &self,
        entity_identifier: EntityTypeIdentifier,
    ) -> impl Iterator<Item = EntitySharedRwLock> {
        let inner = self.inner.read().unwrap();
        let archetypes = unsafe { &*(&inner.archetypes as *const _) } as &Vec<Archetype>;
        archetypes
            .iter()
            .filter(move |archetype| archetype.get_type_identifier().contains(&entity_identifier))
            .map(|archetype| archetype.iter())
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
    pub fn for_each(
        &self,
        entity_identifier: EntityTypeIdentifier,
        callback: impl EntityQueryReadCallback,
    ) {
        self.iter_components(entity_identifier)
            .par_bridge()
            .for_each(move |components| (callback.inject_components())(components));
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
    pub fn for_each_mut(
        &self,
        entity_identifier: EntityTypeIdentifier,
        callback: impl EntityQueryWriteCallback,
    ) {
        self.iter_components(entity_identifier)
            .par_bridge()
            .for_each(move |components| (callback.inject_components())(components));
    }

    /// Iterate over all entities with a specific archetype type
    /// Use every entity that contains the provided entity type
    /// Also map components to the order of provided entity type
    /// identifier
    ///
    /// # Arguments
    /// * `entity_identifier` - The entity type identifier
    ///
    pub fn iter_components(
        &self,
        entity_identifier: EntityTypeIdentifier,
    ) -> impl Iterator<Item = ComponentListRwLock> {
        let this = unsafe { &*(self as *const _) } as &EntityService;
        this.iter_entities(entity_identifier.clone())
            .map(move |entity| {
                entity
                    .iter_components(&entity_identifier.clone())
                    .collect::<Vec<_>>()
            })
            .flatten()
    }

    /// Add a new entity in the storage
    /// Create the archetype if it don't exists
    /// Returns the newly created entity id
    ///
    /// # Arguments
    /// * `entity` - The entity that will be added
    ///
    pub fn create(&self, name: String, mut components: Vec<AnyComponent>) -> EntityId {
        let entity_id = {
            let mut inner = self.inner.write().unwrap();
            inner.id_incrementer += 1;
            inner.id_incrementer
        };

        components.sort_by(|a, b| a.get_class_name().cmp(&b.get_class_name()));
        let entity_identifier = get_type_identifier_by_any(&components);

        match self.archetype_by_identifier(entity_identifier) {
            Some(archetype) => {
                archetype.add(entity_id, name, components);
            }
            None => {
                let mut inner = self.inner.write().unwrap();
                let archetype = Archetype::new(entity_id, name, components);
                inner.archetypes.push(archetype);
            }
        }

        self.on_entity_created.notify(self.get(entity_id).unwrap());
        entity_id
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn remove(&self, entity_id: EntityId) {
        let inner = self.inner.read().unwrap();
        if let Some(_) =
            inner
                .archetypes
                .iter()
                .find_map(|archetype| match archetype.remove(entity_id) {
                    Ok(entity) => Some(entity),
                    Err(err) => match err {
                        RemoveEntityError::NotFound => None,
                    },
                })
        {
            self.on_entity_removed.notify(entity_id);
        } else {
            log::error!(
                "Trying to delete an unregistered entity with entity id {:?}",
                entity_id
            );
        }
    }

    fn archetype_by_identifier(
        &self,
        entity_identifier: EntityTypeIdentifier,
    ) -> Option<&Archetype> {
        let inner = self.inner.read().unwrap();

        let result = inner
            .archetypes
            .iter()
            .find(|archetype| *archetype.get_type_identifier() == entity_identifier);

        unsafe { std::mem::transmute::<Option<&Archetype>, Option<&Archetype>>(result) }
    }

    /// Clear all the entities
    pub fn clear(&self) {
        let inner = self.inner.read().unwrap();

        // Propagate all entity removed events
        inner.archetypes.iter().for_each(|archetype| {
            archetype.iter().for_each(|entity| {
                let entity = entity.read();
                self.on_entity_removed.notify(entity.entity_id);
            })
        });
        std::mem::drop(inner);

        // Clear all entities
        let mut inner = self.inner.write().unwrap();
        inner.id_incrementer = 0;
        inner.archetypes.clear();
    }

    /// Create a snapshot over all the entities
    pub fn snapshot(&self) -> EntityServiceSnapshot {
        let serialized_entities = self
            .iter_all_entities()
            .filter_map(|entity| {
                let reader = entity.read();
                let serialized_components = Serialized::Array(
                    entity
                        .iter_all_components()
                        .filter_map(|component| component.serialize())
                        .collect::<Vec<_>>(),
                );

                let serialized_entity = Serialized::SerializedObject {
                    class_name: "Entity".to_string(),
                    fields: hashmap! {
                        "name".to_string() => Serialized::String(reader.name.clone()),
                        "enabled".to_string() => Serialized::Bool(reader.enabled),
                        "components".to_string() => serialized_components,
                    },
                };

                Some(serialized_entity)
            })
            .collect::<Vec<_>>();

        EntityServiceSnapshot(Serialized::Array(serialized_entities))
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
            let name = if let Some(Serialized::String(name)) = fields.get("name") {
                name
            } else {
                return;
            };

            let enabled = if let Some(Serialized::Bool(enabled)) = fields.get("enabled") {
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

            let entity_id = self.create(name.clone(), components);
            let entity = self.get_entity(entity_id).unwrap();
            let mut entity = entity.write();
            entity.enabled = *enabled;
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

                    let result = this
                        .get_entity(arg1)
                        .map(|entity| Serialized::NativeObject(Box::new(entity)));

                    Ok(result)
                })),
            },
            MethodInfo {
                name: "iter_entities".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = cast_introspect_ref::<EntityService>(this);

                    let mut caster = ArgumentCaster::new("iter_entities", args);
                    let arg1 = caster.cast_next::<Vec<String>>()?;

                    let iterator = this
                        .iter_entities(EntityTypeIdentifier(arg1))
                        .map(|entity| Serialized::NativeObject(Box::new(entity)));

                    Ok(Some(Serialized::Iterator(Arc::new(RwLock::new(iterator)))))
                })),
            },
            MethodInfo {
                name: "iter_components".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = cast_introspect_ref::<EntityService>(this);

                    let mut caster = ArgumentCaster::new("iter_components", args);
                    let arg1 = caster.cast_next::<Vec<String>>()?;

                    let iterator = this
                        .iter_components(EntityTypeIdentifier(arg1))
                        .map(|component| Serialized::NativeObject(Box::new(component)));

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
                    let id = this.create(arg1, arg2);

                    Ok(Some(id.fruity_into()))
                })),
            },
            MethodInfo {
                name: "remove".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = cast_introspect_ref::<EntityService>(this);

                    let mut caster = ArgumentCaster::new("remove", args);
                    let arg1 = caster.cast_next::<EntityId>()?;
                    this.remove(arg1);

                    Ok(None)
                })),
            },
        ]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![FieldInfo {
            name: "on_entity_created".to_string(),
            ty: TypeId::of::<Signal<EntitySharedRwLock>>(),
            serializable: false,
            getter: Arc::new(|this| {
                this.downcast_ref::<EntityService>()
                    .unwrap()
                    .on_entity_created
                    .clone()
                    .fruity_into()
            }),
            setter: SetterCaller::None,
        }]
    }
}

impl Resource for EntityService {}

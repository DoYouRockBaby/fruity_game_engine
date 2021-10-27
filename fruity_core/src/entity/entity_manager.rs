use crate::component::component::AnyComponent;
use crate::component::component_list_rwlock::ComponentListRwLock;
use crate::entity::archetype::rwlock::EntitySharedRwLock;
use crate::entity::archetype::Archetype;
use crate::entity::entity::get_type_identifier_by_any;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::service::service::Service;
use crate::service::utils::cast_service;
use crate::service::utils::cast_service_mut;
use crate::service::utils::ArgumentCaster;
use crate::signal::Signal;
use fruity_any::*;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use fruity_introspect::SetterCaller;
use std::sync::Arc;
use std::sync::RwLock;

/// An error over entity deletion
pub enum RemoveEntityError {
    /// The entity don't exists in any archetype storage
    NotFound,
}

/// A storage for every entities, use [’Archetypes’] to store entities of different types
#[derive(Debug, FruityAny)]
pub struct EntityManager {
    id_incrementer: u64,
    archetypes: Vec<Archetype>,

    /// Signal propagated when a new entity is inserted into the collection
    pub on_entity_created: Signal<EntitySharedRwLock>,

    /// Signal propagated when a new entity is removed from the collection
    pub on_entity_removed: Signal<EntityId>,
}

impl EntityManager {
    /// Returns an EntityManager
    pub fn new() -> EntityManager {
        EntityManager {
            id_incrementer: 0,
            archetypes: Vec::new(),
            on_entity_created: Signal::new(),
            on_entity_removed: Signal::new(),
        }
    }

    /// Get a locked entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get(&self, entity_id: EntityId) -> Option<EntitySharedRwLock> {
        self.archetypes
            .iter()
            .find_map(|archetype| archetype.get(entity_id))
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
        let archetypes = unsafe { &*(&self.archetypes as *const _) } as &Vec<Archetype>;
        archetypes
            .iter()
            .filter(move |archetype| archetype.get_type_identifier().contains(&entity_identifier))
            .map(|archetype| archetype.iter())
            .flatten()
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
        let this = unsafe { &*(self as *const _) } as &EntityManager;
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
    pub fn create(&mut self, components: Vec<AnyComponent>) -> EntityId {
        self.id_incrementer += 1;
        let entity_id = self.id_incrementer;
        let entity_identifier = get_type_identifier_by_any(&components);

        match self.archetype_by_identifier(entity_identifier) {
            Some(archetype) => {
                archetype.add(entity_id, components);
            }
            None => {
                let archetype = Archetype::new(entity_id, components);
                self.archetypes.push(archetype);
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
    pub fn remove(&mut self, entity_id: EntityId) {
        if let Some(_) =
            self.archetypes
                .iter_mut()
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
        self.archetypes
            .iter()
            .find(|archetype| *archetype.get_type_identifier() == entity_identifier)
    }
}

impl IntrospectObject for EntityManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "create".to_string(),
                call: MethodCaller::Mut(Arc::new(move |this, args| {
                    let this = cast_service_mut::<EntityManager>(this);

                    let mut caster = ArgumentCaster::new("create", args);
                    let arg1 = caster.cast_next::<Vec<AnyComponent>>()?;
                    this.create(arg1);

                    Ok(None)
                })),
            },
            MethodInfo {
                name: "iter_entities".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = cast_service::<EntityManager>(this);

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
                    let this = cast_service::<EntityManager>(this);

                    let mut caster = ArgumentCaster::new("iter_components", args);
                    let arg1 = caster.cast_next::<Vec<String>>()?;

                    let iterator = this
                        .iter_components(EntityTypeIdentifier(arg1))
                        .map(|component| Serialized::NativeObject(Box::new(component)));

                    Ok(Some(Serialized::Iterator(Arc::new(RwLock::new(iterator)))))
                })),
            },
        ]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![FieldInfo {
            name: "on_entity_created".to_string(),
            getter: Arc::new(|this| {
                this.downcast_ref::<EntityManager>()
                    .unwrap()
                    .on_entity_created
                    .clone()
                    .into()
            }),
            setter: SetterCaller::None,
        }]
    }
}

impl Service for EntityManager {}

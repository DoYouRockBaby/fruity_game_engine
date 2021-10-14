use crate::component::component_list_rwlock::ComponentListRwLock;
use crate::entity::archetype::Archetype;
use crate::entity::entity::Entity;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity::Iter as EntityIter;
use crate::entity::entity::IterMut as EntityIterMut;
use crate::entity::entity_rwlock::EntityRwLock;
use crate::serialize::serialized::Serialized;
use crate::service::service::Service;
use crate::service::utils::assert_argument_count;
use crate::service::utils::cast_argument;
use crate::service::utils::cast_service;
use crate::ServiceManager;
use crate::World;
use fruity_any_derive::*;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use rayon::prelude::*;
use std::any::Any;
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
    service_manager: Arc<RwLock<ServiceManager>>,
}

impl EntityManager {
    /// Returns an EntityManager
    pub fn new(world: &World) -> EntityManager {
        EntityManager {
            id_incrementer: 0,
            archetypes: Vec::new(),
            service_manager: world.service_manager.clone(),
        }
    }

    /// Get a locked entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get(&self, entity_id: EntityId) -> Option<&EntityRwLock> {
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
    ) -> impl Iterator<Item = &EntityRwLock> {
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
            .filter_map(move |entity| entity.iter_components(&entity_identifier.clone()).ok())
            .flatten()
    }

    /// Iterate over all entities with a specific archetype type
    /// Use every entity that contains the provided entity type
    ///
    /// # Arguments
    /// * `entity_identifier` - The entity type identifier
    /// * `callback` - The callback that will be called for every entity
    ///
    pub fn for_each_entity<F: Fn(EntityRwLock) + Sync + Send>(
        &self,
        entity_identifier: EntityTypeIdentifier,
        callback: F,
    ) {
        self.iter_entities(entity_identifier)
            .par_bridge()
            .for_each(move |entity| {
                callback(entity.clone());
            });
    }

    /// Iterate over all entities with a specific archetype type
    /// Use every entity that contains the provided entity type
    /// Also map components to the order of provided entity type
    /// identifier
    ///
    /// # Arguments
    /// * `entity_identifier` - The entity type identifier
    ///
    pub fn for_each<F: Fn(EntityIter) + Sync + Send>(
        &self,
        entity_identifier: EntityTypeIdentifier,
        callback: F,
    ) {
        self.iter_entities(entity_identifier.clone())
            .par_bridge()
            .for_each(move |entity| {
                entity
                    .read()
                    .unwrap()
                    .iter_component_tuple(&entity_identifier)
                    .for_each(|components| callback(components));
            });
    }

    /// Iterate over all entities with a specific archetype type with mutability
    /// Use every entity that contains the provided entity type
    /// Also map components to the order of provided entity type
    /// identifier
    ///
    /// # Arguments
    /// * `entity_identifier` - The entity type identifier
    ///
    pub fn for_each_mut<F: Fn(EntityIterMut) + Sync + Send>(
        &self,
        entity_identifier: EntityTypeIdentifier,
        callback: F,
    ) {
        self.iter_entities(entity_identifier.clone())
            .par_bridge()
            .for_each(move |entity| {
                entity
                    .write()
                    .unwrap()
                    .iter_mut_component_tuple(&entity_identifier)
                    .for_each(|components| callback(components));
            });
    }

    /// Add a new entity in the storage
    /// Create the archetype if it don't exists
    /// Returns the newly created entity id
    ///
    /// # Arguments
    /// * `entity` - The entity that will be added
    ///
    pub fn create(&mut self, entity: Entity) -> EntityId {
        self.id_incrementer += 1;
        let entity_id = EntityId(self.id_incrementer);
        let entity_identifier = entity.get_type_identifier();

        match self.archetype_mut_by_identifier(entity_identifier) {
            Some(archetype) => {
                archetype.add(entity_id, entity);
                entity_id
            }
            None => {
                let archetype = Archetype::new(entity_id, entity);
                self.archetypes.push(archetype);
                entity_id
            }
        }
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn remove(&mut self, entity_id: EntityId) {
        if !self
            .archetypes
            .iter_mut()
            .any(|archetype| match archetype.remove(entity_id) {
                Ok(()) => true,
                Err(err) => match err {
                    RemoveEntityError::NotFound => false,
                },
            })
        {
            log::error!(
                "Trying to delete an unregistered entity with entity id {:?}",
                entity_id
            );
        }
    }

    #[allow(dead_code)]
    fn archetype_by_identifier(
        &self,
        entity_identifier: EntityTypeIdentifier,
    ) -> Option<&Archetype> {
        self.archetypes
            .iter()
            .find(|archetype| *archetype.get_type_identifier() == entity_identifier)
    }

    fn archetype_mut_by_identifier(
        &mut self,
        entity_identifier: EntityTypeIdentifier,
    ) -> Option<&mut Archetype> {
        self.archetypes
            .iter_mut()
            .find(|archetype| *archetype.get_type_identifier() == entity_identifier)
    }
}

impl IntrospectMethods<Serialized> for EntityManager {
    fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
        vec![
            MethodInfo {
                name: "iter_entities".to_string(),
                args: vec!["[String]".to_string()],
                return_type: None,
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = unsafe { &*(this as *const _) } as &dyn Any;
                    let this = cast_service::<EntityManager>(this);
                    assert_argument_count("iter_entities", 1, &args)?;

                    let arg1 =
                        cast_argument("iter_entities", 0, &args, |arg| arg.as_string_array())?;

                    let iterator = this
                        .iter_entities(EntityTypeIdentifier(arg1))
                        .map(|entity| Serialized::Entity(entity.clone()));

                    Ok(Some(Serialized::Iterator(Arc::new(RwLock::new(iterator)))))
                })),
            },
            MethodInfo {
                name: "iter_components".to_string(),
                args: vec!["[String]".to_string()],
                return_type: None,
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = cast_service::<EntityManager>(this);
                    assert_argument_count("iter_components", 1, &args)?;

                    let arg1 =
                        cast_argument("iter_components", 0, &args, |arg| arg.as_string_array())?;

                    let iterator = this
                        .iter_components(EntityTypeIdentifier(arg1))
                        .map(|components| Serialized::ComponentList(components));

                    Ok(Some(Serialized::Iterator(Arc::new(RwLock::new(iterator)))))
                })),
            },
        ]
    }
}

impl Service for EntityManager {}

use crate::component::component::AnyComponent;
use crate::component::component_reference::ComponentReference;
use crate::entity::archetype::component_collection::ComponentCollection;
use crate::entity::archetype::EntityReference;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Clone)]
pub(crate) struct ComponentStorage {
    pub(crate) collection: Arc<RwLock<Box<dyn ComponentCollection>>>,
    pub(crate) components_per_entity: usize,
}

impl ComponentStorage {
    pub(crate) fn new(components: Vec<AnyComponent>) -> Self {
        let components_per_entity = components.len();
        let first_component = components.get(0).unwrap();
        let mut collection = first_component.get_collection();
        collection.add_many(components);

        ComponentStorage {
            collection: Arc::new(RwLock::new(collection)),
            components_per_entity,
        }
    }

    /// Get components of an entity
    ///
    /// # Arguments
    /// * `entity_reference` - The entity reference
    ///
    pub(crate) fn get_entity_components(
        &self,
        entity_reference: EntityReference,
    ) -> impl Iterator<Item = ComponentReference> + '_ {
        let start_index = entity_reference.entity_id * self.components_per_entity;
        let end_index = start_index + self.components_per_entity;

        (start_index..end_index)
            .into_iter()
            .map(move |index| ComponentReference {
                entity_reference: entity_reference.clone(),
                collection: self.collection.clone(),
                component_index: index,
            })
    }

    pub(crate) fn add(&self, components: Vec<AnyComponent>) {
        // Check the components count
        if components.len() != self.components_per_entity {
            panic!("Try to instantiate a component array from a component array with the wrong size of elements");
        }

        let mut collection_writer = self.collection.write().unwrap();
        collection_writer.add_many(components);
    }

    pub(crate) fn remove(&self, entity_id: usize) -> Vec<AnyComponent> {
        let index = entity_id * self.components_per_entity;

        let mut collection_writer = self.collection.write().unwrap();
        collection_writer.remove_many(index, self.components_per_entity)
    }
}

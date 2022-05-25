use crate::component::component::AnyComponent;
use crate::entity::archetype::component_collection::ComponentCollection;

pub(crate) struct ComponentStorage {
    pub(crate) collection: Box<dyn ComponentCollection>,
    pub(crate) components_per_entity: usize,
}

impl ComponentStorage {
    pub(crate) fn new(components: Vec<AnyComponent>) -> Self {
        let components_per_entity = components.len();
        let first_component = components.get(0).unwrap();
        let mut collection = first_component.get_collection();
        collection.add_many(components);

        ComponentStorage {
            collection,
            components_per_entity,
        }
    }

    pub(crate) fn add(&mut self, components: Vec<AnyComponent>) {
        // Check the components count
        if components.len() != self.components_per_entity {
            panic!("Try to instantiate a component array from a component array with the wrong size of elements");
        }

        self.collection.add_many(components);
    }

    pub(crate) fn remove(&mut self, entity_id: usize) -> Vec<AnyComponent> {
        let index = entity_id * self.components_per_entity;
        self.collection
            .remove_many(index, self.components_per_entity)
    }
}

use crate::component::component::AnyComponent;
use crate::entity::archetype::component_collection::ComponentCollection;
use crate::entity::archetype::Component;

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

    pub(crate) fn get(&self, entity_id: usize) -> impl Iterator<Item = &dyn Component> {
        let start_index = entity_id * self.components_per_entity;
        let end_index = start_index + self.components_per_entity;

        (start_index..end_index).filter_map(|index| self.collection.get(&index))
    }
}

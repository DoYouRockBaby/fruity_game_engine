use crate::entity::archetype::component_collection::ComponentCollection;
use crate::entity::archetype::AnyComponent;
use crate::entity::archetype::Component;

/// A collection of entities that share the same component structure
/// Can store multiple components of the same type
pub struct ComponentArray<T: Component> {
    components: Vec<T>,
    components_per_entity: usize,
}

impl<T: Component> ComponentArray<T> {
    pub fn new(components_per_entity: usize) -> Self {
        Self {
            components: Vec::new(),
            components_per_entity,
        }
    }
}

impl<T: Component> ComponentCollection for ComponentArray<T> {
    fn get(&self, index: &usize) -> Vec<&dyn Component> {
        let start_index = index * self.components_per_entity;
        let end_index = start_index + self.components_per_entity;

        self.components[start_index..end_index]
            .iter()
            .map(|component| component as &dyn Component)
            .collect::<Vec<_>>()
    }

    fn add(&mut self, components: Vec<AnyComponent>) {
        // Check the components count
        if components.len() != self.components_per_entity {
            panic!("Try to instantiate a component array from a component array with the wrong size of elements");
        }

        // Check that all the components have the good type and convert it to an array of typed component
        let mut components = components.into_iter().map(|component| match component.into_box().as_any_box().downcast::<T>() {
            Ok(component) => *component,
            Err(_) => {
                panic!("Try to instantiate a component array from a array of components with wrong type");
            }
        }).collect::<Vec<_>>();

        self.components.append(&mut components);
    }

    fn remove(&mut self, index: usize) -> Vec<AnyComponent> {
        let start_index = index * self.components_per_entity;
        let end_index = start_index + self.components_per_entity;

        let components = self.components.drain(start_index..end_index);
        components
            .into_iter()
            .map(|component| AnyComponent::new(component))
            .collect::<Vec<_>>()
    }

    fn get_components_per_entity(&self) -> usize {
        self.components_per_entity
    }
}

use crate::entity::archetype::component_collection::ComponentCollection;
use crate::entity::archetype::AnyComponent;
use crate::entity::archetype::Component;

/// A collection of entities that share the same component structure
/// Can store multiple components of the same type
pub struct ComponentArray<T: Component> {
    components: Vec<T>,
}

impl<T: Component> ComponentArray<T> {
    /// Returns a ComponentArray
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
}

impl<T: Component> ComponentCollection for ComponentArray<T> {
    fn get(&self, index: &usize) -> Option<&dyn Component> {
        self.components
            .get(*index)
            .map(|component| component as &dyn Component)
    }

    fn add_many(&mut self, components: Vec<AnyComponent>) {
        // Check that all the components have the good type and convert it to an array of typed component
        let mut components = components.into_iter().map(|component| match component.into_box().as_any_box().downcast::<T>() {
            Ok(component) => *component,
            Err(_) => {
                panic!("Try to instantiate a component array from a array of components with wrong type");
            }
        }).collect::<Vec<_>>();

        self.components.append(&mut components);
    }

    fn remove_many(&mut self, index: usize, count: usize) -> Vec<AnyComponent> {
        let end_index = index + count;

        let components = self.components.drain(index..end_index);
        components
            .into_iter()
            .map(|component| AnyComponent::new(component))
            .collect::<Vec<_>>()
    }
}

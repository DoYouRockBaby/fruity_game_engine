use crate::entity::archetype::AnyComponent;
use crate::entity::archetype::Component;
use crate::entity::archetype::ComponentDecoder;

/// A collection of entities that share the same component structure
/// Can store multiple components of the same type
pub struct ComponentArray {
    buffer: Vec<u8>,
    component_count: usize,
    component_encode_size: usize,
    decoder: ComponentDecoder,
}

impl ComponentArray {
    pub(crate) fn new(components: Vec<AnyComponent>) -> Self {
        // Check that components is not empty
        if components.len() == 0 {
            panic!("Try to instantiate a component array from an empty component array");
        }
        let first_component = components.first().unwrap();

        // Check that all the components share the same memory size
        if components.len() == 0 {
            for component in components {
                if component.encode_size() != first_component.encode_size() {
                    panic!("Try to instantiate a component array from a array of components with different size");
                }
            }
        }

        // Instantiate the array
        let mut result = Self {
            buffer: Vec::with_capacity(first_component.encode_size() * components.len()),
            component_count: components.len(),
            component_encode_size: first_component.encode_size(),
            decoder: first_component.get_decoder(),
        };

        // Insert the first components
        result.add(components);

        result
    }

    pub(crate) fn get(&self, index: &usize) -> Vec<&dyn Component> {
        let start_buffer = index * (self.component_encode_size * self.component_count);

        (0..self.component_count)
            .map(|component_index| {
                let start_buffer = start_buffer + component_index * self.component_encode_size;
                let end_buffer = start_buffer + self.component_encode_size;
                let component_buffer = &self.buffer[start_buffer..end_buffer];

                (self.decoder)(&component_buffer)
            })
            .collect::<Vec<_>>()
    }

    pub(crate) fn add(&mut self, components: Vec<AnyComponent>) {
        // Check the components count
        if components.len() == self.component_count {
            panic!("Try to instantiate a component array from a component array with the wrong size of elements");
        }

        // Check that every components have the good memory size
        for component in components {
            if component.encode_size() != self.component_encode_size {
                panic!("Try to insert into a component array from a array of components with different size");
            }
        }

        let mut new_component_buffer =
            Vec::<u8>::with_capacity(self.component_encode_size * self.component_count);
        new_component_buffer.resize(self.component_encode_size * self.component_count, 0);

        components
            .into_iter()
            .enumerate()
            .for_each(|(component_index, component)| {
                let start_buffer = component_index * self.component_encode_size;
                let end_buffer = start_buffer + self.component_encode_size;
                component.encode(&mut new_component_buffer[start_buffer..end_buffer]);

                // TODO: Release the memory on drain
                std::mem::forget(component);
            });

        self.buffer.append(&mut new_component_buffer);
    }

    pub(crate) fn remove(&mut self, index: usize) -> Vec<AnyComponent> {
        let start_buffer = index * (self.component_encode_size * self.component_count);

        (0..self.component_count)
            .map(|component_index| {
                let start_buffer = start_buffer + component_index * self.component_encode_size;
                let end_buffer = start_buffer + self.component_encode_size;

                let component_buffer = self
                    .buffer
                    .drain(start_buffer..end_buffer)
                    .collect::<Vec<_>>();

                let component = (self.decoder)(&component_buffer).duplicate();
                AnyComponent::from_box(component)
            })
            .collect::<Vec<_>>()
    }

    pub(crate) fn get_component_count(&self) -> usize {
        self.component_count
    }
}

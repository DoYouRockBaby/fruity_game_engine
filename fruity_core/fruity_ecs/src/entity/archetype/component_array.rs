use crate::entity::archetype::AnyComponent;
use crate::entity::archetype::Component;
use crate::entity::archetype::ComponentDecoder;

/// A collection of entities that share the same component structure
pub struct ComponentArray {
    buffer: Vec<u8>,
    component_encode_size: usize,
    decoder: ComponentDecoder,
}

impl ComponentArray {
    pub(crate) fn new(first_component: AnyComponent) -> Self {
        let mut result = Self {
            buffer: Vec::with_capacity(first_component.encode_size()),
            component_encode_size: first_component.encode_size(),
            decoder: first_component.get_decoder(),
        };
        result.add(first_component);

        result
    }

    pub(crate) fn get(&self, index: &usize) -> &dyn Component {
        let buffer_start = index * self.component_encode_size;
        let buffer_end = buffer_start + self.component_encode_size;
        let component_buffer = &self.buffer[buffer_start..buffer_end];

        (self.decoder)(&component_buffer)
    }

    pub(crate) fn add(&mut self, component: AnyComponent) {
        let mut new_component_buffer = Vec::<u8>::with_capacity(self.component_encode_size);
        new_component_buffer.resize(self.component_encode_size, 0);

        component.encode(&mut new_component_buffer);
        self.buffer.append(&mut new_component_buffer);

        // TODO: Release the memory on drain
        std::mem::forget(component);
    }

    pub(crate) fn remove(&mut self, index: usize) -> AnyComponent {
        let start_buffer = index * self.component_encode_size;
        let end_buffer = start_buffer + self.component_encode_size;

        let component_buffer = self
            .buffer
            .drain(start_buffer..end_buffer)
            .collect::<Vec<_>>();

        let component = (self.decoder)(&component_buffer).duplicate();
        AnyComponent::from_box(component)
    }
}

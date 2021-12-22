use crate::component::component_reference::ComponentReference;
use crate::entity::archetype::AnyComponent;
use crate::entity::archetype::Component;
use crate::entity::archetype::ComponentDecoder;
use fruity_core::utils::slice::copy;
use std::mem::size_of;
use std::sync::RwLock;

/// A collection of entities that share the same component structure
pub struct ComponentArray {
    buffer: Vec<u8>,
    component_encode_size: usize,
    decoder: ComponentDecoder,
}

impl ComponentArray {
    pub(crate) fn new(first_component: AnyComponent) -> Self {
        let mut result = Self {
            buffer: Vec::with_capacity(size_of::<RwLock<()>>() + first_component.encode_size()),
            component_encode_size: first_component.encode_size(),
            decoder: first_component.get_decoder(),
        };
        result.add(first_component);

        result
    }

    pub(crate) fn get(&self, index: &usize) -> ComponentReference {
        let buffer_index = index * self.component_cell_size();
        ComponentReference::new(
            self.get_rwlock_reference(buffer_index),
            self.get_component_data_reference(buffer_index),
        )
    }

    pub(crate) fn len(&self) -> usize {
        self.buffer.len() / self.component_encode_size
    }

    pub(crate) fn add(&mut self, component: AnyComponent) {
        let mut new_component_buffer = Vec::<u8>::with_capacity(self.component_cell_size());
        new_component_buffer.resize(self.component_cell_size(), 0);

        Self::encode_rwlock_reference(&mut new_component_buffer);
        component.encode(&mut new_component_buffer[size_of::<RwLock<()>>()..]);
        self.buffer.append(&mut new_component_buffer);

        // TODO: Release the memory on drain
        std::mem::forget(component);
    }

    pub(crate) fn replace(&mut self, index: usize, component: AnyComponent) {
        /*let mut new_component_buffer = Vec::<u8>::with_capacity(self.component_cell_size());
        new_component_buffer.resize(self.component_cell_size(), 0);*/

        let start_buffer = index * self.component_cell_size();
        let end_buffer = start_buffer + self.component_cell_size();

        let mut component_buffer = &mut self.buffer[start_buffer..end_buffer];
        Self::encode_rwlock_reference(&mut component_buffer);
        component.encode(&mut component_buffer[size_of::<RwLock<()>>()..]);

        // TODO: Release the memory on drain
        std::mem::forget(component);
    }

    fn encode_rwlock_reference(mut buffer: &mut [u8]) {
        let rwlock = RwLock::<()>::new(());
        let encoded_rwlock = unsafe {
            std::slice::from_raw_parts(
                (&*&rwlock as *const RwLock<()>) as *const u8,
                size_of::<RwLock<()>>(),
            )
        };

        copy(&mut buffer, encoded_rwlock);

        // TODO: Release the memory on drain
        std::mem::forget(rwlock);
    }

    fn get_component_data_reference(&self, buffer_index: usize) -> &dyn Component {
        let buffer_start = buffer_index + size_of::<RwLock<()>>();
        let buffer_end = buffer_start + self.component_encode_size;
        let component_data_buffer = &self.buffer[buffer_start..buffer_end];
        (self.decoder)(component_data_buffer)
    }

    fn get_rwlock_reference(&self, buffer_start: usize) -> &RwLock<()> {
        let buffer_end = buffer_start + size_of::<RwLock<()>>();
        let rwlock_buffer = &self.buffer[buffer_start..buffer_end];
        let (_head, body, _tail) = unsafe { rwlock_buffer.align_to::<RwLock<()>>() };
        &body[0]
    }

    fn component_cell_size(&self) -> usize {
        size_of::<RwLock<()>>() + self.component_encode_size
    }
}

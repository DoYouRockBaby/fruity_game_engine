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
        let buffer_start = index * self.component_cell_size();
        let buffer_end = buffer_start + self.component_cell_size();
        let component_buffer = &self.buffer[buffer_start..buffer_end];

        ComponentReference::new(
            self.get_rwlock_reference(component_buffer),
            self.get_component_data_reference(component_buffer),
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

    pub(crate) fn remove(&mut self, index: usize) -> AnyComponent {
        let start_buffer = index * self.component_cell_size();
        let end_buffer = start_buffer + self.component_cell_size();

        let component_buffer = self
            .buffer
            .drain(start_buffer..end_buffer)
            .collect::<Vec<_>>();

        let component = self
            .get_component_data_reference(&component_buffer)
            .duplicate();

        AnyComponent::from_box(component)
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

    fn get_rwlock_reference<'a>(&self, component_buffer: &'a [u8]) -> &'a RwLock<()> {
        let buffer_end = size_of::<RwLock<()>>();
        let rwlock_buffer = &component_buffer[0..buffer_end];
        let (_head, body, _tail) = unsafe { rwlock_buffer.align_to::<RwLock<()>>() };
        &body[0]
    }

    fn get_component_data_reference<'a>(&self, component_buffer: &'a [u8]) -> &'a dyn Component {
        let buffer_start = size_of::<RwLock<()>>();
        let component_data_buffer = &component_buffer[buffer_start..];
        (self.decoder)(component_data_buffer)
    }

    fn component_cell_size(&self) -> usize {
        size_of::<RwLock<()>>() + self.component_encode_size
    }
}

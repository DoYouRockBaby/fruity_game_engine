use crate::entity::archetype::AnyComponent;
use crate::entity::archetype::Component;
use crate::entity::archetype::ComponentDecodingInfos;
use crate::entity::archetype::EntityCellHead;
use crate::entity::archetype::EntityId;
use fruity_core::utils::slice::copy;
use std::mem::size_of;

// TODO: There is a lot of optimizations to do inside this

/// Get the size of an entity
///
/// # Arguments
/// * `components` - The list of the entity components
///
pub(crate) fn entity_size(components: &[AnyComponent]) -> usize {
    let components_per_entity = components.len();

    let all_components_size: usize = components
        .iter()
        .map(|component| component.encode_size())
        .sum();

    let entity_size = size_of::<EntityCellHead>()
        + components_per_entity * size_of::<ComponentDecodingInfos>()
        + all_components_size;

    entity_size
}

/// Encode a new entity into a specified buffer
///
/// # Arguments
/// * `entity_id` - The entity id
/// * `entity_buffer` - The buffer where the entity will be encoded
/// * `components` - The list of the entity components
///
pub(crate) fn encode_entity(
    entity_id: EntityId,
    name: String,
    mut entity_buffer: &mut [u8],
    components: Vec<AnyComponent>,
) {
    // Store the head
    let head = EntityCellHead::new(entity_id, name);
    let encoded_head = unsafe {
        std::slice::from_raw_parts(
            (&*&head as *const EntityCellHead) as *const u8,
            size_of::<EntityCellHead>(),
        )
    };

    copy(&mut entity_buffer, encoded_head);

    // TODO: Handle the memory leaks
    // Exists to prevent the nested values to be droped
    std::mem::forget(head);

    // Store the component decoding infos
    let mut relative_index = 0;
    let decoding_infos_buffer_index = size_of::<EntityCellHead>();
    for (index, component) in components.iter().enumerate() {
        let buffer_index =
            decoding_infos_buffer_index + index * size_of::<ComponentDecodingInfos>();
        let buffer_end = buffer_index + size_of::<ComponentDecodingInfos>();
        let infos_buffer = &mut entity_buffer[buffer_index..buffer_end];

        let decoding_infos = ComponentDecodingInfos {
            name: component.get_class_name(),
            relative_index,
            size: component.encode_size(),
            decoder: component.get_decoder(),
            decoder_mut: component.get_decoder_mut(),
        };

        relative_index += component.encode_size();

        let encoded_infos = unsafe {
            std::slice::from_raw_parts(
                (&*&decoding_infos as *const ComponentDecodingInfos) as *const u8,
                size_of::<ComponentDecodingInfos>(),
            )
        };

        std::mem::forget(decoding_infos);
        copy(infos_buffer, encoded_infos);
    }

    // Encode every components into the buffer
    let mut component_buffer_index =
        size_of::<EntityCellHead>() + components.len() * size_of::<ComponentDecodingInfos>();
    for component in components.into_iter() {
        let buffer_index = component_buffer_index;
        let buffer_end = component_buffer_index + component.encode_size();
        let component_buffer = &mut entity_buffer[buffer_index..buffer_end];
        component.encode(component_buffer);

        component_buffer_index += component.encode_size();

        // TODO: Handle the memory leaks
        // Exists to prevent the nested values to be droped
        std::mem::forget(component);
    }
}

// Get the entity rw lock stored in an archetype from it's index
///
/// # Arguments
/// * `components` - The list of the entity components
///
pub(crate) fn decode_entity_head<'a>(buffer: &'a [u8], buffer_index: usize) -> &'a EntityCellHead {
    let buffer_end = buffer_index + size_of::<EntityCellHead>();
    let entity_lock_buffer = &buffer[buffer_index..buffer_end];
    let (_head, body, _tail) = unsafe { entity_lock_buffer.align_to::<EntityCellHead>() };
    &body[0]
}

// Get the entity rw lock stored in an archetype from it's index with mutability
///
/// # Arguments
/// * `components` - The list of the entity components
///
pub(crate) fn decode_entity_head_mut<'a>(
    buffer: &'a mut [u8],
    buffer_index: usize,
) -> &'a mut EntityCellHead {
    let buffer_end = buffer_index + size_of::<EntityCellHead>();
    let entity_lock_buffer = &mut buffer[buffer_index..buffer_end];
    let (_head, body, _tail) = unsafe { entity_lock_buffer.align_to_mut::<EntityCellHead>() };
    &mut body[0]
}

// Get the entity rw lock stored in an archetype from it's index
///
/// # Arguments
/// * `components` - The list of the entity components
///
pub(crate) fn decode_components<'a>(
    head: &'a EntityCellHead,
    components_per_entity: usize,
    entity_size: usize,
) -> Vec<&'a dyn Component> {
    let (_, component_infos_buffer, components_buffer) =
        get_entry_buffers(head, components_per_entity, entity_size);

    // Get component decoding infos
    let component_decoding_infos = get_component_decoding_infos(component_infos_buffer);

    // Deserialize every components
    let components = component_decoding_infos
        .iter()
        .map(move |decoding_info| {
            let component_buffer_index = decoding_info.relative_index;
            let component_buffer_end = component_buffer_index + decoding_info.size;
            let component_buffer = &components_buffer[component_buffer_index..component_buffer_end];

            (decoding_info.decoder)(component_buffer)
        })
        .collect::<Vec<_>>();

    components
}

pub(crate) fn decode_components_mut<'a>(
    head: &'a EntityCellHead,
    components_per_entity: usize,
    entity_size: usize,
) -> Vec<&'a mut dyn Component> {
    let (_, component_infos_buffer, components_buffer) =
        get_entry_buffers_mut(head, components_per_entity, entity_size);

    // Get component decoding infos
    let component_decoding_infos = get_component_decoding_infos(component_infos_buffer);

    // Deserialize every components
    let components = component_decoding_infos
        .into_iter()
        .map(|decoding_info| {
            let components_buffer = unsafe { &mut *(components_buffer as *mut _) } as &mut [u8];

            let component_buffer_index = decoding_info.relative_index;
            let component_buffer_end = component_buffer_index + decoding_info.size;
            let component_buffer =
                &mut components_buffer[component_buffer_index..component_buffer_end];

            (decoding_info.decoder_mut)(component_buffer)
        })
        .collect::<Vec<_>>();

    components
}

// Get the entry buffer and split it
// Split the entity buffer into three other ones, one for the lock, one
// for the encoding infos and one for the component datas
fn get_entry_buffers<'a>(
    head: &'a EntityCellHead,
    components_per_entity: usize,
    entity_size: usize,
) -> (&'a [u8], &'a [u8], &'a [u8]) {
    // Get the whole entity buffer
    let entity_buffer = unsafe {
        std::slice::from_raw_parts((&*head as *const EntityCellHead) as *const u8, entity_size)
    };

    // Split the entity buffer into three other one
    let (head_buffer, rest) = entity_buffer.split_at(size_of::<EntityCellHead>());
    let (component_infos_buffer, components_buffer) =
        rest.split_at(components_per_entity * size_of::<ComponentDecodingInfos>());

    (head_buffer, component_infos_buffer, components_buffer)
}

// Get the entry buffer with mutability and split it
// Split the entity buffer into three other ones, one for the lock, one
// for the encoding infos and one for the component datas
fn get_entry_buffers_mut<'a>(
    head: &'a EntityCellHead,
    components_per_entity: usize,
    entity_size: usize,
) -> (&'a mut [u8], &'a mut [u8], &'a mut [u8]) {
    // Get the whole entity buffer
    let entity_buffer = unsafe {
        std::slice::from_raw_parts_mut(
            (&*head as *const EntityCellHead as *mut EntityCellHead) as *mut u8,
            entity_size,
        )
    };

    // Split the entity buffer into three other one
    let (head_buffer, rest) = entity_buffer.split_at_mut(size_of::<EntityCellHead>());
    let (component_infos_buffer, components_buffer) =
        rest.split_at_mut(components_per_entity * size_of::<ComponentDecodingInfos>());

    (head_buffer, component_infos_buffer, components_buffer)
}

// Get the components decoding infos for an entity in the inner_archetype
fn get_component_decoding_infos(entity_bufer: &[u8]) -> Vec<&ComponentDecodingInfos> {
    entity_bufer
        .chunks(size_of::<ComponentDecodingInfos>())
        .map(|chunk| get_component_decoding_info(chunk))
        .collect::<Vec<_>>()
}

// Get the components decoding infos for an entity component in the inner_archetype
fn get_component_decoding_info(entity_bufer: &[u8]) -> &ComponentDecodingInfos {
    let (_head, body, _tail) = unsafe { entity_bufer.align_to::<ComponentDecodingInfos>() };
    &body[0]
}

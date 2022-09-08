use crate::InputService;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::RwLock;
use fruity_windows::window_service::WindowService;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;

pub type DragCallback = Box<dyn Fn(&DragAction) + Send + Sync + 'static>;
pub type DragEndCallback = Box<dyn Fn(&DragAction) + Send + Sync + 'static>;

pub struct DragAction {
    pub start_pos: (u32, u32),
    pub cursor_pos: (u32, u32),
    callback: DragCallback,
    end_callback: DragEndCallback,
}

impl Debug for DragAction {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

#[derive(Debug, FruityAny)]
pub struct DragService {
    current_drag_action: RwLock<Option<DragAction>>,
    input_service: ResourceReference<InputService>,
    window_service: ResourceReference<dyn WindowService>,
}

impl DragService {
    pub fn new(resource_container: ResourceContainer) -> Self {
        let input_service = resource_container.require::<InputService>();
        let window_service = resource_container.require::<dyn WindowService>();
        let window_service_reader = window_service.read();

        let resource_container_2 = resource_container.clone();
        window_service_reader
            .on_end_update()
            .add_observer(move |_| {
                puffin::profile_scope!("update_drag");

                let drag_service = resource_container_2.require::<DragService>();
                let drag_service_reader = drag_service.read();

                drag_service_reader.update_drag();
            });

        Self {
            current_drag_action: RwLock::new(None),
            input_service,
            window_service,
        }
    }

    pub fn start_drag(&self, start_callback: impl Fn() -> (DragCallback, DragEndCallback)) {
        let start_pos = {
            let window_service_reader = self.window_service.read();
            window_service_reader.get_cursor_position()
        };

        let start_callback_result = start_callback();

        let drag_action = DragAction {
            start_pos,
            cursor_pos: start_pos,
            callback: start_callback_result.0,
            end_callback: start_callback_result.1,
        };

        let mut current_drag_action_writer = self.current_drag_action.write();
        *current_drag_action_writer = Some(drag_action);
    }

    pub fn update_drag(&self) {
        // If the left mouse button is released, we stop dragging
        if !self.is_dragging_button_pressed() && self.is_dragging() {
            // Call the end action
            let mut current_drag_action_writer = self.current_drag_action.write();
            if let Some(current_drag_action) = current_drag_action_writer.deref_mut() {
                // Update cursor pos
                current_drag_action.cursor_pos = {
                    let window_service_reader = self.window_service.read();
                    window_service_reader.get_cursor_position()
                };

                (current_drag_action.end_callback)(&current_drag_action);

                // Clear the current action
                *current_drag_action_writer = None;
            }
        };

        // If a drag is active, we execute the associated callback
        let mut current_drag_action_writer = self.current_drag_action.write();
        if let Some(current_drag_action) = current_drag_action_writer.deref_mut() {
            // Update cursor pos
            current_drag_action.cursor_pos = {
                let window_service_reader = self.window_service.read();
                window_service_reader.get_cursor_position()
            };

            (current_drag_action.callback)(&current_drag_action);
        }
    }

    fn is_dragging_button_pressed(&self) -> bool {
        let input_service = self.input_service.read();
        input_service.is_source_pressed("Mouse/Left")
    }

    fn is_dragging(&self) -> bool {
        let current_drag_action_reader = self.current_drag_action.read();

        if let Some(_) = current_drag_action_reader.deref() {
            true
        } else {
            false
        }
    }
}

impl IntrospectObject for DragService {
    fn get_class_name(&self) -> String {
        "DragService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for DragService {}

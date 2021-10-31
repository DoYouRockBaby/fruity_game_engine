use crate::ui_element::layout::Column;
use crate::ui_element::layout::Empty;
use crate::ui_element::UIAlign;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use fruity_core::component::component_rwlock::ComponentRwLock;
use fruity_introspect::FieldInfo;
use std::any::TypeId;

pub mod primitive;

pub trait EditableComponent {
    fn type_id() -> TypeId;
    fn render_edit(component: ComponentRwLock, field_info: &FieldInfo) -> UIElement;
}

pub fn edit_component_component(component: ComponentRwLock) -> UIElement {
    let reader = component.read();
    let fields_edit = reader
        .get_field_infos()
        .iter()
        .map(|field_info| {
            if field_info.ty == u8::type_id() {
                u8::render_edit(component.clone(), field_info)
            } else if field_info.ty == u16::type_id() {
                u16::render_edit(component.clone(), field_info)
            } else if field_info.ty == u32::type_id() {
                u32::render_edit(component.clone(), field_info)
            } else if field_info.ty == u64::type_id() {
                u64::render_edit(component.clone(), field_info)
            } else if field_info.ty == usize::type_id() {
                usize::render_edit(component.clone(), field_info)
            } else if field_info.ty == i8::type_id() {
                i8::render_edit(component.clone(), field_info)
            } else if field_info.ty == i16::type_id() {
                i16::render_edit(component.clone(), field_info)
            } else if field_info.ty == i32::type_id() {
                i32::render_edit(component.clone(), field_info)
            } else if field_info.ty == i64::type_id() {
                i64::render_edit(component.clone(), field_info)
            } else if field_info.ty == isize::type_id() {
                isize::render_edit(component.clone(), field_info)
            } else if field_info.ty == f32::type_id() {
                f32::render_edit(component.clone(), field_info)
            } else if field_info.ty == f64::type_id() {
                f64::render_edit(component.clone(), field_info)
            } else if field_info.ty == bool::type_id() {
                bool::render_edit(component.clone(), field_info)
            } else if field_info.ty == String::type_id() {
                String::render_edit(component.clone(), field_info)
            } else {
                Empty {}.elem()
            }
        })
        .collect::<Vec<_>>();

    Column {
        children: fields_edit,
        align: UIAlign::Start,
    }
    .elem()
}

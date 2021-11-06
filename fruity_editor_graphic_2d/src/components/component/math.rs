use fruity_core::component::component_rwlock::ComponentRwLock;
use fruity_editor::ui_element::display::Text;
use fruity_editor::ui_element::input::FloatInput;
use fruity_editor::ui_element::layout::Row;
use fruity_editor::ui_element::UIElement;
use fruity_editor::ui_element::UIWidget;
use fruity_graphic_2d::math::vector2d::Vector2d;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::SetterCaller;
use std::convert::TryFrom;
use std::sync::Arc;

pub fn draw_editor_vector_2d(component: ComponentRwLock, field_info: &FieldInfo) -> UIElement {
    let reader = component.read();
    let value = (field_info.getter)(reader.as_any_ref());
    let value = if let Ok(value) = Vector2d::try_from(value) {
        value
    } else {
        Vector2d::default()
    };

    let field_info = field_info.clone();
    let field_info_2 = field_info.clone();
    let component = component.clone();
    let component_2 = component.clone();
    Row {
        children: vec![
            Text {
                text: field_info.name.to_string(),
            }
            .elem(),
            FloatInput {
                value: value.x as f64,
                on_change: Arc::new(move |new_x_value: f64| {
                    let mut writer = component.write();

                    match &field_info.setter {
                        SetterCaller::Const(setter) => {
                            let mut value = value;
                            value.x = new_x_value as f32;
                            setter(writer.as_any_ref(), Serialized::try_from(value).unwrap())
                        }
                        SetterCaller::Mut(setter) => {
                            let mut value = value;
                            value.x = new_x_value as f32;
                            setter(writer.as_any_mut(), Serialized::try_from(value).unwrap())
                        }
                        SetterCaller::None => (),
                    };
                }),
            }
            .elem(),
            FloatInput {
                value: value.y as f64,
                on_change: Arc::new(move |new_y_value: f64| {
                    let mut writer = component_2.write();

                    match &field_info_2.setter {
                        SetterCaller::Const(setter) => {
                            let mut value = value;
                            value.y = new_y_value as f32;
                            setter(writer.as_any_ref(), Serialized::try_from(value).unwrap())
                        }
                        SetterCaller::Mut(setter) => {
                            let mut value = value;
                            value.y = new_y_value as f32;
                            setter(writer.as_any_mut(), Serialized::try_from(value).unwrap())
                        }
                        SetterCaller::None => (),
                    };
                }),
            }
            .elem(),
        ],
        ..Default::default()
    }
    .elem()
}

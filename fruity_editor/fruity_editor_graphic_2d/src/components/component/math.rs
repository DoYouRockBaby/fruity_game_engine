use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_editor::ui_element::display::Text;
use fruity_editor::ui_element::input::FloatInput;
use fruity_editor::ui_element::layout::Row;
use fruity_editor::ui_element::layout::RowItem;
use fruity_editor::ui_element::UIElement;
use fruity_editor::ui_element::UISize;
use fruity_editor::ui_element::UIWidget;
use fruity_graphic::math::vector2d::Vector2d;
use std::sync::Arc;

pub fn draw_editor_vector_2d(
    introspect: Box<dyn SerializableObject>,
    field_info: &FieldInfo,
) -> UIElement {
    let value = (field_info.getter)(introspect.as_any_ref());
    let value = if let Ok(value) = Vector2d::fruity_try_from(value) {
        value
    } else {
        Vector2d::default()
    };

    let introspect_2 = introspect.duplicate();
    let field_info = field_info.clone();
    let field_info_2 = field_info.clone();
    Row {
        children: vec![
            RowItem {
                size: UISize::Units(40.0),
                child: Text {
                    text: field_info.name.to_string(),
                    ..Default::default()
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.5),
                child: FloatInput {
                    value: value.x as f64,
                    on_change: Arc::new(move |new_x_value: f64| {
                        match &field_info.setter {
                            SetterCaller::Const(setter) => {
                                let mut value = value;
                                value.x = new_x_value as f32;
                                setter(introspect.as_any_ref(), value.fruity_into())
                            }
                            SetterCaller::Mut(_) => (),
                            SetterCaller::None => (),
                        };
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.5),
                child: FloatInput {
                    value: value.y as f64,
                    on_change: Arc::new(move |new_y_value: f64| {
                        match &field_info_2.setter {
                            SetterCaller::Const(setter) => {
                                let mut value = value;
                                value.y = new_y_value as f32;
                                setter(introspect_2.as_any_ref(), value.fruity_into())
                            }
                            SetterCaller::Mut(_) => (),
                            SetterCaller::None => (),
                        };
                    }),
                }
                .elem(),
                ..Default::default()
            },
        ],
        ..Default::default()
    }
    .elem()
}

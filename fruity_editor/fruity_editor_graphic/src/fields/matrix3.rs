use fruity_core::convert::FruityTryFrom;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use fruity_editor::ui_element::display::Text;
use fruity_editor::ui_element::input::FloatInput;
use fruity_editor::ui_element::layout::Row;
use fruity_editor::ui_element::layout::RowItem;
use fruity_editor::ui_element::UIElement;
use fruity_editor::ui_element::UISize;
use fruity_editor::ui_element::UIWidget;
use fruity_graphic::math::matrix3::Matrix3;
use std::sync::Arc;

pub fn draw_editor_matrix3(
    name: &str,
    value: Box<dyn SerializableObject>,
    on_update: impl Fn(Box<dyn SerializableObject>) + Send + Sync + 'static,
) -> UIElement {
    let value = if let Ok(value) = Matrix3::fruity_try_from(Serialized::NativeObject(value)) {
        value
    } else {
        Matrix3::default()
    };

    let on_update = Arc::new(on_update);
    let on_update_2 = on_update.clone();
    let on_update_3 = on_update.clone();
    let on_update_4 = on_update.clone();
    let on_update_5 = on_update.clone();
    let on_update_6 = on_update.clone();
    let on_update_7 = on_update.clone();
    let on_update_8 = on_update.clone();
    let on_update_9 = on_update.clone();
    Row {
        children: vec![
            RowItem {
                size: UISize::FillPortion(1.0),
                child: Text {
                    text: name.to_string(),
                    ..Default::default()
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[0][0] as f64,
                    on_change: Arc::new(move |new_value: f64| {
                        let mut value = value;
                        value.0[0][0] = new_value as f32;
                        on_update(Box::new(value));
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[0][0] as f64,
                    on_change: Arc::new(move |new_value: f64| {
                        let mut value = value;
                        value.0[0][0] = new_value as f32;
                        on_update_2(Box::new(value));
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[0][0] as f64,
                    on_change: Arc::new(move |new_value: f64| {
                        let mut value = value;
                        value.0[0][0] = new_value as f32;
                        on_update_3(Box::new(value));
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[1][0] as f64,
                    on_change: Arc::new(move |new_value: f64| {
                        let mut value = value;
                        value.0[1][0] = new_value as f32;
                        on_update_4(Box::new(value));
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[1][0] as f64,
                    on_change: Arc::new(move |new_value: f64| {
                        let mut value = value;
                        value.0[1][0] = new_value as f32;
                        on_update_5(Box::new(value));
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[1][0] as f64,
                    on_change: Arc::new(move |new_value: f64| {
                        let mut value = value;
                        value.0[1][0] = new_value as f32;
                        on_update_6(Box::new(value));
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[2][0] as f64,
                    on_change: Arc::new(move |new_value: f64| {
                        let mut value = value;
                        value.0[2][0] = new_value as f32;
                        on_update_7(Box::new(value));
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[2][0] as f64,
                    on_change: Arc::new(move |new_value: f64| {
                        let mut value = value;
                        value.0[2][0] = new_value as f32;
                        on_update_8(Box::new(value));
                    }),
                }
                .elem(),
            },
            RowItem {
                size: UISize::FillPortion(0.33),
                child: FloatInput {
                    value: value.0[2][0] as f64,
                    on_change: Arc::new(move |new_value: f64| {
                        let mut value = value;
                        value.0[2][0] = new_value as f32;
                        on_update_9(Box::new(value));
                    }),
                }
                .elem(),
            },
        ],
        ..Default::default()
    }
    .elem()
}

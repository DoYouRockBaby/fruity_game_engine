use std::any::Any;

pub mod draw_ui_element;

pub enum UIElement {
    Row(Vec<UIElement>),
    Column(Vec<UIElement>),
    Text(String),
    Button {
        label: String,
        on_click: Box<dyn Fn() + Send + Sync>,
    },
    Input {
        label: String,
        value: String,
        placeholder: String,
        on_changed: Box<dyn Fn(&str) + Send + Sync>,
    },
    IntegerInput {
        label: String,
        value: i64,
        on_changed: Box<dyn Fn(i64) + Send + Sync>,
    },
    FloatInput {
        label: String,
        value: f64,
        on_changed: Box<dyn Fn(f64) + Send + Sync>,
    },
    Checkbox {
        label: String,
        value: bool,
        on_changed: Box<dyn Fn(bool) + Send + Sync>,
    },
    ListView {
        items: Vec<Box<dyn Any + Send + Sync>>,
        get_key: Box<dyn Fn(&dyn Any) -> usize + Send + Sync>,
        render_item: Box<dyn Fn(&dyn Any) -> UIElement + Send + Sync>,
        on_clicked: Box<dyn Fn(&dyn Any) + Send + Sync>,
        is_selected: Option<Box<dyn Fn(&dyn Any) -> bool + Send + Sync>>,
    },
}

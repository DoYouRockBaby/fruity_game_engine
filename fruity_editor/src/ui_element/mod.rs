use std::any::Any;

pub mod draw_ui_element;

pub enum UIElement {
    Empty,
    Row(Vec<UIElement>),
    Column(Vec<UIElement>),
    Text(String),
    Button {
        label: String,
        on_click: Box<dyn FnMut() + Send + Sync>,
    },
    Input {
        label: String,
        value: String,
        placeholder: String,
        on_change: Box<dyn FnMut(&str) + Send + Sync>,
    },
    IntegerInput {
        label: String,
        value: i64,
        on_change: Box<dyn FnMut(i64) + Send + Sync>,
    },
    FloatInput {
        label: String,
        value: f64,
        on_change: Box<dyn FnMut(f64) + Send + Sync>,
    },
    Checkbox {
        label: String,
        value: bool,
        on_change: Box<dyn FnMut(bool) + Send + Sync>,
    },
    ListView {
        items: Vec<Box<dyn Any + Send + Sync>>,
        get_key: Box<dyn Fn(&dyn Any) -> usize + Send + Sync>,
        render_item: Box<dyn Fn(&dyn Any) -> UIElement + Send + Sync>,
        on_clicked: Box<dyn FnMut(&dyn Any) + Send + Sync>,
    },
}

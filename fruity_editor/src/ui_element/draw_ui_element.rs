use crate::hooks::topo;
use crate::hooks::use_global;
use crate::hooks::use_state;
use crate::hooks::CloneState;
use crate::state::theme::ThemeState;
use crate::state::Message;
use crate::ui_element::UIAlign;
use crate::ui_element::UIElement;
use iced::button;
use iced::scrollable;
use iced::text_input;
use iced::Alignment;
use iced::Button;
use iced::Checkbox;
use iced::Column;
use iced::Container;
use iced::Row;
use iced::Scrollable;
use iced::Text;
use iced::TextInput;
use iced_wgpu::Renderer;
use iced_winit::Element;
use iced_winit::Length;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

impl Into<Alignment> for UIAlign {
    fn into(self) -> Alignment {
        match self {
            UIAlign::Start => Alignment::Start,
            UIAlign::Center => Alignment::Center,
            UIAlign::End => Alignment::End,
        }
    }
}

#[topo::nested]
pub fn draw_ui_element<'a>(element: UIElement) -> Element<'a, Message, Renderer> {
    match element {
        UIElement::Empty => draw_empty(),
        UIElement::Row { children, align } => draw_row(children, align),
        UIElement::Column { children, align } => draw_column(children, align),
        UIElement::Text(text) => draw_text(text),
        UIElement::Button { label, on_click } => draw_button(label, on_click),
        UIElement::Input {
            label,
            value,
            placeholder,
            on_change,
        } => draw_input(&label, &value, &placeholder, on_change),
        UIElement::IntegerInput {
            label,
            value,
            on_change,
        } => draw_integer_input(&label, value, on_change),
        UIElement::FloatInput {
            label,
            value,
            on_change,
        } => draw_float_input(&label, value, on_change),
        UIElement::Checkbox {
            label,
            value,
            on_change,
        } => draw_checkbox(label, value, on_change),
        UIElement::ListView {
            items,
            get_key,
            render_item,
            on_clicked,
        } => draw_listview(items, get_key, render_item, on_clicked),
    }
}

fn draw_empty<'a>() -> Element<'a, Message, Renderer> {
    Row::new().into()
}

fn draw_row<'a>(children: Vec<UIElement>, align: UIAlign) -> Element<'a, Message, Renderer> {
    let theme_state = use_global::<ThemeState>();

    Container::new(
        children
            .into_iter()
            .fold(Row::new().align_items(align.into()), |row, element| {
                row.push(draw_ui_element(element))
            }),
    )
    .style(theme_state.theme)
    .into()
}

fn draw_column<'a>(children: Vec<UIElement>, align: UIAlign) -> Element<'a, Message, Renderer> {
    let theme_state = use_global::<ThemeState>();

    Container::new(
        children
            .into_iter()
            .fold(Column::new().align_items(align.into()), |row, element| {
                row.push(draw_ui_element(element))
            }),
    )
    .style(theme_state.theme)
    .into()
}

fn draw_text<'a>(text: String) -> Element<'a, Message, Renderer> {
    Text::new(text).size(16).into()
}

#[topo::nested]
fn draw_button<'a>(
    label: String,
    on_click: Box<dyn FnMut() + Send + Sync>,
) -> Element<'a, Message, Renderer> {
    let theme_state = use_global::<ThemeState>();
    let button_state = use_state(|| Rc::<RefCell<button::State>>::default());
    let button_state = button_state.get();
    let mut button_state = button_state.borrow_mut();

    // TODO: Try to find a way to remove that
    // Create a custom use_state for mutable references
    let button_state = unsafe {
        std::mem::transmute::<&mut iced::button::State, &mut iced::button::State>(&mut button_state)
    };

    let label = Text::new(label).size(16);
    let on_click = Arc::new(Mutex::new(on_click));
    Button::new(button_state, label)
        .on_press(Message::Callback(on_click))
        .style(theme_state.theme)
        .into()
}

#[topo::nested]
fn draw_input<'a>(
    label: &str,
    value: &str,
    placeholder: &str,
    on_change: Box<dyn FnMut(&str) + Send + Sync>,
) -> Element<'a, Message, Renderer> {
    let theme_state = use_global::<ThemeState>();
    let input_state = use_state(|| Rc::<RefCell<text_input::State>>::default());
    let input_state = input_state.get();
    let mut input_state = input_state.borrow_mut();

    // TODO: Try to find a way to remove that
    // Create a custom use_state for mutable references
    let input_state = unsafe {
        std::mem::transmute::<&mut iced::text_input::State, &mut iced::text_input::State>(
            &mut input_state,
        )
    };

    let input_value = use_state(|| "".to_string());
    if !input_state.is_focused() {
        input_value.set(value.to_string());
    }

    let label = Text::new(label).size(16);
    let on_change = Arc::new(Mutex::new(on_change));
    let input: Element<Message, Renderer> = TextInput::new(
        input_state,
        &placeholder,
        &input_value.get(),
        move |value| {
            input_value.set(value.clone());
            Message::Empty
        },
    )
    .on_submit(Message::StringChanged(on_change.clone(), input_value.get()))
    .padding(4)
    .size(16)
    .style(theme_state.theme)
    .into();

    Row::new().push(label).push(input).into()
}

fn draw_integer_input<'a>(
    label: &str,
    value: i64,
    mut on_change: Box<dyn FnMut(i64) + Send + Sync>,
) -> Element<'a, Message, Renderer> {
    draw_input(
        label,
        &value.to_string(),
        "",
        Box::new(move |value| {
            if let Ok(value) = value.parse::<i64>() {
                on_change(value)
            }
        }),
    )
}

fn draw_float_input<'a>(
    label: &str,
    value: f64,
    mut on_change: Box<dyn FnMut(f64) + Send + Sync>,
) -> Element<'a, Message, Renderer> {
    draw_input(
        label,
        &value.to_string(),
        "",
        Box::new(move |value| {
            if let Ok(value) = value.parse::<f64>() {
                on_change(value)
            }
        }),
    )
}

fn draw_checkbox<'a>(
    label: String,
    value: bool,
    on_change: Box<dyn FnMut(bool) + Send + Sync>,
) -> Element<'a, Message, Renderer> {
    let theme_state = use_global::<ThemeState>();

    let on_change = Arc::new(Mutex::new(on_change));
    Checkbox::new(value, &label, move |value| {
        Message::BoolChanged(on_change.clone(), value)
    })
    .size(16)
    .style(theme_state.theme)
    .into()
}

#[topo::nested]
fn draw_listview<'a>(
    items: Vec<Box<dyn Any + Send + Sync>>,
    get_key: Box<dyn Fn(&dyn Any) -> usize + Send + Sync>,
    render_item: Box<dyn Fn(&dyn Any) -> UIElement + Send + Sync>,
    on_clicked: Box<dyn FnMut(&dyn Any) + Send + Sync>,
) -> Element<'a, Message, Renderer> {
    let theme_state = use_global::<ThemeState>();
    let scroll_state = use_state(|| Rc::<RefCell<scrollable::State>>::default());
    let list_state = use_state(|| Rc::<RefCell<HashMap<usize, button::State>>>::default());

    let scroll_state = scroll_state.get();
    let mut scroll_state = scroll_state.borrow_mut();

    // TODO: Try to find a way to remove that
    // Create a custom use_state for mutable references
    let scroll_state = unsafe {
        std::mem::transmute::<&mut iced::scrollable::State, &mut iced::scrollable::State>(
            &mut scroll_state,
        )
    };

    let old_list_state = list_state.get();
    let mut old_list_state = old_list_state.borrow_mut();

    // Update list button states from keys
    let keys = items
        .iter()
        .map(|item| get_key(item.deref()))
        .collect::<Vec<_>>();

    let new_list_state = keys
        .into_iter()
        .fold(HashMap::new(), |mut new_list_state, key| {
            let item_state = match old_list_state.remove(&key) {
                Some(item_state) => item_state,
                None => button::State::default(),
            };

            new_list_state.insert(key, item_state);
            new_list_state
        });

    let new_list_state = Rc::new(RefCell::new(new_list_state));
    list_state.set(new_list_state);

    let list_state = list_state.get();
    let mut list_state = list_state.borrow_mut();

    let on_clicked = Arc::new(Mutex::new(on_clicked));
    let content = items.into_iter().fold(
        Scrollable::new(scroll_state)
            .width(Length::Fill)
            .height(Length::Units(500))
            .style(theme_state.theme),
        |scrollable, item| {
            // TODO: Try to find a way to remove that
            // Create a custom use_state for mutable references
            let list_state = unsafe {
                std::mem::transmute::<
                    &mut HashMap<usize, button::State>,
                    &mut HashMap<usize, button::State>,
                >(&mut list_state)
            };

            scrollable.push({
                let key = get_key(item.deref());
                let rendered_item = render_item(item.deref());

                let item_state = list_state.get_mut(&key).unwrap();
                let item: Element<Message, Renderer> =
                    Button::new(item_state, draw_ui_element(rendered_item))
                        .style(theme_state.theme.list_item())
                        .on_press(Message::AnyChanged(on_clicked.clone(), item.into()))
                        .width(Length::Fill)
                        .into();

                item
            })
        },
    );

    Container::new(content)
        .style(theme_state.theme.list_view())
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

use crate::state::Message;
use crate::state::State;
use crate::ui_element::UIElement;
use comp_state::topo;
use comp_state::use_state;
use comp_state::CloneState;
use comp_state::StateAccess;
use iced::button;
use iced::scrollable;
use iced::text_input;
use iced::Button;
use iced::Checkbox;
use iced::Column;
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
use std::rc::Rc;
use std::sync::Arc;

#[topo::nested]
fn draw_ui_element(element: UIElement, state: &State) -> Element<Message, Renderer> {
    let button_state = use_state(|| Rc::<RefCell<button::State>>::default());
    let input_state = use_state(|| Rc::<RefCell<text_input::State>>::default());
    let scroll_state = use_state(|| Rc::<RefCell<scrollable::State>>::default());
    let list_state = use_state(|| Rc::<RefCell<HashMap<usize, button::State>>>::default());

    match element {
        UIElement::Row(elements) => draw_row(elements, state),
        UIElement::Column(elements) => draw_column(elements, state),
        UIElement::Text(text) => draw_text(text, state),
        UIElement::Button { label, on_click } => {
            let button_state = button_state.get();
            let mut button_state = button_state.borrow_mut();

            // TODO: Try to find a way to remove that
            // Create a custom use_state for mutable references
            let button_state = unsafe {
                std::mem::transmute::<&mut iced::button::State, &mut iced::button::State>(
                    &mut button_state,
                )
            };

            draw_button(label, on_click, button_state, state)
        }
        UIElement::Input {
            label,
            value,
            placeholder,
            on_changed,
        } => {
            let input_state = input_state.get();
            let mut input_state = input_state.borrow_mut();

            // TODO: Try to find a way to remove that
            // Create a custom use_state for mutable references
            let input_state = unsafe {
                std::mem::transmute::<&mut iced::text_input::State, &mut iced::text_input::State>(
                    &mut input_state,
                )
            };

            draw_input(label, value, placeholder, on_changed, input_state, state)
        }
        UIElement::IntegerInput {
            label,
            value,
            on_changed,
        } => {
            let input_state = input_state.get();
            let mut input_state = input_state.borrow_mut();

            // TODO: Try to find a way to remove that
            // Create a custom use_state for mutable references
            let input_state = unsafe {
                std::mem::transmute::<&mut iced::text_input::State, &mut iced::text_input::State>(
                    &mut input_state,
                )
            };

            draw_integer_input(label, value, on_changed, input_state, state)
        }
        UIElement::FloatInput {
            label,
            value,
            on_changed,
        } => {
            let input_state = input_state.get();
            let mut input_state = input_state.borrow_mut();

            // TODO: Try to find a way to remove that
            // Create a custom use_state for mutable references
            let input_state = unsafe {
                std::mem::transmute::<&mut iced::text_input::State, &mut iced::text_input::State>(
                    &mut input_state,
                )
            };

            draw_float_input(label, value, on_changed, input_state, state)
        }
        UIElement::Checkbox {
            label,
            value,
            on_changed,
        } => draw_checkbox(label, value, on_changed, state),
        UIElement::ListView {
            items,
            get_key,
            render_item,
            on_clicked,
            is_selected,
        } => {
            let scroll_state = scroll_state.get();
            let mut scroll_state = scroll_state.borrow_mut();

            // TODO: Try to find a way to remove that
            // Create a custom use_state for mutable references
            let scroll_state = unsafe {
                std::mem::transmute::<&mut iced::scrollable::State, &mut iced::scrollable::State>(
                    &mut scroll_state,
                )
            };

            draw_listview(
                items,
                get_key,
                render_item,
                on_clicked,
                is_selected,
                scroll_state,
                &list_state,
                state,
            )
        }
    }
}

fn draw_row(elements: Vec<UIElement>, state: &State) -> Element<Message, Renderer> {
    elements
        .into_iter()
        .fold(Row::new(), |row, element| {
            row.push(draw_ui_element(element, state))
        })
        .into()
}

fn draw_column(elements: Vec<UIElement>, state: &State) -> Element<Message, Renderer> {
    elements
        .into_iter()
        .fold(Column::new(), |row, element| {
            row.push(draw_ui_element(element, state))
        })
        .into()
}

fn draw_text(text: String, _state: &State) -> Element<Message, Renderer> {
    Text::new(text).size(16).into()
}

fn draw_button<'a>(
    label: String,
    on_click: Box<dyn Fn() + Send + Sync>,
    button_state: &'a mut iced::button::State,
    state: &State,
) -> Element<'a, Message, Renderer> {
    let label = Text::new(label).size(16);
    Button::new(button_state, label)
        .on_press(Message::Callback(on_click.into()))
        .style(state.theme.theme)
        .into()
}

fn draw_input<'a>(
    label: String,
    value: String,
    placeholder: String,
    on_changed: Box<dyn Fn(&str) + Send + Sync>,
    input_state: &'a mut text_input::State,
    state: &State,
) -> Element<'a, Message, Renderer> {
    let label = Text::new(label).size(16);
    let on_changed: Arc<dyn Fn(&str) + Send + Sync> = on_changed.into();
    let input: Element<Message, Renderer> =
        TextInput::new(input_state, &placeholder, &value, move |value| {
            Message::StringChanged(on_changed.clone(), value)
        })
        .style(state.theme.theme)
        .into();

    Row::new().push(label).push(input).into()
}

fn draw_integer_input<'a>(
    label: String,
    value: i64,
    on_changed: Box<dyn Fn(i64) + Send + Sync>,
    input_state: &'a mut text_input::State,
    state: &State,
) -> Element<'a, Message, Renderer> {
    let label = Text::new(label).size(16);
    let on_changed: Arc<dyn Fn(i64) + Send + Sync> = on_changed.into();
    let input: Element<Message, Renderer> =
        TextInput::new(input_state, "", &value.to_string(), move |value| {
            if let Ok(value) = value.parse::<i64>() {
                Message::IntegerChanged(on_changed.clone(), value)
            } else {
                Message::Empty
            }
        })
        .style(state.theme.theme)
        .into();

    Row::new().push(label).push(input).into()
}

fn draw_float_input<'a>(
    label: String,
    value: f64,
    on_changed: Box<dyn Fn(f64) + Send + Sync>,
    input_state: &'a mut text_input::State,
    state: &State,
) -> Element<'a, Message, Renderer> {
    let label = Text::new(label).size(16);
    let on_changed: Arc<dyn Fn(f64) + Send + Sync> = on_changed.into();
    let input: Element<Message, Renderer> =
        TextInput::new(input_state, "", &value.to_string(), move |value| {
            if let Ok(value) = value.parse::<f64>() {
                Message::FloatChanged(on_changed.clone(), value)
            } else {
                Message::Empty
            }
        })
        .style(state.theme.theme)
        .into();

    Row::new().push(label).push(input).into()
}

fn draw_checkbox(
    label: String,
    value: bool,
    on_changed: Box<dyn Fn(bool) + Send + Sync>,
    state: &State,
) -> Element<Message, Renderer> {
    let on_changed: Arc<dyn Fn(bool) + Send + Sync> = on_changed.into();
    Checkbox::new(value, &label, move |value| {
        Message::BoolChanged(on_changed.clone(), value)
    })
    .style(state.theme.theme)
    .into()
}

fn draw_listview<'a>(
    items: Vec<Box<dyn Any + Send + Sync>>,
    get_key: Box<dyn Fn(&dyn Any) -> usize + Send + Sync>,
    render_item: Box<dyn Fn(&dyn Any) -> UIElement + Send + Sync>,
    on_clicked: Box<dyn Fn(&dyn Any) + Send + Sync>,
    _is_selected: Option<Box<dyn Fn(&dyn Any) -> bool + Send + Sync>>,
    scroll_state: &'a mut scrollable::State,
    list_state: &StateAccess<Rc<RefCell<HashMap<usize, button::State>>>>,
    state: &'a State,
) -> Element<'a, Message, Renderer> {
    let old_list_state = list_state.get();
    let mut old_list_state = old_list_state.borrow_mut();

    // Update list button states from keys
    let keys = items.iter().map(|item| get_key(item)).collect::<Vec<_>>();
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

    let on_clicked: Arc<dyn Fn(&dyn Any) + Send + Sync> = on_clicked.into();
    items
        .into_iter()
        .fold(
            Scrollable::new(scroll_state)
                .width(Length::Fill)
                .height(Length::Units(500))
                .style(state.theme.theme),
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
                    let key = get_key(&item);
                    let rendered_item = render_item(&item);

                    let item_state = list_state.get_mut(&key).unwrap();
                    let item: Element<Message, Renderer> =
                        Button::new(item_state, draw_ui_element(rendered_item, state))
                            .style(state.theme.theme.list_item())
                            .on_press(Message::AnyChanged(on_clicked.clone(), item.into()))
                            .width(Length::Fill)
                            .into();

                    item
                })
            },
        )
        .into()
}

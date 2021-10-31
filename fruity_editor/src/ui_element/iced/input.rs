use crate::hooks::topo;
use crate::hooks::use_global;
use crate::hooks::use_state;
use crate::state::theme::ThemeState;
use crate::ui_element::input::Button;
use crate::ui_element::input::Checkbox;
use crate::ui_element::input::FloatInput;
use crate::ui_element::input::Input;
use crate::ui_element::input::IntegerInput;
use crate::ui_element::Message;
use comp_state::CloneState;
use iced::button;
use iced::text_input;
use iced::Button as IcedButton;
use iced::Checkbox as IcedCheckbox;
use iced::Row as IcedRow;
use iced::Text as IcedText;
use iced::TextInput as IcedTextInput;
use iced_wgpu::Renderer;
use iced_winit::Element;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

#[topo::nested]
pub fn draw_button<'a>(elem: Button) -> Element<'a, Message, Renderer> {
    let theme_state = use_global::<ThemeState>();
    let button_state = use_state(|| Rc::<RefCell<button::State>>::default());
    let button_state = button_state.get();
    let mut button_state = button_state.borrow_mut();

    // TODO: Try to find a way to remove that
    // Create a custom use_state for mutable references
    let button_state = unsafe {
        std::mem::transmute::<&mut iced::button::State, &mut iced::button::State>(&mut button_state)
    };

    let label = IcedText::new(elem.label).size(16);
    IcedButton::new(button_state, label)
        .on_press(Message::Callback(elem.on_click.clone()))
        .style(theme_state.theme)
        .into()
}

#[topo::nested]
pub fn draw_input<'a>(elem: Input) -> Element<'a, Message, Renderer> {
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
        input_value.set(elem.value);
    }

    let label = IcedText::new(elem.label).size(16);
    let input: Element<Message, Renderer> = IcedTextInput::new(
        input_state,
        &elem.placeholder,
        &input_value.get(),
        move |value| {
            input_value.set(value.clone());
            Message::Empty
        },
    )
    .on_submit(Message::StringChanged(
        elem.on_change.clone(),
        input_value.get(),
    ))
    .padding(4)
    .size(16)
    .style(theme_state.theme)
    .into();

    IcedRow::new().push(label).push(input).into()
}

pub fn draw_integer_input<'a>(elem: IntegerInput) -> Element<'a, Message, Renderer> {
    let input = Input {
        label: elem.label.clone(),
        value: elem.value.to_string(),
        placeholder: "".to_string(),
        on_change: Arc::new(move |value: &str| {
            if let Ok(value) = value.parse::<i64>() {
                (elem.on_change)(value)
            }
        }),
    };

    draw_input(input)
}

pub fn draw_float_input<'a>(elem: FloatInput) -> Element<'a, Message, Renderer> {
    let input = Input {
        label: elem.label.clone(),
        value: elem.value.to_string(),
        placeholder: "".to_string(),
        on_change: Arc::new(move |value: &str| {
            if let Ok(value) = value.parse::<f64>() {
                (elem.on_change)(value)
            }
        }),
    };

    draw_input(input)
}

pub fn draw_checkbox<'a>(elem: Checkbox) -> Element<'a, Message, Renderer> {
    let theme_state = use_global::<ThemeState>();

    let on_change = elem.on_change;
    IcedCheckbox::new(elem.value, &elem.label, move |value| {
        Message::BoolChanged(on_change.clone(), value)
    })
    .size(16)
    .style(theme_state.theme)
    .into()
}

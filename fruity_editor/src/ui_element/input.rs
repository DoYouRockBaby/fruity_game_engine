use crate::hooks::topo;
use crate::hooks::use_global;
use crate::hooks::use_state;
use crate::state::theme::ThemeState;
use crate::ui_element::Message;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
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
use std::sync::Mutex;

pub struct Button {
    pub label: String,
    pub on_click: Arc<Mutex<dyn FnMut() + Send + Sync>>,
}

impl UIWidget for Button {
    #[topo::nested]
    fn draw<'a>(&self) -> Element<'a, Message, Renderer> {
        let theme_state = use_global::<ThemeState>();
        let button_state = use_state(|| Rc::<RefCell<button::State>>::default());
        let button_state = button_state.get();
        let mut button_state = button_state.borrow_mut();

        // TODO: Try to find a way to remove that
        // Create a custom use_state for mutable references
        let button_state = unsafe {
            std::mem::transmute::<&mut iced::button::State, &mut iced::button::State>(
                &mut button_state,
            )
        };

        let label = IcedText::new(&self.label).size(16);
        IcedButton::new(button_state, label)
            .on_press(Message::Callback(self.on_click.clone()))
            .style(theme_state.theme)
            .into()
    }

    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct Input {
    pub label: String,
    pub value: String,
    pub placeholder: String,
    pub on_change: Arc<Mutex<dyn FnMut(&str) + Send + Sync>>,
}

impl UIWidget for Input {
    #[topo::nested]
    fn draw<'a>(&self) -> Element<'a, Message, Renderer> {
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
            input_value.set(self.value.clone());
        }

        let label = IcedText::new(&self.label).size(16);
        let input: Element<Message, Renderer> = IcedTextInput::new(
            input_state,
            &self.placeholder,
            &input_value.get(),
            move |value| {
                input_value.set(value.clone());
                Message::Empty
            },
        )
        .on_submit(Message::StringChanged(
            self.on_change.clone(),
            input_value.get(),
        ))
        .padding(4)
        .size(16)
        .style(theme_state.theme)
        .into();

        IcedRow::new().push(label).push(input).into()
    }

    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct IntegerInput {
    pub label: String,
    pub value: i64,
    pub on_change: Arc<Mutex<dyn FnMut(i64) + Send + Sync>>,
}

impl UIWidget for IntegerInput {
    fn draw<'a>(&self) -> Element<'a, Message, Renderer> {
        let on_change = self.on_change.clone();
        let input = Input {
            label: self.label.clone(),
            value: self.value.to_string(),
            placeholder: "".to_string(),
            on_change: Arc::new(Mutex::new(move |value: &str| {
                if let Ok(value) = value.parse::<i64>() {
                    let mut on_change = on_change.lock().unwrap();
                    on_change(value)
                }
            })),
        };

        input.draw()
    }

    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct FloatInput {
    pub label: String,
    pub value: f64,
    pub on_change: Arc<Mutex<dyn FnMut(f64) + Send + Sync>>,
}

impl UIWidget for FloatInput {
    fn draw<'a>(&self) -> Element<'a, Message, Renderer> {
        let on_change = self.on_change.clone();
        let input = Input {
            label: self.label.clone(),
            value: self.value.to_string(),
            placeholder: "".to_string(),
            on_change: Arc::new(Mutex::new(move |value: &str| {
                if let Ok(value) = value.parse::<f64>() {
                    let mut on_change = on_change.lock().unwrap();
                    on_change(value)
                }
            })),
        };

        input.draw()
    }

    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct Checkbox {
    pub label: String,
    pub value: bool,
    pub on_change: Arc<Mutex<dyn FnMut(bool) + Send + Sync>>,
}

impl UIWidget for Checkbox {
    fn draw<'a>(&self) -> Element<'a, Message, Renderer> {
        let theme_state = use_global::<ThemeState>();

        let on_change = self.on_change.clone();
        IcedCheckbox::new(self.value, &self.label, move |value| {
            Message::BoolChanged(on_change.clone(), value)
        })
        .size(16)
        .style(theme_state.theme)
        .into()
    }

    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

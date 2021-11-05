use crate::hooks::topo;
use crate::hooks::use_state;
use crate::ui_element::input::Button;
use crate::ui_element::input::Checkbox;
use crate::ui_element::input::FloatInput;
use crate::ui_element::input::Input;
use crate::ui_element::input::IntegerInput;
use comp_state::CloneState;
use std::sync::Arc;

#[topo::nested]
pub fn draw_button<'a>(elem: Button, ui: &mut egui::Ui) {
    if ui.button(elem.label).clicked() {
        (elem.on_click)()
    }
}

#[topo::nested]
pub fn draw_input<'a>(elem: Input, ui: &mut egui::Ui) {
    let input_value = use_state(|| String::default());

    let mut new_value = input_value.get();
    let response = ui.add(egui::TextEdit::singleline(&mut new_value).hint_text(&elem.placeholder));

    if response.lost_focus() {
        (elem.on_change)(&new_value);
    }

    if response.changed() {
        input_value.set(new_value);
    }

    if !response.has_focus() {
        input_value.set(elem.value);
    }
}

pub fn draw_integer_input<'a>(elem: IntegerInput, ui: &mut egui::Ui) {
    let input = Input {
        value: elem.value.to_string(),
        placeholder: "".to_string(),
        on_change: Arc::new(move |value: &str| {
            if let Ok(value) = value.parse::<i64>() {
                (elem.on_change)(value)
            }
        }),
    };

    draw_input(input, ui)
}

pub fn draw_float_input<'a>(elem: FloatInput, ui: &mut egui::Ui) {
    let input = Input {
        value: elem.value.to_string(),
        placeholder: "".to_string(),
        on_change: Arc::new(move |value: &str| {
            if let Ok(value) = value.parse::<f64>() {
                (elem.on_change)(value)
            }
        }),
    };

    draw_input(input, ui)
}

pub fn draw_checkbox<'a>(elem: Checkbox, ui: &mut egui::Ui) {
    let mut new_value = elem.value;
    ui.checkbox(&mut new_value, &elem.label);

    if new_value != elem.value {
        (elem.on_change)(new_value);
    }
}

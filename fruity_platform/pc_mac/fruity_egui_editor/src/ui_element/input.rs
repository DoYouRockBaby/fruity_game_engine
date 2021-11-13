use crate::ui_element::app::DrawContext;
use comp_state::CloneState;
use fruity_editor::hooks::topo;
use fruity_editor::hooks::use_state;
use fruity_editor::ui_element::input::Button;
use fruity_editor::ui_element::input::Checkbox;
use fruity_editor::ui_element::input::FloatInput;
use fruity_editor::ui_element::input::ImageButton;
use fruity_editor::ui_element::input::Input;
use fruity_editor::ui_element::input::IntegerInput;
use fruity_wgpu_graphic::resources::texture_resource::WgpuTextureResource;
use std::sync::Arc;

#[topo::nested]
pub fn draw_button<'a>(elem: Button, ui: &mut egui::Ui, _ctx: &mut DrawContext) {
    if ui
        .add_enabled(elem.enabled, egui::Button::new(elem.label))
        .clicked()
    {
        (elem.on_click)()
    }
}

#[topo::nested]
pub fn draw_image_button<'a>(elem: ImageButton, ui: &mut egui::Ui, ctx: &mut DrawContext) {
    let egui_texture_id = {
        let image = elem.image.read();
        let image = image.downcast_ref::<WgpuTextureResource>();

        ctx.egui_rpass.egui_texture_from_wgpu_texture(
            ctx.device,
            &image.texture,
            wgpu::FilterMode::Linear,
        )
    };

    if ui
        .add(egui::ImageButton::new(
            egui_texture_id,
            egui::Vec2::new(elem.width, elem.height),
        ))
        .clicked()
    {
        (elem.on_click)()
    }
}

#[topo::nested]
pub fn draw_input<'a>(elem: Input, ui: &mut egui::Ui, _ctx: &mut DrawContext) {
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

pub fn draw_integer_input<'a>(elem: IntegerInput, ui: &mut egui::Ui, ctx: &mut DrawContext) {
    let input = Input {
        value: elem.value.to_string(),
        placeholder: "".to_string(),
        on_change: Arc::new(move |value: &str| {
            if let Ok(value) = value.parse::<i64>() {
                (elem.on_change)(value)
            }
        }),
    };

    draw_input(input, ui, ctx)
}

pub fn draw_float_input<'a>(elem: FloatInput, ui: &mut egui::Ui, ctx: &mut DrawContext) {
    let input = Input {
        value: elem.value.to_string(),
        placeholder: "".to_string(),
        on_change: Arc::new(move |value: &str| {
            if let Ok(value) = value.parse::<f64>() {
                (elem.on_change)(value)
            }
        }),
    };

    draw_input(input, ui, ctx)
}

pub fn draw_checkbox<'a>(elem: Checkbox, ui: &mut egui::Ui, _ctx: &mut DrawContext) {
    let mut new_value = elem.value;
    ui.add(egui::Checkbox::new(&mut new_value, &elem.label));

    if new_value != elem.value {
        (elem.on_change)(new_value);
    }
}
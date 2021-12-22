use crate::ui_element::app::DrawContext;
use crate::ui_element::topo::CallId;
use comp_state::CloneState;
use egui::epaint;
use egui::CursorIcon;
use egui::Id;
use egui::LayerId;
use egui::Order;
use egui::Response;
use egui::Sense;
use egui::Shape;
use egui::Ui;
use fruity_editor::hooks::topo;
use fruity_editor::hooks::use_state;
use fruity_editor::ui_element::input::Button;
use fruity_editor::ui_element::input::Checkbox;
use fruity_editor::ui_element::input::FloatInput;
use fruity_editor::ui_element::input::ImageButton;
use fruity_editor::ui_element::input::Input;
use fruity_editor::ui_element::input::IntegerInput;
use fruity_wgpu_graphic::resources::texture_resource::WgpuTextureResource;
use lazy_static::*;
use std::any::Any;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    static ref CURRENT_DRAGGED_ITEM: Mutex::<Option<Arc<dyn Any + Send + Sync>>> = Mutex::new(None);
}

#[topo::nested]
pub fn draw_button<'a>(elem: Button, ui: &mut egui::Ui, _ctx: &mut DrawContext) {
    let response = ui.add_enabled(elem.enabled, egui::Button::new(elem.label.clone()));

    if response.clicked() {
        (elem.on_click)()
    }

    // Handle drag & drop
    if let Some(drag_item) = &elem.drag_item {
        drag_source(
            ui,
            Id::new("item").with(CallId::current()),
            response.clone(),
            move || {
                let mut current_dragged_item = CURRENT_DRAGGED_ITEM.lock().unwrap();
                *current_dragged_item = Some(drag_item.clone());
            },
            |ui| {
                ui.add_enabled(false, egui::Button::new(elem.label.clone()));
            },
        )
    }

    if let Some(on_drag) = &elem.on_drag {
        let accept_dragged = if let Some(accept_drag) = &elem.accept_drag {
            let current_dragged_item = CURRENT_DRAGGED_ITEM.lock().unwrap();

            if let Some(current_dragged_item) = current_dragged_item.deref() {
                accept_drag(current_dragged_item.deref())
            } else {
                false
            }
        } else {
            false
        };

        drag_target(
            ui,
            Id::new("item").with(CallId::current()),
            accept_dragged,
            move || {
                let mut current_dragged_item = CURRENT_DRAGGED_ITEM.lock().unwrap();

                if let Some(current_dragged_item) = current_dragged_item.take() {
                    on_drag(current_dragged_item.deref());
                }
            },
            response.clone(),
        )
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

    let response = ui.add(egui::ImageButton::new(
        egui_texture_id,
        egui::Vec2::new(elem.width, elem.height),
    ));

    if response.clicked() {
        (elem.on_click)()
    }

    // Handle drag & drop
    if let Some(drag_item) = &elem.drag_item {
        drag_source(
            ui,
            Id::new("item").with(CallId::current()),
            response.clone(),
            move || {
                let mut current_dragged_item = CURRENT_DRAGGED_ITEM.lock().unwrap();
                *current_dragged_item = Some(drag_item.clone());
            },
            |ui| {
                ui.add(egui::ImageButton::new(
                    egui_texture_id,
                    egui::Vec2::new(elem.width, elem.height),
                ));
            },
        )
    }

    if let Some(on_drag) = &elem.on_drag {
        let accept_dragged = if let Some(accept_drag) = &elem.accept_drag {
            let current_dragged_item = CURRENT_DRAGGED_ITEM.lock().unwrap();

            if let Some(current_dragged_item) = current_dragged_item.deref() {
                accept_drag(current_dragged_item.deref())
            } else {
                false
            }
        } else {
            false
        };

        drag_target(
            ui,
            Id::new("item").with(CallId::current()),
            accept_dragged,
            move || {
                let mut current_dragged_item = CURRENT_DRAGGED_ITEM.lock().unwrap();

                if let Some(current_dragged_item) = current_dragged_item.take() {
                    on_drag(current_dragged_item.deref());
                }
            },
            response.clone(),
        )
    }
}

fn drag_source(
    ui: &mut Ui,
    id: Id,
    response: Response,
    on_drag: impl FnOnce(),
    body: impl FnOnce(&mut Ui),
) {
    let is_being_dragged = ui.memory().is_being_dragged(id);
    let response = ui.interact(response.rect, id, Sense::drag());

    if response.drag_started() {
        on_drag();
    }

    if !is_being_dragged {
        if response.hovered() {
            ui.output().cursor_icon = CursorIcon::Grab;
        }
    } else {
        ui.output().cursor_icon = CursorIcon::Grabbing;

        // Paint the body to a new layer:
        let layer_id = LayerId::new(Order::Tooltip, id);
        let response = ui.with_layer_id(layer_id, body).response;

        // Move the visuals of the body to where the mouse is.
        if let Some(pointer_pos) = ui.input().pointer.interact_pos() {
            let delta = pointer_pos - response.rect.center();
            ui.ctx().translate_layer(layer_id, delta);
        }
    }
}

fn drag_target(
    ui: &mut Ui,
    id: Id,
    accept_dragged: bool,
    on_drag: impl FnOnce(),
    response: Response,
) {
    let response = ui.interact(response.rect, id, Sense::hover());
    let is_being_dragged = ui.memory().is_anything_being_dragged();
    let where_to_put_background = ui.painter().add(Shape::Noop);

    let style = ui.visuals().widgets.active;

    if is_being_dragged && accept_dragged {
        if response.hovered() {
            ui.painter().set(
                where_to_put_background,
                epaint::RectShape::stroke(response.rect, style.corner_radius, style.fg_stroke),
            );
        } else {
            ui.painter().set(
                where_to_put_background,
                epaint::RectShape::stroke(response.rect, style.corner_radius, style.bg_stroke),
            );
        }
    }

    if !is_being_dragged && response.hovered() {
        on_drag();
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
        (elem.on_edit)(&new_value);
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
        ..Default::default()
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
        ..Default::default()
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

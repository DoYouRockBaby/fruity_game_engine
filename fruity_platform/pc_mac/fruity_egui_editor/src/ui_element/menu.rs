use crate::ui_element::app::DrawContext;
use crate::ui_element::draw_element;
use egui::menu;
use egui::Button;
use fruity_editor::ui::context::UIContext;
use fruity_editor::ui::elements::menu::MenuBar;
use fruity_editor::ui::elements::menu::MenuSection;
use fruity_editor::ui::hooks::use_read_service;
use fruity_input::input_service::InputService;

pub fn draw_menu_bar<'a>(
    elem: MenuBar,
    ctx: &mut UIContext,
    _ui: &mut egui::Ui,
    draw_ctx: &mut DrawContext,
) {
    egui::TopBottomPanel::top("menu_bar").show(&draw_ctx.platform.context(), |ui| {
        menu::bar(ui, |ui| {
            elem.children
                .into_iter()
                .for_each(|child| draw_element(child, ctx, ui, draw_ctx));
        });
    });
}

pub fn draw_menu_section<'a>(
    elem: MenuSection,
    ctx: &mut UIContext,
    ui: &mut egui::Ui,
    _draw_ctx: &mut DrawContext,
) {
    // Draw menu
    menu::menu(ui, elem.label, {
        let items = elem.items.clone();
        |ui| {
            items.into_iter().for_each({
                |item| {
                    let enabled = match item.options.is_enabled {
                        Some(is_enabled) => is_enabled(ctx),
                        None => true,
                    };

                    if ui.add_enabled(enabled, Button::new(item.label)).clicked() {
                        (item.action)(ctx);
                    }
                }
            });
        }
    });

    // Handle shortcuts
    elem.items.into_iter().for_each({
        |item| {
            let enabled = match item.options.is_enabled {
                Some(is_enabled) => is_enabled(ctx),
                None => true,
            };

            if enabled {
                if let Some(shortcut) = &item.options.shortcut {
                    let input_service = use_read_service::<InputService>(ctx);
                    if input_service.is_keyboard_pressed_this_frame(shortcut) {
                        (item.action)(ctx);
                    }
                }
            }
        }
    });
}

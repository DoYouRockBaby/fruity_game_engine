use crate::ui_element::app::DrawContext;
use crate::ui_element::draw_element;
use egui::menu;
use egui::Button;
use fruity_editor::hooks::topo;
use fruity_editor::hooks::use_global;
use fruity_editor::state::world::WorldState;
use fruity_editor::ui_element::menu::MenuBar;
use fruity_editor::ui_element::menu::MenuSection;
use fruity_input::input_service::InputService;

#[topo::nested]
pub fn draw_menu_bar<'a>(elem: MenuBar, _ui: &mut egui::Ui, ctx: &mut DrawContext) {
    egui::TopBottomPanel::top("menu_bar").show(&ctx.platform.context(), |ui| {
        menu::bar(ui, |ui| {
            elem.children
                .into_iter()
                .for_each(|child| draw_element(child, ui, ctx));
        });
    });
}

#[topo::nested]
pub fn draw_menu_section<'a>(elem: MenuSection, ui: &mut egui::Ui, _ctx: &mut DrawContext) {
    // Draw menu
    menu::menu(ui, elem.label, {
        let items = elem.items.clone();
        |ui| {
            items.into_iter().for_each({
                |item| {
                    let enabled = match item.options.is_enabled {
                        Some(is_enabled) => is_enabled(),
                        None => true,
                    };

                    if ui.add_enabled(enabled, Button::new(item.label)).clicked() {
                        (item.on_click)();
                    }
                }
            });
        }
    });

    // Handle shortcuts
    elem.items.into_iter().for_each({
        |item| {
            let enabled = match item.options.is_enabled {
                Some(is_enabled) => is_enabled(),
                None => true,
            };

            if enabled {
                if let Some(shortcut) = &item.options.shortcut {
                    let world_state = use_global::<WorldState>();
                    let resource_container = world_state.resource_container.clone();

                    let input_service = resource_container.require::<InputService>();
                    let input_service = input_service.read();
                    if input_service.is_keyboard_pressed_this_frame(shortcut) {
                        (item.on_click)();
                    }
                }
            }
        }
    });
}

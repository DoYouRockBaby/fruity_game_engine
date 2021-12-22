use fruity_editor::ui_element::menu::MenuItem;
use std::thread::spawn;

#[derive(Debug)]
pub struct SecondaryActionState {
    below_widget: Option<egui::Response>,
    secondary_actions: Vec<MenuItem>,
}

impl Default for SecondaryActionState {
    fn default() -> Self {
        Self {
            below_widget: None,
            secondary_actions: Vec::new(),
        }
    }
}

impl SecondaryActionState {
    pub fn draw_secondary_actions(&self, ui: &mut egui::Ui) {
        if let Some(below_widget) = &self.below_widget {
            egui::popup::popup_below_widget(
                ui,
                egui::Id::new("secondary_actions"),
                &below_widget,
                |ui| {
                    ui.vertical(|ui| {
                        self.secondary_actions.iter().for_each(|secondary_action| {
                            if ui
                                .small_button(secondary_action.label.to_string())
                                .clicked()
                            {
                                let on_click = secondary_action.on_click.clone();
                                spawn(move || {
                                    on_click();
                                });
                            }
                        });
                    });
                },
            );
        }
    }

    pub fn display_secondary_actions(
        &mut self,
        ui: &mut egui::Ui,
        below_widget: egui::Response,
        secondary_actions: Vec<MenuItem>,
    ) {
        self.below_widget = Some(below_widget);
        self.secondary_actions = secondary_actions;
        ui.memory().open_popup(egui::Id::new("secondary_actions"));
    }
}

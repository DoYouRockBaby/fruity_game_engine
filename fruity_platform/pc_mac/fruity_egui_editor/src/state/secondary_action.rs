use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_editor::editor_menu_service::MenuItem;
use fruity_editor::ui::context::UIContext;

#[derive(Debug, FruityAny)]
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
    pub fn draw_secondary_actions(&self, ctx: &UIContext, ui: &mut egui::Ui) {
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
                                let on_click = secondary_action.action.clone();
                                on_click(ctx);
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

// TODO
impl IntrospectObject for SecondaryActionState {
    fn get_class_name(&self) -> String {
        "SecondaryActionState".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for SecondaryActionState {}

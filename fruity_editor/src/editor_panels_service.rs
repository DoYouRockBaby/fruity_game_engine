use crate::ui::context::UIContext;
use crate::ui::elements::pane::UIPaneSide;
use crate::ui::elements::UIElement;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use std::fmt::Debug;
use std::sync::Arc;

pub struct PanelItem {
    pub label: String,
    pub default_side: UIPaneSide,
    pub renderer: Arc<dyn Fn(&mut UIContext) -> UIElement + Send + Sync>,
}

#[derive(FruityAny)]
pub struct EditorPanelsService {
    panels: Vec<PanelItem>,
}

impl EditorPanelsService {
    pub fn new(_resource_container: ResourceContainer) -> Self {
        Self { panels: Vec::new() }
    }

    pub fn add_panel(
        &mut self,
        label: &str,
        default_side: UIPaneSide,
        renderer: impl Fn(&mut UIContext) -> UIElement + Send + Sync + 'static,
    ) {
        self.panels.push(PanelItem {
            label: label.to_string(),
            default_side,
            renderer: Arc::new(renderer),
        });
    }

    pub fn iter_panels(&self) -> impl Iterator<Item = &PanelItem> {
        self.panels.iter()
    }
}

impl Debug for EditorPanelsService {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

// TODO: Complete that
impl IntrospectObject for EditorPanelsService {
    fn get_class_name(&self) -> String {
        "EditorPanelsService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for EditorPanelsService {}

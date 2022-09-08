use crate::ui::context::UIContext;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct MenuItemOptions {
    pub is_enabled: Option<Arc<dyn Fn(&UIContext) -> bool + Send + Sync>>,
    pub shortcut: Option<String>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Section {
    pub order: usize,
    pub label: String,
}

#[derive(Clone)]
pub struct MenuItem {
    pub label: String,
    pub action: Arc<dyn Fn(&UIContext) + Send + Sync>,
    pub options: MenuItemOptions,
}

impl Debug for MenuItem {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

#[derive(FruityAny)]
pub struct EditorMenuService {
    sections: BTreeMap<Section, Vec<MenuItem>>,
}

impl EditorMenuService {
    pub fn new(_resource_container: ResourceContainer) -> Self {
        Self {
            sections: BTreeMap::new(),
        }
    }

    pub fn add_section(&mut self, label: &str, order: usize) {
        let section = Section {
            label: label.to_string(),
            order,
        };

        // If a previous sections exists, we keep it's item
        let items = if let Some(previous_items) = self.sections.remove(&section) {
            previous_items
        } else {
            Vec::new()
        };

        self.sections.insert(section, items);
    }

    pub fn add_menu(
        &mut self,
        label: &str,
        section_label: &str,
        action: impl Fn(&UIContext) + Send + Sync + 'static,
        options: MenuItemOptions,
    ) {
        // Get or create the menu section
        let section_items = if let Some(section_items) = self
            .sections
            .iter_mut()
            .find(|(section, _)| section.label == section_label)
        {
            section_items.1
        } else {
            self.add_section(section_label, usize::MAX);
            self.sections
                .iter_mut()
                .find(|(section, _)| section.label == section_label)
                .unwrap()
                .1
        };

        section_items.push(MenuItem {
            label: label.to_string(),
            action: Arc::new(action),
            options,
        });
    }

    pub fn iter_sections(&self) -> impl Iterator<Item = (String, Vec<MenuItem>)> + '_ {
        self.sections.iter().map(|(section, items)| {
            (
                section.label.to_string(),
                items
                    .iter()
                    .map(|menu_item| menu_item.clone())
                    .collect::<Vec<_>>(),
            )
        })
    }
}

impl Debug for EditorMenuService {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

// TODO: Complete that
impl IntrospectObject for EditorMenuService {
    fn get_class_name(&self) -> String {
        "EditorMenuService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for EditorMenuService {}

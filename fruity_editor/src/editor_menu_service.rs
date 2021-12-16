use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::Arc;

struct Section {
    label: String,
    order: usize,
}

impl PartialEq for Section {
    fn eq(&self, rhs: &Self) -> bool {
        self.label == rhs.label
    }
}

impl Eq for Section {}

impl PartialOrd for Section {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.order.partial_cmp(&rhs.order)
    }
}

impl Ord for Section {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.order.cmp(&rhs.order)
    }
}

struct MenuItem {
    label: String,
    action: Arc<dyn Fn() + Send + Sync>,
}

#[derive(FruityAny)]
pub struct EditorMenuService {
    sections: BTreeMap<Section, Vec<MenuItem>>,
}

impl EditorMenuService {
    pub fn new(_resource_container: Arc<ResourceContainer>) -> Self {
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
        action: impl Fn() + Send + Sync + 'static,
    ) {
        let section = Section {
            label: label.to_string(),
            order: usize::MAX,
        };

        // Get or create the menu section
        let section_items = if let Some(section_items) = self.sections.get_mut(&section) {
            section_items
        } else {
            self.add_section(section_label, usize::MAX);
            self.sections.get_mut(&section).unwrap()
        };

        section_items.push(MenuItem {
            label: label.to_string(),
            action: Arc::new(action),
        });
    }

    pub fn iter_sections(
        &self,
    ) -> impl Iterator<Item = (String, Vec<(String, Arc<dyn Fn() + Send + Sync>)>)> + '_ {
        self.sections.iter().map(|(section, items)| {
            (
                section.label.to_string(),
                items
                    .iter()
                    .map(|menu_item| (menu_item.label.to_string(), menu_item.action.clone()))
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

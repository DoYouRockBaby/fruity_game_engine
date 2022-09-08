use crate::dialog_service::DialogService;
use crate::state::inspector::InspectorState;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::yaml::serialize_yaml;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity::entity_service::EntityServiceSnapshot;
use fruity_ecs::system::system_service::SystemService;
use std::fs::File;

#[derive(Debug, FruityAny)]
pub struct SceneState {
    resource_container: ResourceContainer,
    entity_service: ResourceReference<EntityService>,
    system_service: ResourceReference<SystemService>,
    inspector_state: ResourceReference<InspectorState>,
    snapshot: Option<EntityServiceSnapshot>,
    current_filepath: Option<String>,
}

impl SceneState {
    pub fn new(resource_container: ResourceContainer) -> Self {
        Self {
            resource_container: resource_container.clone(),
            entity_service: resource_container.require::<EntityService>(),
            system_service: resource_container.require::<SystemService>(),
            inspector_state: resource_container.require::<InspectorState>(),
            snapshot: None,
            current_filepath: None,
        }
    }

    pub fn run(&mut self) {
        let mut inspector_state = self.inspector_state.write();

        let entity_service = self.entity_service.read();
        self.snapshot = Some(entity_service.snapshot());
        inspector_state.unselect();
        entity_service.restore(self.snapshot.as_ref().unwrap());
        std::mem::drop(entity_service);

        let system_service = self.system_service.read();
        system_service.set_paused(false);
    }

    pub fn pause(&mut self) {
        let system_service = self.system_service.read();
        system_service.set_paused(true);
    }

    pub fn stop(&mut self) {
        let mut inspector_state = self.inspector_state.write();
        let entity_service = self.entity_service.read();

        let system_service = self.system_service.read();
        entity_service.restore(self.snapshot.as_ref().unwrap());
        inspector_state.unselect();
        system_service.set_paused(true);
        self.snapshot = None;
    }

    pub fn is_running(&self) -> bool {
        let system_service = self.system_service.read();
        !system_service.is_paused()
    }

    pub fn can_stop(&self) -> bool {
        self.snapshot.is_some()
    }

    pub fn open(&mut self) {
        let dialog_service = self.resource_container.require::<dyn DialogService>();
        let dialog_service = dialog_service.read();

        if let Some(filepath) = dialog_service.open(&["*"]) {
            let mut inspector_state = self.inspector_state.write();
            let entity_service = self.entity_service.read();
            let system_service = self.system_service.read();

            entity_service.restore_from_file(&filepath);
            system_service.set_paused(true);
            inspector_state.unselect();
            self.current_filepath = Some(filepath);
        }
    }

    pub fn save(&mut self) {
        if let Some(filepath) = &self.current_filepath {
            if let Ok(mut writer) = File::create(&filepath) {
                let entity_service = self.entity_service.read();
                let snapshot = entity_service.snapshot();

                if let Ok(_) = serialize_yaml(&mut writer, &snapshot.0) {
                    self.current_filepath = Some(filepath.clone());
                } else {
                }
            }
        } else {
            self.save_as()
        }
    }

    pub fn can_save(&self) -> bool {
        if let Some(_) = &self.current_filepath {
            true
        } else {
            false
        }
    }

    pub fn save_as(&mut self) {
        let dialog_service = self.resource_container.require::<dyn DialogService>();
        let dialog_service = dialog_service.read();

        if let Some(filepath) = dialog_service.save("scene.frsc", &["frsc"]) {
            if let Ok(mut writer) = File::create(&filepath) {
                let entity_service = self.entity_service.read();
                let snapshot = entity_service.snapshot();

                if let Ok(_) = serialize_yaml(&mut writer, &snapshot.0) {
                    self.current_filepath = Some(filepath);
                } else {
                }
            }
        }
    }
}

// TODO
impl IntrospectObject for SceneState {
    fn get_class_name(&self) -> String {
        "SceneState".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for SceneState {}

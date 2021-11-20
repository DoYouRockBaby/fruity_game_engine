use crate::dialog_service::DialogService;
use crate::hooks::use_global;
use crate::state::inspector::InspectorState;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::yaml::deserialize_yaml;
use fruity_core::serialize::yaml::serialize_yaml;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity::entity_service::EntityServiceSnapshot;
use fruity_ecs::system::system_service::SystemService;
use std::fs::File;
use std::sync::Arc;

#[derive(Debug)]
pub struct SceneState {
    resource_container: Arc<ResourceContainer>,
    entity_service: ResourceReference<EntityService>,
    system_service: ResourceReference<SystemService>,
    snapshot: Option<EntityServiceSnapshot>,
    current_filepath: Option<String>,
}

impl SceneState {
    pub fn new(resource_container: Arc<ResourceContainer>) -> Self {
        Self {
            resource_container: resource_container.clone(),
            entity_service: resource_container.require::<EntityService>(),
            system_service: resource_container.require::<SystemService>(),
            snapshot: None,
            current_filepath: None,
        }
    }

    pub fn run(&mut self) {
        let inspector_state = use_global::<InspectorState>();

        let entity_service = self.entity_service.read();
        self.snapshot = Some(entity_service.snapshot());
        inspector_state.unselect();
        entity_service.restore(self.snapshot.as_ref().unwrap());
        std::mem::drop(entity_service);

        let system_service = self.system_service.read();
        system_service.run_begin();
        system_service.set_paused(false);
    }

    pub fn pause(&mut self) {
        let system_service = self.system_service.read();
        system_service.set_paused(true);
    }

    pub fn stop(&mut self) {
        let inspector_state = use_global::<InspectorState>();
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

        if let Some(filepath) = dialog_service.open(&["*.frsc"]) {
            if let Ok(mut reader) = File::open(&filepath) {
                if let Some(snapshot) = deserialize_yaml(&mut reader) {
                    let inspector_state = use_global::<InspectorState>();
                    let entity_service = self.entity_service.read();
                    let system_service = self.system_service.read();

                    entity_service.restore(&EntityServiceSnapshot(snapshot));
                    system_service.set_paused(true);
                    inspector_state.unselect();
                    self.current_filepath = Some(filepath);
                } else {
                }
            }
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

    pub fn save_as(&mut self) {
        let dialog_service = self.resource_container.require::<dyn DialogService>();
        let dialog_service = dialog_service.read();

        if let Some(filepath) = dialog_service.save(&["frsc"]) {
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

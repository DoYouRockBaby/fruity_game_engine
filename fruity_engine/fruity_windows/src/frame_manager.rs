use crate::WindowsManager;
use fruity_any::*;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::service::utils::cast_service;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use fruity_introspect::SetterCaller;
use std::any::TypeId;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Instant;

#[derive(FruityAny)]
pub struct FrameManager {
    last_frame_instant: Instant,
    delta: f32,
}

impl Debug for FrameManager {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl FrameManager {
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> FrameManager {
        let service_manager_reader = service_manager.read().unwrap();
        let windows_manager = service_manager_reader.read::<WindowsManager>();

        let service_manager_2 = service_manager.clone();
        windows_manager.on_enter_loop.add_observer(move |_| {
            let service_manager = service_manager_2.read().unwrap();
            let mut frame_manager = service_manager.write::<FrameManager>();
            frame_manager.begin_frame();
        });

        let service_manager = service_manager.clone();
        windows_manager.on_start_update.add_observer(move |_| {
            let service_manager = service_manager.read().unwrap();
            let mut frame_manager = service_manager.write::<FrameManager>();
            frame_manager.begin_frame();
        });

        FrameManager {
            delta: 0.0,
            last_frame_instant: Instant::now(),
        }
    }

    pub fn get_delta(&self) -> f32 {
        self.delta
    }

    fn begin_frame(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame_instant);

        self.delta = delta.as_secs_f32();
        self.last_frame_instant = now;
    }
}

impl IntrospectObject for FrameManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![MethodInfo {
            name: "get_delta".to_string(),
            call: MethodCaller::Const(Arc::new(|this, _args| {
                let this = cast_service::<FrameManager>(this);
                let result = this.get_delta();
                Ok(Some(Serialized::F32(result)))
            })),
        }]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![FieldInfo {
            name: "delta".to_string(),
            ty: TypeId::of::<f32>(),
            getter: Arc::new(|this| this.downcast_ref::<FrameManager>().unwrap().delta.into()),
            setter: SetterCaller::None,
        }]
    }
}

impl Service for FrameManager {}

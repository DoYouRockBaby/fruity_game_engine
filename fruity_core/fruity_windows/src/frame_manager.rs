use fruity_any::*;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::utils::cast_introspect_ref;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use fruity_introspect::SetterCaller;
use std::any::TypeId;
use std::fmt::Debug;
use std::sync::Arc;
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
    pub fn new(_resource_manager: Arc<ResourceManager>) -> FrameManager {
        FrameManager {
            delta: 0.0,
            last_frame_instant: Instant::now(),
        }
    }

    pub fn get_delta(&self) -> f32 {
        self.delta
    }

    pub fn begin_frame(&mut self) {
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
                let this = cast_introspect_ref::<FrameManager>(this);
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

impl Resource for FrameManager {}

use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::utils::introspect::cast_introspect_ref;
use std::any::TypeId;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Instant;

#[derive(FruityAny)]
pub struct FrameService {
    last_frame_instant: Instant,
    delta: f32,
}

impl Debug for FrameService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl FrameService {
    pub fn new(_resource_container: Arc<ResourceContainer>) -> FrameService {
        FrameService {
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

impl IntrospectObject for FrameService {
    fn get_class_name(&self) -> String {
        "FrameService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![MethodInfo {
            name: "get_delta".to_string(),
            call: MethodCaller::Const(Arc::new(|this, _args| {
                let this = cast_introspect_ref::<FrameService>(this);
                let result = this.get_delta();
                Ok(Some(Serialized::F32(result)))
            })),
        }]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![FieldInfo {
            name: "delta".to_string(),
            ty: TypeId::of::<f32>(),
            serializable: false,
            getter: Arc::new(|this| this.downcast_ref::<FrameService>().unwrap().delta.into()),
            setter: SetterCaller::None,
        }]
    }
}

impl Resource for FrameService {}

use fruity_any_derive::*;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service::Service;
use fruity_ecs::service::service_rwlock::ServiceRwLock;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodInfo;
use fruity_windows::windows_manager::WindowsManager;

#[derive(Debug, FruityAny)]
pub struct GraphicsManager {}

impl GraphicsManager {
    pub fn new(windows_manager: ServiceRwLock<WindowsManager>) -> GraphicsManager {
        windows_manager.on_init.add_observer(|window| {
            let size = window.inner_size();

            // The instance is a handle to our GPU
            // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
            let instance = wgpu::Instance::new(wgpu::Backends::all());
            let surface = unsafe { instance.create_surface(window) };
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .unwrap();
        });

        GraphicsManager {}
    }
}

impl IntrospectMethods<Serialized> for GraphicsManager {
    fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
        vec![
            /*MethodInfo {
                name: "run".to_string(),
                args: vec![],
                return_type: None,
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_service::<WindowsManager>(this);
                    this.run();
                    Ok(None)
                })),
            },*/
        ]
    }
}

impl Service for GraphicsManager {}

use fruity_any_derive::*;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service::Service;
use fruity_ecs::world::World;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodInfo;
use fruity_windows::windows_manager::WindowsManager;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;
use winit::window::Window;

#[derive(Debug)]
struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

#[derive(Debug, FruityAny)]
pub struct GraphicsManager {
    state: Option<State>,
}

impl GraphicsManager {
    pub fn new(world: &World) -> GraphicsManager {
        let service_manager = world.service_manager.read().unwrap();
        let windows_manager = service_manager.get::<WindowsManager>().unwrap();
        let windows_manager = windows_manager.read().unwrap();

        let service_manager = world.service_manager.clone();
        windows_manager.on_init.add_observer(move |window| {
            let service_manager = service_manager.read().unwrap();
            let graphics_manager = service_manager.get::<GraphicsManager>().unwrap();
            let mut graphics_manager = graphics_manager.write().unwrap();

            graphics_manager.initialize(window.clone());
        });

        let service_manager = world.service_manager.clone();
        windows_manager.on_draw.add_observer(move |_| {
            let service_manager = service_manager.read().unwrap();
            let graphics_manager = service_manager.get::<GraphicsManager>().unwrap();
            let mut graphics_manager = graphics_manager.write().unwrap();

            graphics_manager.draw();
        });

        let service_manager = world.service_manager.clone();
        windows_manager
            .on_resize
            .add_observer(move |(width, height)| {
                let service_manager = service_manager.read().unwrap();
                let graphics_manager = service_manager.get::<GraphicsManager>().unwrap();
                let mut graphics_manager = graphics_manager.write().unwrap();

                graphics_manager.resize(*width, *height);
            });

        GraphicsManager { state: None }
    }

    pub fn initialize(&mut self, window: Arc<RwLock<Window>>) {
        let future = async {
            let window = window.read().unwrap();

            // The instance is a handle to our GPU
            // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
            let instance = wgpu::Instance::new(wgpu::Backends::all());
            let surface = unsafe { instance.create_surface(window.deref()) };
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .unwrap();

            // Create the device and queue
            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::default(),
                        label: None,
                    },
                    None, // Trace path
                )
                .await
                .unwrap();

            // Base configuration for the surface
            let size = window.inner_size();
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface.get_preferred_format(&adapter).unwrap(),
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Fifo,
            };

            surface.configure(&device, &config);

            // Update state
            self.state = Some(State {
                surface,
                device,
                queue,
                config,
            });
        };

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(future);
    }

    pub fn draw(&mut self) {
        if let Some(state) = &mut self.state {
            let output = state.surface.get_current_texture().unwrap();

            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder =
                state
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });
            }

            // submit will accept anything that implements IntoIter
            state.queue.submit(std::iter::once(encoder.finish()));
            output.present();
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if let Some(state) = &mut self.state {
            state.config.width = width as u32;
            state.config.height = height as u32;

            state.surface.configure(&state.device, &state.config)
        }
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

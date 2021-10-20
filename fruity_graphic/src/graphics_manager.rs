use fruity_any::*;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::service::service::Service;
use fruity_core::world::World;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodInfo;
use fruity_windows::windows_manager::WindowsManager;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;
use winit::window::Window;

#[derive(Debug)]
pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub rendering_view: wgpu::TextureView,
}

#[derive(Debug, FruityAnySyncSend)]
pub struct GraphicsManager {
    state: Option<State>,
    current_output: Option<wgpu::SurfaceTexture>,
    current_encoder: Option<RwLock<wgpu::CommandEncoder>>,
}

impl GraphicsManager {
    pub fn new(world: &World) -> GraphicsManager {
        let service_manager = world.service_manager.read().unwrap();
        let windows_manager = service_manager.read::<WindowsManager>();

        // Subscribe to windows observer to proceed the graphics when it's neededs
        let service_manager = world.service_manager.clone();
        windows_manager.on_windows_creation.add_observer(move |_| {
            let service_manager = service_manager.read().unwrap();
            let mut graphics_manager = service_manager.write::<GraphicsManager>();
            let windows_manager = service_manager.read::<WindowsManager>();
            let window = windows_manager.get_window().unwrap();

            graphics_manager.initialize(window.clone());
        });

        let service_manager = world.service_manager.clone();
        windows_manager.on_start_update.add_observer(move |_| {
            let service_manager = service_manager.read().unwrap();
            let mut graphics_manager = service_manager.write::<GraphicsManager>();

            graphics_manager.start_draw();
        });

        let service_manager = world.service_manager.clone();
        windows_manager.on_end_update.add_observer(move |_| {
            let service_manager = service_manager.read().unwrap();
            let mut graphics_manager = service_manager.write::<GraphicsManager>();

            graphics_manager.end_draw();
        });

        let service_manager = world.service_manager.clone();
        windows_manager
            .on_resize
            .add_observer(move |(width, height)| {
                let service_manager = service_manager.read().unwrap();
                let mut graphics_manager = service_manager.write::<GraphicsManager>();

                graphics_manager.resize(*width, *height);
            });

        GraphicsManager {
            state: None,
            current_encoder: None,
            current_output: None,
        }
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

            // Get the texture view where the scene will be rendered
            let output = surface.get_current_texture().unwrap();
            let rendering_view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            // Update state
            self.state = Some(State {
                surface,
                device,
                queue,
                config,
                rendering_view,
            });
        };

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(future);
    }

    pub fn start_draw(&mut self) {
        if let Some(state) = &mut self.state {
            // Get the texture view where the scene will be rendered
            let output = state.surface.get_current_texture().unwrap();
            let rendering_view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            self.current_output = Some(output);
            state.rendering_view = rendering_view;

            let encoder = state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            // Store the handles about this frame
            self.current_encoder = Some(RwLock::new(encoder))
        }
    }

    pub fn end_draw(&mut self) {
        let encoder = if let Some(encoder) = self.current_encoder.take() {
            encoder.into_inner().unwrap()
        } else {
            return;
        };

        let output = if let Some(output) = self.current_output.take() {
            output
        } else {
            return;
        };

        let queue = self.get_queue().unwrap();

        // submit will accept anything that implements IntoIter
        queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if let Some(state) = &mut self.state {
            state.config.width = width as u32;
            state.config.height = height as u32;

            state.surface.configure(&state.device, &state.config)
        }
    }

    pub fn get_device(&self) -> Option<&wgpu::Device> {
        self.state.as_ref().map(|state| &state.device)
    }

    pub fn get_queue(&self) -> Option<&wgpu::Queue> {
        self.state.as_ref().map(|state| &state.queue)
    }

    pub fn get_surface(&self) -> Option<&wgpu::Surface> {
        self.state.as_ref().map(|state| &state.surface)
    }

    pub fn get_config(&self) -> Option<&wgpu::SurfaceConfiguration> {
        self.state.as_ref().map(|state| &state.config)
    }

    pub fn get_rendering_view(&self) -> Option<&wgpu::TextureView> {
        self.state.as_ref().map(|state| &state.rendering_view)
    }

    pub fn get_encoder(&self) -> Option<&RwLock<wgpu::CommandEncoder>> {
        self.current_encoder.as_ref()
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

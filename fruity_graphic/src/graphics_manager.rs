use fruity_any::*;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service::Service;
use fruity_ecs::world::World;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodInfo;
use fruity_windows::windows_manager::WindowsManager;
use std::collections::VecDeque;
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
    pub render_pipeline: wgpu::RenderPipeline,
}

type RenderingAction = dyn Fn(&mut wgpu::RenderPass, &State) + Send + Sync;

pub struct FrameState {
    encoder: wgpu::CommandEncoder,
    rendering_queue: VecDeque<Box<RenderingAction>>,
    rendering_view: wgpu::TextureView,
}

impl Debug for FrameState {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

#[derive(Debug, FruityAnySyncSend)]
pub struct GraphicsManager {
    state: Option<State>,
    frame_state: Option<FrameState>,
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
            frame_state: None,
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

            // Create the main render pipeline
            let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl("assets/shader.wgsl".into()),
            });

            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "main", // 1.
                    buffers: &[],        // 2.
                },
                fragment: Some(wgpu::FragmentState {
                    // 3.
                    module: &shader,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        // 4.
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw, // 2.
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLAMPING
                    clamp_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None, // 1.
                multisample: wgpu::MultisampleState {
                    count: 1,                         // 2.
                    mask: !0,                         // 3.
                    alpha_to_coverage_enabled: false, // 4.
                },
            });

            // Update state
            self.state = Some(State {
                surface,
                device,
                queue,
                config,
                render_pipeline,
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
            let output = state.surface.get_current_texture().unwrap();

            // Get the texture view where the scene will be rendered
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let encoder = state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            // Store the handles about this frame
            self.frame_state = Some(FrameState {
                encoder,
                rendering_queue: VecDeque::new(),
                rendering_view: view,
            })
        }
    }

    pub fn end_draw(&mut self) {
        let state = if let Some(state) = &mut self.state {
            state
        } else {
            return;
        };

        let mut frame_state = if let Some(frame_state) = self.frame_state.take() {
            frame_state
        } else {
            return;
        };

        // Proceed the main render pass
        {
            let mut render_pass = {
                frame_state
                    .encoder
                    .begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &frame_state.rendering_view,
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
                    })
            };

            while let Some(action) = frame_state.rendering_queue.pop_front() {
                action(&mut render_pass, state);
            }
        }

        // submit will accept anything that implements IntoIter
        let output = state.surface.get_current_texture().unwrap();
        state
            .queue
            .submit(std::iter::once(frame_state.encoder.finish()));
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

    pub fn get_render_pipeline(&self) -> Option<&wgpu::RenderPipeline> {
        self.state.as_ref().map(|state| &state.render_pipeline)
    }

    pub fn push_rendering_action<'a, F>(&mut self, action: F)
    where
        F: Fn(&mut wgpu::RenderPass, &State) + Send + Sync + 'static,
    {
        if let Some(frame_state) = &mut self.frame_state.take() {
            frame_state.rendering_queue.push_back(Box::new(action));
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

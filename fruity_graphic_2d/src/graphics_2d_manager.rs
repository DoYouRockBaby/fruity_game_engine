use fruity_any::*;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service::Service;
use fruity_ecs::service::service_rwlock::ServiceRwLock;
use fruity_ecs::world::World;
use fruity_graphic::graphics_manager::GraphicsManager;
use fruity_graphic::resources::texture_resource::TextureResource;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodInfo;
use std::fs::File;
use std::io::Read;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2, // NEW!
                },
            ],
        }
    }
}

#[derive(Debug, FruityAnySyncSend)]
pub struct Graphics2dManager {
    graphics_manager: ServiceRwLock<GraphicsManager>,
}

impl Graphics2dManager {
    pub fn new(world: &World) -> Graphics2dManager {
        let service_manager = world.service_manager.read().unwrap();
        let graphics_manager = service_manager.get::<GraphicsManager>().unwrap();

        Graphics2dManager { graphics_manager }
    }

    pub fn draw_texture(&self, texture: &TextureResource, x: f32, y: f32, w: f32, h: f32) {
        let graphics_manager = self.graphics_manager.read().unwrap();

        let texture_view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let device = graphics_manager.get_device().unwrap();
        let config = graphics_manager.get_config().unwrap();
        let rendering_view = graphics_manager.get_rendering_view().unwrap();

        // Create the main render pipeline
        let mut buffer = String::new();
        let mut settings_file = File::open("assets/shader.wgsl").unwrap();
        if let Err(err) = settings_file.read_to_string(&mut buffer) {
            log::error!("{}", err.to_string());
            return;
        }

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(buffer.into()),
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: false,
                            filtering: true,
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLAMPING
                clamp_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        let vertices: &[Vertex] = &[
            Vertex {
                position: [x, y, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [x + w, y, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [x + w, y + h, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [x, y + h, 0.0],
                tex_coords: [0.0, 0.0],
            },
        ];

        let indices: &[u16] = &[0, 1, 2, 3, 0, 2, /* padding */ 0];
        let num_indices = indices.len() as u32;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let mut encoder = graphics_manager.get_encoder().unwrap().write().unwrap();
        let mut render_pass = {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: rendering_view,
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

        render_pass.set_pipeline(&render_pipeline);
        render_pass.set_bind_group(0, &diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..num_indices, 0, 0..1);
    }
}

impl IntrospectMethods<Serialized> for Graphics2dManager {
    fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
        vec![]
    }
}

impl Service for Graphics2dManager {}

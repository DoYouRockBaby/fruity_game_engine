use fruity_any::*;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service::Service;
use fruity_ecs::service::service_rwlock::ServiceRwLock;
use fruity_ecs::world::World;
use fruity_graphic::graphics_manager::GraphicsManager;
use fruity_graphic::texture_resource::TextureResource;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodInfo;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
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
        let mut graphics_manager_writer = self.graphics_manager.write().unwrap();

        let texture_view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let graphics_manager = self.graphics_manager.clone();
        graphics_manager_writer.push_rendering_action(move |render_pass, state| {
            let device = &state.device;
            let config = &state.config;
            let render_pipeline =
                unsafe { &*(&state.render_pipeline as *const _) } as &wgpu::RenderPipeline;

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
                                // This is only for TextureSampleType::Depth
                                comparison: false,
                                // This should be true if the sample_type of the texture is:
                                //     TextureSampleType::Float { filterable: true }
                                // Otherwise you'll get an error.
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

            let vertices: &[Vertex] = &[
                Vertex {
                    position: [x, y, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
                Vertex {
                    position: [x + w, y, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [x + w, y + h, 0.0],
                    color: [0.0, 0.0, 1.0],
                },
                Vertex {
                    position: [x, y + h, 0.0],
                    color: [0.0, 0.0, 1.0],
                },
            ];

            const indices: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, 0];
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

            render_pass.set_pipeline(render_pipeline);
            render_pass.set_bind_group(0, &diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..num_indices, 0, 0..1);
        });
    }
}

impl IntrospectMethods<Serialized> for Graphics2dManager {
    fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
        vec![]
    }
}

impl Service for Graphics2dManager {}

use fruity_any::*;
use fruity_core::resource::resources_manager::ResourceIdentifier;
use fruity_core::resource::resources_manager::ResourcesManager;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::service::service_rwlock::ServiceRwLock;
use fruity_graphic::graphics_manager::GraphicsManager;
use fruity_graphic::math::Color;
use fruity_graphic::math::Matrix4;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::material_resource::Vertex;
use fruity_graphic::resources::shader_resource::ShaderResource;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::sync::Arc;
use std::sync::RwLock;
use wgpu::util::DeviceExt;

#[derive(Debug, FruityAny)]
pub struct Graphics2dManager {
    graphics_manager: ServiceRwLock<GraphicsManager>,
    resource_manager: ServiceRwLock<ResourcesManager>,
}

impl Graphics2dManager {
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> Graphics2dManager {
        let service_manager = service_manager.read().unwrap();
        let graphics_manager = service_manager.get::<GraphicsManager>().unwrap();
        let resource_manager = service_manager.get::<ResourcesManager>().unwrap();

        Graphics2dManager {
            graphics_manager,
            resource_manager,
        }
    }

    pub fn start_rendering(&self, view_proj: Matrix4) {
        let mut graphics_manager = self.graphics_manager.write().unwrap();

        // Create the camera buffer for the camera transform
        graphics_manager.update_camera(view_proj);

        let rendering_view = graphics_manager.get_rendering_view();
        let mut encoder = graphics_manager.get_encoder().unwrap().write().unwrap();

        // Display the background
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
        });
    }

    pub fn draw_square(&self, x: f32, y: f32, w: f32, h: f32, material: &MaterialResource) {
        let graphics_manager = self.graphics_manager.read().unwrap();

        let device = graphics_manager.get_device();
        let rendering_view = graphics_manager.get_rendering_view();

        // Create the main render pipeline
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
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            })
        };

        render_pass.set_pipeline(&material.render_pipeline);
        material.bind_groups.iter().for_each(|(index, bind_group)| {
            render_pass.set_bind_group(*index, &bind_group, &[]);
        });
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..num_indices, 0, 0..1);
    }

    pub fn draw_line(&self, _x1: f32, _y1: f32, _x2: f32, _y2: f32, _width: f32, color: &Color) {
        let graphics_manager = self.graphics_manager.read().unwrap();
        let resource_manager = self.resource_manager.read().unwrap();

        let device = graphics_manager.get_device();
        let rendering_view = graphics_manager.get_rendering_view();

        // Get resources
        let shader = resource_manager
            .get_resource::<ShaderResource>(ResourceIdentifier("Shaders/Draw Line".to_string()))
            .unwrap();

        let material = resource_manager
            .get_resource::<MaterialResource>(ResourceIdentifier("Materials/Draw Line".to_string()))
            .unwrap();

        // Create the main render pipeline
        let vertices: &[Vertex] = &[
            Vertex {
                position: [-1.0, -1.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [-1.0, 1.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
        ];

        let vertex_count = vertices.len() as u32;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        /*let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
                "
                [[block]]
                struct ColorBuffer {
                    color: vec4<f32>;
                };

                struct VertexInput {
                    [[location(0)]] position: vec3<f32>;
                    [[location(1)]] tex_coords: vec2<f32>;
                };

                struct VertexOutput {
                    [[location(0)]] color: vec4<f32>;
                    [[builtin(position)]] position: vec4<f32>;
                };

                [[group(0), binding(0)]]
                var<uniform> color_buffer: ColorBuffer;
                [[stage(vertex)]]
                fn vs_main(model: VertexInput) -> VertexOutput {
                    var out: VertexOutput;
                    out.position = vec4<f32>(model.position, 1.0);
                    out.color = color_buffer.color;
                    return out;
                }
                [[stage(fragment)]]
                fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
                    return in.color;
                }
            ",
            )),
        });*/

        /*let color_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("color_bind_group_layout"),
        });*/

        let color_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Color Buffer"),
            contents: bytemuck::cast_slice(&[color.clone()]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let color_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &shader.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: color_buffer.as_entire_binding(),
            }],
            label: Some("color_bind_group"),
        });

        /*let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
             label: None,
             bind_group_layouts: &[&shader.bind_group_layout],
             push_constant_ranges: &[],
         });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
             label: None,
             layout: Some(&pipeline_layout),
             vertex: wgpu::VertexState {
                 module: &shader,
                 entry_point: "vs_main",
                 buffers: &[wgpu::VertexBufferLayout {
                     array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                     step_mode: wgpu::VertexStepMode::Vertex,
                     attributes: &[
                         wgpu::VertexAttribute {
                             offset: 0,
                             shader_location: 0,
                             format: wgpu::VertexFormat::Float32x3,
                         },
                         wgpu::VertexAttribute {
                             offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                             shader_location: 1,
                             format: wgpu::VertexFormat::Float32x2,
                         },
                     ],
                 }],
             },
             fragment: Some(wgpu::FragmentState {
                 module: &shader,
                 entry_point: "fs_main",
                 targets: &[config.format.into()],
             }),
             primitive: wgpu::PrimitiveState {
                 topology: wgpu::PrimitiveTopology::LineList,
                 front_face: wgpu::FrontFace::Ccw,
                 ..Default::default()
             },
             depth_stencil: None,
             multisample: wgpu::MultisampleState {
                 count: 1,
                 ..Default::default()
             },
         });*/

        let mut encoder = graphics_manager.get_encoder().unwrap().write().unwrap();
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: rendering_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&material.render_pipeline);
        render_pass.set_bind_group(0, &color_bind_group, &[]);
        material.bind_groups.iter().for_each(|(index, bind_group)| {
            render_pass.set_bind_group(*index, &bind_group, &[]);
        });
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..vertex_count, 0..1);
    }
}

impl IntrospectObject for Graphics2dManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Service for Graphics2dManager {}

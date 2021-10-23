use fruity_any::*;
use fruity_core::service::service::Service;
use fruity_core::service::service_rwlock::ServiceRwLock;
use fruity_core::world::World;
use fruity_graphic::graphics_manager::GraphicsManager;
use fruity_graphic::math::Matrix4;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::material_resource::Vertex;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use wgpu::util::DeviceExt;

#[derive(Debug, FruityAny)]
pub struct Graphics2dManager {
    graphics_manager: ServiceRwLock<GraphicsManager>,
}

impl Graphics2dManager {
    pub fn new(world: &World) -> Graphics2dManager {
        let service_manager = world.service_manager.read().unwrap();
        let graphics_manager = service_manager.get::<GraphicsManager>().unwrap();

        Graphics2dManager { graphics_manager }
    }

    pub fn start_rendering(&self, view_proj: Matrix4) {
        let mut graphics_manager = self.graphics_manager.write().unwrap();

        // Create the camera buffer for the camera transform
        graphics_manager.update_camera(view_proj);

        let rendering_view = graphics_manager.get_rendering_view().unwrap();
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

        let device = graphics_manager.get_device().unwrap();
        let rendering_view = graphics_manager.get_rendering_view().unwrap();

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

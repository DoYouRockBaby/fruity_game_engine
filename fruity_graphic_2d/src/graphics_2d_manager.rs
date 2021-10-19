use fruity_any::*;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service::Service;
use fruity_ecs::service::service_rwlock::ServiceRwLock;
use fruity_ecs::world::World;
use fruity_graphic::graphics_manager::GraphicsManager;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::material_resource::Vertex;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodInfo;
use std::fs::File;
use std::io::Read;
use wgpu::util::DeviceExt;

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

    pub fn draw_texture(&self, x: f32, y: f32, w: f32, h: f32, material: &MaterialResource) {
        let graphics_manager = self.graphics_manager.read().unwrap();

        let device = graphics_manager.get_device().unwrap();
        let rendering_view = graphics_manager.get_rendering_view().unwrap();

        // Create the main render pipeline
        let mut buffer = String::new();
        let mut settings_file = File::open("assets/shader.wgsl").unwrap();
        if let Err(err) = settings_file.read_to_string(&mut buffer) {
            log::error!("{}", err.to_string());
            return;
        }

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

        render_pass.set_pipeline(&material.render_pipeline);
        render_pass.set_bind_group(0, &material.bind_group, &[]);
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

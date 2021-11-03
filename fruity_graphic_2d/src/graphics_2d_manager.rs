use crate::Vector2d;
use fruity_any::*;
use fruity_core::resource::resources_manager::ResourceIdentifier;
use fruity_core::resource::resources_manager::ResourcesManager;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::service::service_rwlock::ServiceRwLock;
use fruity_graphic::graphics_manager::GraphicsManager;
use fruity_graphic::math::Color;
use fruity_graphic::math::Matrix4;
use fruity_graphic::resources::material_resource::BufferIdentifier;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::material_resource::Vertex;
use fruity_graphic::resources::shader_resource::ShaderResource;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use fruity_windows::windows_manager::WindowsManager;
use std::sync::Arc;
use std::sync::RwLock;
use wgpu::util::DeviceExt;

#[derive(Debug, FruityAny)]
pub struct Graphics2dManager {
    windows_manager: ServiceRwLock<WindowsManager>,
    graphics_manager: ServiceRwLock<GraphicsManager>,
    resource_manager: ServiceRwLock<ResourcesManager>,
}

impl Graphics2dManager {
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> Graphics2dManager {
        let service_manager = service_manager.read().unwrap();
        let windows_manager = service_manager.get::<WindowsManager>().unwrap();
        let graphics_manager = service_manager.get::<GraphicsManager>().unwrap();
        let resource_manager = service_manager.get::<ResourcesManager>().unwrap();

        Graphics2dManager {
            windows_manager,
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

    pub fn draw_square(&self, pos: Vector2d, size: Vector2d, material: &MaterialResource) {
        let graphics_manager = self.graphics_manager.read().unwrap();

        let device = graphics_manager.get_device();
        let rendering_view = graphics_manager.get_rendering_view();

        // Create the main render pipeline
        let vertices: &[Vertex] = &[
            Vertex {
                position: [pos.x, pos.y, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [pos.x + size.x, pos.y, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [pos.x + size.x, pos.y + size.y, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [pos.x, pos.y + size.y, 0.0],
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

    pub fn draw_line(&self, pos1: Vector2d, pos2: Vector2d, width: u32, color: &Color) {
        let windows_manager = self.windows_manager.read().unwrap();
        let graphics_manager = self.graphics_manager.read().unwrap();
        let resource_manager = self.resource_manager.read().unwrap();

        let queue = graphics_manager.get_queue();
        let device = graphics_manager.get_device();
        let rendering_view = graphics_manager.get_rendering_view();
        let camera_transform = graphics_manager.get_camera_transform().clone();
        let viewport_size = windows_manager.get_size().clone();

        // Get resources
        let material = resource_manager
            .get_resource::<MaterialResource>(ResourceIdentifier("Materials/Draw Line".to_string()))
            .unwrap();

        let shader = resource_manager
            .get_resource::<ShaderResource>(ResourceIdentifier("Shaders/Draw Line".to_string()))
            .unwrap();

        // Calculate the geometry
        let diff = pos2 - pos1;
        let mut thick_vec = diff.normal().normalise() * width as f32;
        thick_vec.x /= viewport_size.0 as f32;
        thick_vec.y /= viewport_size.1 as f32;

        let pos1 = camera_transform * pos1;
        let pos2 = camera_transform * pos2;

        let vertices: &[Vertex] = &[
            Vertex {
                position: [pos1.x - thick_vec.x, pos1.y - thick_vec.y, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [pos1.x + thick_vec.x, pos1.y + thick_vec.y, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [pos2.x + thick_vec.x, pos2.y + thick_vec.y, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [pos2.x - thick_vec.x, pos2.y - thick_vec.y, 0.0],
                tex_coords: [0.0, 0.0],
            },
        ];

        let indices: &[u16] = &[2, 1, 0, 2, 0, 3, /* padding */ 0];
        let num_indices = indices.len() as u32;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

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

        // Update material color
        material.write_buffer(
            &BufferIdentifier(0, 0),
            queue,
            bytemuck::cast_slice(&[color.clone()]),
        );

        let color_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[color.clone()]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Color buffer"),
            layout: &shader.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: color_buffer.as_entire_binding(),
            }],
        });

        // Draw
        // TODO: Find a way to remove that
        let bind_group =
            unsafe { std::mem::transmute::<&wgpu::BindGroup, &wgpu::BindGroup>(&bind_group) };
        render_pass.set_pipeline(&material.render_pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..num_indices, 0, 0..1);
        std::mem::drop(render_pass);
    }

    /// Get the cursor position in the 2D world, take in care the camera transform
    pub fn get_cursor_position(&self) -> Vector2d {
        let windows_manager = self.windows_manager.read().unwrap();
        let graphics_manager = self.graphics_manager.read().unwrap();

        // Get informations from the services
        let cursor_position = windows_manager.get_cursor_position();
        let viewport_size = windows_manager.get_size();
        let camera_transform = graphics_manager.get_camera_transform().clone();
        std::mem::drop(graphics_manager);
        std::mem::drop(windows_manager);

        // Transform the cursor in the engine world (especialy taking care of camera)
        let cursor_pos = Vector2d::new(
            (cursor_position.0 as f32 / viewport_size.0 as f32) * 2.0 - 1.0,
            (cursor_position.1 as f32 / viewport_size.1 as f32) * -2.0 + 1.0,
        );

        // log::debug!("{:?}", cursor_position);

        camera_transform.invert() * cursor_pos
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

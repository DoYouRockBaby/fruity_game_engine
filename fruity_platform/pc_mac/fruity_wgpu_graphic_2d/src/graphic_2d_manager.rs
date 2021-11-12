use fruity_any::*;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_graphic::graphic_manager::GraphicManager;
use fruity_graphic::math::Color;
use fruity_graphic::math::Matrix4;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::shader_resource::ShaderResource;
use fruity_graphic_2d::graphic_2d_manager::Graphic2dManager;
use fruity_graphic_2d::math::vector2d::Vector2d;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use fruity_wgpu_graphic::graphic_manager::WgpuGraphicsManager;
use fruity_wgpu_graphic::resources::material_resource::BufferIdentifier;
use fruity_wgpu_graphic::resources::material_resource::Vertex;
use fruity_wgpu_graphic::resources::material_resource::WgpuMaterialResource;
use fruity_wgpu_graphic::resources::shader_resource::WgpuShaderResource;
use fruity_windows::windows_manager::WindowsManager;
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[derive(Debug, FruityAny)]
pub struct WgpuGraphics2dManager {
    windows_manager: ResourceReference<dyn WindowsManager>,
    graphic_manager: ResourceReference<dyn GraphicManager>,
    resource_manager: Arc<ResourceManager>,
}

impl WgpuGraphics2dManager {
    pub fn new(resource_manager: Arc<ResourceManager>) -> WgpuGraphics2dManager {
        let windows_manager =
            resource_manager.require::<dyn WindowsManager>("windows_manager");
        let graphic_manager =
            resource_manager.require::<dyn GraphicManager>("graphic_manager");

        WgpuGraphics2dManager {
            windows_manager,
            graphic_manager,
            resource_manager,
        }
    }

    /// Get the cursor position in the 2D world, take in care the camera transform
    pub fn get_cursor_position(&self) -> Vector2d {
        let windows_manager = self.windows_manager.read();
        let graphic_manager = self.graphic_manager.read();

        // Get informations from the services
        let cursor_position = windows_manager.get_cursor_position();
        let viewport_size = windows_manager.get_size();
        let camera_transform = graphic_manager.get_camera_transform().clone();
        std::mem::drop(graphic_manager);
        std::mem::drop(windows_manager);

        // Transform the cursor in the engine world (especialy taking care of camera)
        let cursor_pos = Vector2d::new(
            (cursor_position.0 as f32 / viewport_size.0 as f32) * 2.0 - 1.0,
            (cursor_position.1 as f32 / viewport_size.1 as f32) * -2.0 + 1.0,
        );

        camera_transform.invert() * cursor_pos
    }
}

impl Graphic2dManager for WgpuGraphics2dManager {
    fn start_pass(&self, view_proj: Matrix4) {
        let mut graphic_manager = self.graphic_manager.write();
        graphic_manager.update_camera(view_proj);
        graphic_manager.start_pass();
    }

    fn end_pass(&self) {
        let graphic_manager = self.graphic_manager.read();
        graphic_manager.end_pass();
    }

    fn draw_square(
        &self,
        pos: Vector2d,
        size: Vector2d,
        z_index: usize,
        material: ResourceReference<dyn MaterialResource>,
    ) {
        let graphic_manager = self.graphic_manager.read();
        let graphic_manager = graphic_manager.downcast_ref::<WgpuGraphicsManager>();

        let material = material.read();
        let material = material.downcast_ref::<WgpuMaterialResource>();

        let device = graphic_manager.get_device();
        let config = graphic_manager.get_config();

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

        let mut encoder =
            device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                label: Some("draw_square_bundle"),
                color_formats: &[config.format],
                depth_stencil: None,
                sample_count: 1,
            });

        encoder.set_pipeline(&material.render_pipeline);
        material.bind_groups.iter().for_each(|(index, bind_group)| {
            encoder.set_bind_group(*index, &bind_group, &[]);
        });
        encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
        encoder.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        encoder.draw_indexed(0..num_indices, 0, 0..1);
        let bundle = encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some("main"),
        });

        graphic_manager.push_render_bundle(bundle, z_index);
    }

    fn draw_line(&self, pos1: Vector2d, pos2: Vector2d, width: u32, color: Color, z_index: usize) {
        let windows_manager = self.windows_manager.read();
        let graphic_manager = self.graphic_manager.read();
        let graphic_manager = graphic_manager.downcast_ref::<WgpuGraphicsManager>();

        let queue = graphic_manager.get_queue();
        let device = graphic_manager.get_device();
        let config = graphic_manager.get_config();
        let camera_transform = graphic_manager.get_camera_transform().clone();
        let viewport_size = windows_manager.get_size().clone();

        // Get resources
        let material = self
            .resource_manager
            .require::<dyn MaterialResource>("Materials/Draw Line");
        let material = material.read();
        let material = material.downcast_ref::<WgpuMaterialResource>();

        let shader = self
            .resource_manager
            .require::<dyn ShaderResource>("Shaders/Draw Line");
        let shader = shader.read();
        let shader = shader.downcast_ref::<WgpuShaderResource>();

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
        let mut encoder =
            device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                label: None,
                color_formats: &[config.format],
                depth_stencil: None,
                sample_count: 1,
            });

        encoder.set_pipeline(&material.render_pipeline);
        encoder.set_bind_group(0, &bind_group, &[]);
        encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
        encoder.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        encoder.draw_indexed(0..num_indices, 0, 0..1);
        let bundle = encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some("main"),
        });

        graphic_manager.push_render_bundle(bundle, z_index);
    }
}

impl IntrospectObject for WgpuGraphics2dManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for WgpuGraphics2dManager {}

use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::Color;
use fruity_graphic::math::Matrix4;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::shader_resource::ShaderResource;
use fruity_graphic_2d::graphic_2d_service::Graphic2dService;
use fruity_graphic_2d::math::vector2d::Vector2d;
use fruity_wgpu_graphic::graphic_service::WgpuGraphicManager;
use fruity_wgpu_graphic::resources::material_resource::BufferIdentifier;
use fruity_wgpu_graphic::resources::material_resource::Vertex;
use fruity_wgpu_graphic::resources::material_resource::WgpuMaterialResource;
use fruity_wgpu_graphic::resources::shader_resource::WgpuShaderResource;
use fruity_windows::window_service::WindowService;
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[derive(Debug, FruityAny)]
pub struct WgpuGraphic2dManager {
    window_service: ResourceReference<dyn WindowService>,
    graphic_service: ResourceReference<dyn GraphicService>,
    resource_container: Arc<ResourceContainer>,
}

impl WgpuGraphic2dManager {
    pub fn new(resource_container: Arc<ResourceContainer>) -> WgpuGraphic2dManager {
        let window_service = resource_container.require::<dyn WindowService>();
        let graphic_service = resource_container.require::<dyn GraphicService>();

        WgpuGraphic2dManager {
            window_service,
            graphic_service,
            resource_container,
        }
    }
}

impl Graphic2dService for WgpuGraphic2dManager {
    fn start_pass(&self, view_proj: Matrix4) {
        let mut graphic_service = self.graphic_service.write();
        graphic_service.update_camera(view_proj);
        graphic_service.start_pass();
    }

    fn end_pass(&self) {
        let graphic_service = self.graphic_service.read();
        graphic_service.end_pass();
    }

    fn draw_square(
        &self,
        pos: Vector2d,
        size: Vector2d,
        z_index: usize,
        material: ResourceReference<dyn MaterialResource>,
    ) {
        let graphic_service = self.graphic_service.read();
        let graphic_service = graphic_service.downcast_ref::<WgpuGraphicManager>();

        let material = material.read();
        let material = material.downcast_ref::<WgpuMaterialResource>();

        let device = graphic_service.get_device();
        let config = graphic_service.get_config();

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

        graphic_service.push_render_bundle(bundle, z_index);
    }

    fn draw_line(&self, pos1: Vector2d, pos2: Vector2d, width: u32, color: Color, z_index: usize) {
        let window_service = self.window_service.read();
        let graphic_service = self.graphic_service.read();
        let graphic_service = graphic_service.downcast_ref::<WgpuGraphicManager>();

        let queue = graphic_service.get_queue();
        let device = graphic_service.get_device();
        let config = graphic_service.get_config();
        let camera_transform = graphic_service.get_camera_transform().clone();
        let viewport_size = window_service.get_size().clone();

        // Get resources
        let material = self
            .resource_container
            .get::<dyn MaterialResource>("Materials/Draw Line")
            .unwrap();
        let material = material.read();
        let material = material.downcast_ref::<WgpuMaterialResource>();

        let shader = self
            .resource_container
            .get::<dyn ShaderResource>("Shaders/Draw Line")
            .unwrap();
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

        graphic_service.push_render_bundle(bundle, z_index);
    }

    fn get_cursor_position(&self) -> Vector2d {
        let window_service = self.window_service.read();
        let graphic_service = self.graphic_service.read();

        // Get informations from the resource dependencies
        let cursor_position = window_service.get_cursor_position();
        let viewport_size = window_service.get_size();
        let camera_transform = graphic_service.get_camera_transform().clone();
        std::mem::drop(graphic_service);
        std::mem::drop(window_service);

        // Transform the cursor in the engine world (especialy taking care of camera)
        let cursor_pos = Vector2d::new(
            (cursor_position.0 as f32 / viewport_size.0 as f32) * 2.0 - 1.0,
            (cursor_position.1 as f32 / viewport_size.1 as f32) * -2.0 + 1.0,
        );

        camera_transform.invert() * cursor_pos
    }
}

impl IntrospectObject for WgpuGraphic2dManager {
    fn get_class_name(&self) -> String {
        "Graphic2dManager".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for WgpuGraphic2dManager {}

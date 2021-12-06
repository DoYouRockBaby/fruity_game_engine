use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::material_reference::MaterialReference;
use fruity_graphic::math::matrix3::Matrix3;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::mesh_resource::MeshResource;
use fruity_graphic_2d::graphic_2d_service::Graphic2dService;
use fruity_wgpu_graphic::graphic_service::WgpuGraphicManager;
use fruity_wgpu_graphic::math::material_reference::WgpuMaterialReference;
use fruity_wgpu_graphic::resources::shader_resource::WgpuShaderResource;
use fruity_wgpu_graphic::wgpu_bridge::Instance;
use fruity_windows::window_service::WindowService;
use std::f32::consts::PI;
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

    fn draw_square(&self, transform: Matrix3, z_index: usize, material: &dyn MaterialReference) {
        let graphic_service = self.graphic_service.read();
        let graphic_service = graphic_service.downcast_ref::<WgpuGraphicManager>();

        let device = graphic_service.get_device();
        let config = graphic_service.get_config();

        // Get resources
        let (material_reference, material) = if let Some(material) = material
            .as_any_ref()
            .downcast_ref::<WgpuMaterialReference>(
        ) {
            (material, material.read())
        } else {
            return;
        };

        let shader = if let Some(shader) = &material.shader {
            shader
        } else {
            return;
        };

        let shader_reader = shader.read();
        let shader_reader = shader_reader.downcast_ref::<WgpuShaderResource>();

        // Create the main render pipeline
        let mesh = self
            .resource_container
            .get::<MeshResource>("Meshes/Squad")
            .unwrap();
        let mesh = mesh.read();

        let num_indices = mesh.indices.len() as u32;

        // TODO: Don't do it every frame (AKA: implements the instancied rendering)
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&mesh.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // TODO: Don't do it every frame (AKA: implements the instancied rendering)
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let mut encoder =
            device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                label: Some("draw_square_bundle"),
                color_formats: &[config.format],
                depth_stencil: None,
                sample_count: 1,
            });

        // TODO: Don't do it every frame (AKA: implements the instancied rendering)
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&[Instance {
                transform: transform.into(),
            }]),
            usage: wgpu::BufferUsages::VERTEX,
        });

        encoder.set_pipeline(&shader_reader.render_pipeline);
        material_reference
            .binding_groups
            .iter()
            .for_each(|(index, bind_group)| {
                encoder.set_bind_group(*index, &bind_group, &[]);
            });
        encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
        encoder.set_vertex_buffer(1, instance_buffer.slice(..));
        encoder.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        encoder.draw_indexed(0..num_indices, 0, 0..1);
        let bundle = encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some("main"),
        });

        graphic_service.push_render_bundle(bundle, z_index);
    }

    fn draw_line(&self, pos1: Vector2d, pos2: Vector2d, _width: u32, color: Color, z_index: usize) {
        let window_service = self.window_service.read();

        // TODO: Use width to respect pixel width constraint

        // Calculate squad transform
        let diff = pos2 - pos1;
        let scale_factor = window_service.get_scale_factor();

        let translate = (pos1 + pos2) / 2.0;
        let rotate = (diff.y / diff.x).atan() + PI / 2.0;
        let scale = Vector2d {
            x: 1.0 / (scale_factor as f32) / 100.0,
            y: diff.length(),
        };

        // Calculate transform
        let transform = Matrix3::identity()
            * Matrix3::translation(translate)
            * Matrix3::rotation(rotate)
            * Matrix3::scaling(scale);

        // Create the material
        let graphic_service_reader = self.graphic_service.read();
        let graphic_service_reader = graphic_service_reader.downcast_ref::<WgpuGraphicManager>();

        let draw_line_material = self
            .resource_container
            .get::<MaterialResource>("Materials/Draw Line")
            .unwrap();

        // Update line color
        let draw_line_material =
            WgpuMaterialReference::new(graphic_service_reader, draw_line_material);
        draw_line_material.set_color("color", color);

        // Draw the line
        self.draw_square(transform, z_index, &draw_line_material);
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

impl WgpuGraphic2dManager {}

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

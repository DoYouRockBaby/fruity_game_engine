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
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::mesh_resource::MeshResource;
use fruity_windows::window_service::WindowService;
use std::f32::consts::PI;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, FruityAny)]
pub struct Graphic2dService {
    window_service: ResourceReference<dyn WindowService>,
    graphic_service: ResourceReference<dyn GraphicService>,
    resource_container: Arc<ResourceContainer>,
}

impl Graphic2dService {
    pub fn new(resource_container: Arc<ResourceContainer>) -> Self {
        let window_service = resource_container.require::<dyn WindowService>();
        let graphic_service = resource_container.require::<dyn GraphicService>();

        Self {
            window_service,
            graphic_service,
            resource_container,
        }
    }

    pub fn draw_square(
        &self,
        transform: Matrix3,
        z_index: usize,
        material: &dyn MaterialReference,
    ) {
        let graphic_service = self.graphic_service.read();

        let mesh = self
            .resource_container
            .get::<dyn MeshResource>("Meshes/Squad")
            .unwrap();
        let mesh = mesh.read();

        graphic_service.draw_mesh(transform, z_index, mesh.deref(), material)
    }

    pub fn draw_line(
        &self,
        pos1: Vector2d,
        pos2: Vector2d,
        _width: u32,
        color: Color,
        z_index: usize,
    ) {
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

        // Get the material

        let draw_line_material = self
            .resource_container
            .get::<MaterialResource>("Materials/Draw Line")
            .unwrap();

        // Update line color
        let graphic_service = self.graphic_service.read();
        let draw_line_material = graphic_service.create_material_reference(draw_line_material);
        draw_line_material.set_color("color", color);

        // Draw the line
        self.draw_square(transform, z_index, draw_line_material.deref());
    }

    /// Get the cursor position in the 2D world, take in care the camera transform
    pub fn get_cursor_position(&self) -> Vector2d {
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

impl IntrospectObject for Graphic2dService {
    fn get_class_name(&self) -> String {
        "Graphic2dService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for Graphic2dService {}

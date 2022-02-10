use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::utils::math::normalise_angle_range;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::graphic_service::MaterialParam;
use fruity_graphic::math::material_reference::MaterialReference;
use fruity_graphic::math::matrix3::Matrix3;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::mesh_resource::MeshResource;
use fruity_windows::window_service::WindowService;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::ops::Deref;
use std::ops::Range;
use std::sync::Arc;

#[derive(Debug, FruityAny)]
pub struct Graphic2dService {
    window_service: ResourceReference<dyn WindowService>,
    graphic_service: ResourceReference<dyn GraphicService>,
    resource_container: Arc<ResourceContainer>,
    draw_line_material: Box<dyn MaterialReference>,
    draw_rect_material: Box<dyn MaterialReference>,
    draw_arc_material: Box<dyn MaterialReference>,
}

impl Graphic2dService {
    pub fn new(resource_container: Arc<ResourceContainer>) -> Self {
        let window_service = resource_container.require::<dyn WindowService>();
        let graphic_service = resource_container.require::<dyn GraphicService>();
        let graphic_service_reader = graphic_service.read();

        let draw_line_material = resource_container
            .get::<dyn MaterialResource>("Materials/Draw Line")
            .unwrap();

        let draw_rect_material = resource_container
            .get::<dyn MaterialResource>("Materials/Draw Rect")
            .unwrap();

        let draw_arc_material = resource_container
            .get::<dyn MaterialResource>("Materials/Draw Arc")
            .unwrap();

        Self {
            window_service,
            graphic_service,
            resource_container,
            draw_line_material: graphic_service_reader
                .create_material_reference(draw_line_material),
            draw_rect_material: graphic_service_reader
                .create_material_reference(draw_rect_material),
            draw_arc_material: graphic_service_reader.create_material_reference(draw_arc_material),
        }
    }

    pub fn draw_quad(
        &self,
        identifier: u64,
        material: &dyn MaterialReference,
        params: HashMap<String, MaterialParam>,
        z_index: i32,
    ) {
        let graphic_service = self.graphic_service.read();

        let mesh = self
            .resource_container
            .get::<dyn MeshResource>("Meshes/Squad")
            .unwrap();

        graphic_service.draw_mesh(identifier, mesh.clone(), material, params, z_index)
    }

    pub fn draw_line(
        &self,
        pos1: Vector2d,
        pos2: Vector2d,
        width: u32,
        color: Color,
        z_index: i32,
    ) {
        // Update line instance fields
        self.draw_line_material.set_vector2d("pos1", pos1);
        self.draw_line_material.set_vector2d("pos2", pos2);
        self.draw_line_material.set_uint("width", width);
        self.draw_line_material.set_color("color", color);

        // Draw the line
        self.draw_quad(0, self.draw_line_material.deref(), z_index);
    }

    pub fn draw_rect(
        &self,
        bottom_left: Vector2d,
        top_right: Vector2d,
        width: u32,
        fill_color: Color,
        border_color: Color,
        z_index: i32,
    ) {
        // Update line instance fields
        self.draw_rect_material
            .set_vector2d("bottom_left", bottom_left);
        self.draw_rect_material.set_vector2d("top_right", top_right);
        self.draw_rect_material.set_uint("width", width);
        self.draw_rect_material.set_color("fill_color", fill_color);
        self.draw_rect_material
            .set_color("border_color", border_color);

        // Draw the line
        self.draw_quad(0, self.draw_rect_material.deref(), z_index);
    }

    pub fn draw_arc(
        &self,
        center: Vector2d,
        radius: f32,
        angle_range: Range<f32>,
        width: u32,
        fill_color: Color,
        border_color: Color,
        z_index: i32,
    ) {
        let window_service = self.window_service.read();
        let windows_size = window_service.get_size();

        // Calculate squad transform
        let scale = Vector2d {
            x: radius * 2.0,
            y: radius * 2.0,
        };
        let width = 2.0 * width as f32 / windows_size.0 as f32 / scale.x;

        // Calculate transform
        let transform =
            Matrix3::identity() * Matrix3::translation(center) * Matrix3::scaling(scale);
        let angle_range = normalise_angle_range(angle_range);

        // Update line color
        self.draw_arc_material
            .set_matrix4("transform", transform.into());
        self.draw_arc_material.set_color("fill_color", fill_color);
        self.draw_arc_material
            .set_color("border_color", border_color);
        self.draw_arc_material.set_float("width", width);
        self.draw_arc_material
            .set_float("angle_start", angle_range.start);
        self.draw_arc_material
            .set_float("angle_end", angle_range.end);

        // Draw the line
        self.draw_quad(0, self.draw_arc_material.deref(), z_index);
    }

    pub fn draw_circle(
        &self,
        center: Vector2d,
        radius: f32,
        width: u32,
        fill_color: Color,
        border_color: Color,
        z_index: i32,
    ) {
        self.draw_arc(
            center,
            radius,
            0.0..(2.0 * PI),
            width,
            fill_color,
            border_color,
            z_index,
        );
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

use adjacent_pair_iterator::AdjacentPairIterator;
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
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::mesh_resource::MeshResource;
use maplit::hashmap;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::ops::Range;
use std::sync::Arc;

#[derive(Debug, FruityAny)]
pub struct Graphic2dService {
    graphic_service: ResourceReference<dyn GraphicService>,
    resource_container: Arc<ResourceContainer>,
    draw_line_material: ResourceReference<dyn MaterialResource>,
    draw_dotted_line_material: ResourceReference<dyn MaterialResource>,
    draw_rect_material: ResourceReference<dyn MaterialResource>,
    draw_arc_material: ResourceReference<dyn MaterialResource>,
}

impl Graphic2dService {
    pub fn new(resource_container: Arc<ResourceContainer>) -> Self {
        let graphic_service = resource_container.require::<dyn GraphicService>();

        let draw_line_material = resource_container
            .get::<dyn MaterialResource>("Materials/Draw Line")
            .unwrap();

        let draw_dotted_line_material = resource_container
            .get::<dyn MaterialResource>("Materials/Draw Dotted Line")
            .unwrap();

        let draw_rect_material = resource_container
            .get::<dyn MaterialResource>("Materials/Draw Rect")
            .unwrap();

        let draw_arc_material = resource_container
            .get::<dyn MaterialResource>("Materials/Draw Arc")
            .unwrap();

        Self {
            graphic_service,
            resource_container,
            draw_line_material,
            draw_dotted_line_material,
            draw_rect_material,
            draw_arc_material,
        }
    }

    pub fn draw_quad(
        &self,
        identifier: u64,
        material: ResourceReference<dyn MaterialResource>,
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
        self.draw_quad(
            0,
            self.draw_line_material.clone(),
            hashmap! {
                "pos1".to_string() => MaterialParam::Vector2(pos1),
                "pos2".to_string() => MaterialParam::Vector2(pos2),
                "width".to_string() => MaterialParam::UInt(width),
                "color".to_string() => MaterialParam::Color(color),
            },
            z_index,
        );
    }

    pub fn draw_polyline(&self, points: Vec<Vector2d>, width: u32, color: Color, z_index: i32) {
        points
            .adjacent_pairs()
            .for_each(|(pos1, pos2)| self.draw_line(pos1, pos2, width, color, z_index));
    }

    pub fn draw_dotted_line(
        &self,
        pos1: Vector2d,
        pos2: Vector2d,
        width: u32,
        color: Color,
        z_index: i32,
    ) {
        self.draw_quad(
            0,
            self.draw_dotted_line_material.clone(),
            hashmap! {
                "pos1".to_string() => MaterialParam::Vector2(pos1),
                "pos2".to_string() => MaterialParam::Vector2(pos2),
                "width".to_string() => MaterialParam::UInt(width),
                "color".to_string() => MaterialParam::Color(color),
            },
            z_index,
        );
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
        self.draw_quad(
            0,
            self.draw_rect_material.clone(),
            hashmap! {
                "bottom_left".to_string() => MaterialParam::Vector2(bottom_left),
                "top_right".to_string() => MaterialParam::Vector2(top_right),
                "width".to_string() => MaterialParam::UInt(width),
                "fill_color".to_string() => MaterialParam::Color(fill_color),
                "border_color".to_string() => MaterialParam::Color(border_color),
            },
            z_index,
        );
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
        // Calculate angle range
        let angle_range = normalise_angle_range(angle_range);

        // Draw the arc
        self.draw_quad(
            0,
            self.draw_arc_material.clone(),
            hashmap! {
                "center".to_string() => MaterialParam::Vector2(center),
                "radius".to_string() => MaterialParam::Float(radius),
                "fill_color".to_string() => MaterialParam::Color(fill_color),
                "border_color".to_string() => MaterialParam::Color(border_color),
                "width".to_string() => MaterialParam::UInt(width),
                "angle_start".to_string() => MaterialParam::Float(angle_range.start),
                "angle_end".to_string() => MaterialParam::Float(angle_range.end),
            },
            z_index,
        );
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

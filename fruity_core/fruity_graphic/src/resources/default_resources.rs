use crate::graphic_service::GraphicService;
use crate::math::vector3d::Vector3d;
use crate::resources::material_resource::MaterialResourceSettings;
use crate::resources::material_resource::MaterialSettingsBinding;
use crate::resources::material_resource::MaterialSettingsInstanceAttribute;
use crate::resources::mesh_resource::MeshResourceSettings;
use crate::resources::mesh_resource::Vertex;
use crate::resources::shader_resource::ShaderBinding;
use crate::resources::shader_resource::ShaderBindingGroup;
use crate::resources::shader_resource::ShaderBindingType;
use crate::resources::shader_resource::ShaderBindingVisibility;
use crate::resources::shader_resource::ShaderInstanceAttribute;
use crate::resources::shader_resource::ShaderInstanceAttributeType;
use crate::resources::shader_resource::ShaderResource;
use crate::resources::shader_resource::ShaderResourceSettings;
use crate::Vector2d;
use fruity_core::resource::resource_container::ResourceContainer;
use maplit::hashmap;
use std::sync::Arc;

pub fn load_default_resources(resource_container: Arc<ResourceContainer>) {
    load_squad_mesh(resource_container.clone());
    load_draw_line_shader(resource_container.clone());
    load_draw_line_material(resource_container.clone());
    load_draw_rect_shader(resource_container.clone());
    load_draw_rect_material(resource_container.clone());
    load_draw_arc_shader(resource_container.clone());
    load_draw_arc_material(resource_container.clone());
}

pub fn load_squad_mesh(resource_container: Arc<ResourceContainer>) {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let resource = graphic_service
        .create_mesh_resource(
            "Meshes/Squad",
            MeshResourceSettings {
                vertices: vec![
                    Vertex {
                        position: Vector3d::new(-0.5, -0.5, 0.0),
                        tex_coords: Vector2d::new(0.0, 1.0),
                        normal: Vector3d::new(0.0, 0.0, -1.0),
                    },
                    Vertex {
                        position: Vector3d::new(0.5, -0.5, 0.0),
                        tex_coords: Vector2d::new(1.0, 1.0),
                        normal: Vector3d::new(0.0, 0.0, -1.0),
                    },
                    Vertex {
                        position: Vector3d::new(0.5, 0.5, 0.0),
                        tex_coords: Vector2d::new(1.0, 0.0),
                        normal: Vector3d::new(0.0, 0.0, -1.0),
                    },
                    Vertex {
                        position: Vector3d::new(-0.5, 0.5, 0.0),
                        tex_coords: Vector2d::new(0.0, 0.0),
                        normal: Vector3d::new(0.0, 0.0, -1.0),
                    },
                ],
                indices: vec![0, 1, 2, 3, 0, 2, /* padding */ 0],
            },
        )
        .unwrap();

    resource_container.add("Meshes/Squad", resource);
}

pub fn load_draw_line_shader(resource_container: Arc<ResourceContainer>) {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let code = "
        [[block]]
        struct CameraUniform {
            view_proj: mat4x4<f32>;
        };
        
        struct VertexInput {
            [[location(0)]] position: vec3<f32>;
            [[location(1)]] tex_coords: vec2<f32>;
            [[location(2)]] normal: vec3<f32>;
        };
        
        struct InstanceInput {
            [[location(5)]] model_matrix_0: vec4<f32>;
            [[location(6)]] model_matrix_1: vec4<f32>;
            [[location(7)]] model_matrix_2: vec4<f32>;
            [[location(8)]] model_matrix_3: vec4<f32>;
            [[location(9)]] color: vec4<f32>;
        };
        
        struct VertexOutput {
            [[builtin(position)]] position: vec4<f32>;
            [[location(0)]] color: vec4<f32>;
        };

        [[group(0), binding(0)]]
        var<uniform> camera: CameraUniform;

        [[stage(vertex)]]
        fn main(
            model: VertexInput,
            instance: InstanceInput,
        ) -> VertexOutput {
            let model_matrix = mat4x4<f32>(
                instance.model_matrix_0,
                instance.model_matrix_1,
                instance.model_matrix_2,
                instance.model_matrix_3,
            );

            var out: VertexOutput;
            out.position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
            out.color = instance.color;
            return out;
        }

        [[stage(fragment)]]
        fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
            return in.color;
        }"
    .to_string();

    let resource = graphic_service
        .create_shader_resource(
            "Shaders/Draw Line",
            code,
            ShaderResourceSettings {
                binding_groups: vec![ShaderBindingGroup {
                    bindings: vec![ShaderBinding {
                        visibility: ShaderBindingVisibility::Vertex,
                        ty: ShaderBindingType::Uniform,
                    }],
                }],
                instance_attributes: vec![
                    ShaderInstanceAttribute {
                        location: 5,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 6,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 7,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 8,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 9,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                ],
            },
        )
        .unwrap();

    resource_container.add("Shaders/Draw Line", resource);
}

pub fn load_draw_line_material(resource_container: Arc<ResourceContainer>) {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let shader = resource_container.get::<dyn ShaderResource>("Shaders/Draw Line");

    let resource = graphic_service
        .create_material_resource(
            "Materials/Draw Line",
            MaterialResourceSettings {
                shader,
                bindings: vec![MaterialSettingsBinding::Camera { bind_group: 0 }],
                instance_attributes: hashmap! {
                    "transform".to_string() => MaterialSettingsInstanceAttribute::Matrix4 {
                        vec0_location: 5,
                        vec1_location: 6,
                        vec2_location: 7,
                        vec3_location: 8,
                    },
                    "color".to_string() => MaterialSettingsInstanceAttribute::Vector4 {
                        location: 9,
                    },
                },
            },
        )
        .unwrap();

    resource_container.add("Materials/Draw Line", resource);
}

pub fn load_draw_rect_shader(resource_container: Arc<ResourceContainer>) {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let code = "
        [[block]]
        struct CameraUniform {
            view_proj: mat4x4<f32>;
        };
        
        struct VertexInput {
            [[location(0)]] position: vec3<f32>;
            [[location(1)]] tex_coords: vec2<f32>;
            [[location(2)]] normal: vec3<f32>;
        };
        
        struct InstanceInput {
            [[location(5)]] model_matrix_0: vec4<f32>;
            [[location(6)]] model_matrix_1: vec4<f32>;
            [[location(7)]] model_matrix_2: vec4<f32>;
            [[location(8)]] model_matrix_3: vec4<f32>;
            [[location(9)]] fill_color: vec4<f32>;
            [[location(10)]] border_color: vec4<f32>;
            [[location(11)]] xwidth: f32;
            [[location(12)]] ywidth: f32;
        };
        
        struct VertexOutput {
            [[builtin(position)]] position: vec4<f32>;
            [[location(0)]] border_color: vec4<f32>;
            [[location(1)]] fill_color: vec4<f32>;
            [[location(2)]] tex_coords: vec2<f32>;
            [[location(3)]] xwidth: f32;
            [[location(4)]] ywidth: f32;
        };

        [[group(0), binding(0)]]
        var<uniform> camera: CameraUniform;

        [[stage(vertex)]]
        fn main(
            model: VertexInput,
            instance: InstanceInput,
        ) -> VertexOutput {
            let model_matrix = mat4x4<f32>(
                instance.model_matrix_0,
                instance.model_matrix_1,
                instance.model_matrix_2,
                instance.model_matrix_3,
            );

            var out: VertexOutput;
            out.position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
            out.fill_color = instance.fill_color;
            out.border_color = instance.border_color;
            out.tex_coords = model.tex_coords;
            out.xwidth = instance.xwidth;
            out.ywidth = instance.ywidth;
            return out;
        }

        [[stage(fragment)]]
        fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
            if(
                in.tex_coords.x <= in.xwidth ||
                in.tex_coords.x >= (1.0 - in.xwidth) ||
                in.tex_coords.y <= in.ywidth ||
                in.tex_coords.y >= (1.0 - in.ywidth)
            ) {
                return in.border_color;
            } else {
                return in.fill_color;
            }
        }"
    .to_string();

    let resource = graphic_service
        .create_shader_resource(
            "Shaders/Draw Rect",
            code,
            ShaderResourceSettings {
                binding_groups: vec![ShaderBindingGroup {
                    bindings: vec![ShaderBinding {
                        visibility: ShaderBindingVisibility::Vertex,
                        ty: ShaderBindingType::Uniform,
                    }],
                }],
                instance_attributes: vec![
                    ShaderInstanceAttribute {
                        location: 5,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 6,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 7,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 8,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 9,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 10,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 11,
                        ty: ShaderInstanceAttributeType::Float,
                    },
                    ShaderInstanceAttribute {
                        location: 12,
                        ty: ShaderInstanceAttributeType::Float,
                    },
                ],
            },
        )
        .unwrap();

    resource_container.add("Shaders/Draw Rect", resource);
}

pub fn load_draw_rect_material(resource_container: Arc<ResourceContainer>) {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let shader = resource_container.get::<dyn ShaderResource>("Shaders/Draw Rect");

    let resource = graphic_service
        .create_material_resource(
            "Materials/Draw Rect",
            MaterialResourceSettings {
                shader,
                bindings: vec![MaterialSettingsBinding::Camera { bind_group: 0 }],
                instance_attributes: hashmap! {
                    "transform".to_string() => MaterialSettingsInstanceAttribute::Matrix4 {
                        vec0_location: 5,
                        vec1_location: 6,
                        vec2_location: 7,
                        vec3_location: 8,
                    },
                    "fill_color".to_string() => MaterialSettingsInstanceAttribute::Vector4 {
                        location: 9,
                    },
                    "border_color".to_string() => MaterialSettingsInstanceAttribute::Vector4 {
                        location: 10,
                    },
                    "xwidth".to_string() => MaterialSettingsInstanceAttribute::Float {
                        location: 11,
                    },
                    "ywidth".to_string() => MaterialSettingsInstanceAttribute::Float {
                        location: 12,
                    },
                },
            },
        )
        .unwrap();

    resource_container.add("Materials/Draw Rect", resource);
}

pub fn load_draw_arc_shader(resource_container: Arc<ResourceContainer>) {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let code = "
        [[block]]
        struct CameraUniform {
            view_proj: mat4x4<f32>;
        };
        
        struct VertexInput {
            [[location(0)]] position: vec3<f32>;
            [[location(1)]] tex_coords: vec2<f32>;
            [[location(2)]] normal: vec3<f32>;
        };
        
        struct InstanceInput {
            [[location(5)]] model_matrix_0: vec4<f32>;
            [[location(6)]] model_matrix_1: vec4<f32>;
            [[location(7)]] model_matrix_2: vec4<f32>;
            [[location(8)]] model_matrix_3: vec4<f32>;
            [[location(9)]] fill_color: vec4<f32>;
            [[location(10)]] border_color: vec4<f32>;
            [[location(11)]] width: f32;
            [[location(12)]] angle_start: f32;
            [[location(13)]] angle_end: f32;
        };
        
        struct VertexOutput {
            [[builtin(position)]] position: vec4<f32>;
            [[location(0)]] border_color: vec4<f32>;
            [[location(1)]] fill_color: vec4<f32>;
            [[location(2)]] tex_coords: vec2<f32>;
            [[location(3)]] width: f32;
            [[location(4)]] angle_start: f32;
            [[location(5)]] angle_end: f32;
        };

        [[group(0), binding(0)]]
        var<uniform> camera: CameraUniform;

        [[stage(vertex)]]
        fn main(
            model: VertexInput,
            instance: InstanceInput,
        ) -> VertexOutput {
            let model_matrix = mat4x4<f32>(
                instance.model_matrix_0,
                instance.model_matrix_1,
                instance.model_matrix_2,
                instance.model_matrix_3,
            );

            var out: VertexOutput;
            out.position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
            out.fill_color = instance.fill_color;
            out.border_color = instance.border_color;
            out.tex_coords = model.tex_coords;
            out.width = instance.width;
            out.angle_start = instance.angle_start;
            out.angle_end = instance.angle_end;
            return out;
        }

        [[stage(fragment)]]
        fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
            let circle_coords = 2.0 * (in.tex_coords - vec2<f32>(0.5, 0.5));
            let angle = atan2(-circle_coords.y, circle_coords.x);
            let border_radius = 1.0 - in.width;

            if(
                length(circle_coords) <= 1.0 &&
                angle <= in.angle_end &&
                angle >= in.angle_start
            ) {
                if(length(circle_coords) < border_radius) {
                    return in.fill_color;
                } else {
                    return in.border_color;
                }
            } else {
                return vec4<f32>(0.0, 0.0, 0.0, 0.0);
            }
        }"
    .to_string();

    let resource = graphic_service
        .create_shader_resource(
            "Shaders/Draw Arc",
            code,
            ShaderResourceSettings {
                binding_groups: vec![ShaderBindingGroup {
                    bindings: vec![ShaderBinding {
                        visibility: ShaderBindingVisibility::Vertex,
                        ty: ShaderBindingType::Uniform,
                    }],
                }],
                instance_attributes: vec![
                    ShaderInstanceAttribute {
                        location: 5,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 6,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 7,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 8,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 9,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 10,
                        ty: ShaderInstanceAttributeType::Vector4,
                    },
                    ShaderInstanceAttribute {
                        location: 11,
                        ty: ShaderInstanceAttributeType::Float,
                    },
                    ShaderInstanceAttribute {
                        location: 12,
                        ty: ShaderInstanceAttributeType::Float,
                    },
                    ShaderInstanceAttribute {
                        location: 13,
                        ty: ShaderInstanceAttributeType::Float,
                    },
                ],
            },
        )
        .unwrap();

    resource_container.add("Shaders/Draw Arc", resource);
}

pub fn load_draw_arc_material(resource_container: Arc<ResourceContainer>) {
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    let shader = resource_container.get::<dyn ShaderResource>("Shaders/Draw Arc");

    let resource = graphic_service
        .create_material_resource(
            "Materials/Draw Arc",
            MaterialResourceSettings {
                shader,
                bindings: vec![MaterialSettingsBinding::Camera { bind_group: 0 }],
                instance_attributes: hashmap! {
                    "transform".to_string() => MaterialSettingsInstanceAttribute::Matrix4 {
                        vec0_location: 5,
                        vec1_location: 6,
                        vec2_location: 7,
                        vec3_location: 8,
                    },
                    "fill_color".to_string() => MaterialSettingsInstanceAttribute::Vector4 {
                        location: 9,
                    },
                    "border_color".to_string() => MaterialSettingsInstanceAttribute::Vector4 {
                        location: 10,
                    },
                    "width".to_string() => MaterialSettingsInstanceAttribute::Float {
                        location: 11,
                    },
                    "angle_start".to_string() => MaterialSettingsInstanceAttribute::Float {
                        location: 12,
                    },
                    "angle_end".to_string() => MaterialSettingsInstanceAttribute::Float {
                        location: 13,
                    },
                },
            },
        )
        .unwrap();

    resource_container.add("Materials/Draw Arc", resource);
}

use crate::graphic_service::GraphicService;
use crate::math::RED;
use crate::resources::material_resource::MaterialBinding;
use crate::resources::material_resource::MaterialResource;
use crate::resources::mesh_resource::MeshResourceSettings;
use crate::resources::mesh_resource::Vertex;
use crate::resources::shader_resource::ShaderBinding;
use crate::resources::shader_resource::ShaderBindingGroup;
use crate::resources::shader_resource::ShaderBindingType;
use crate::resources::shader_resource::ShaderBindingVisibility;
use crate::resources::shader_resource::ShaderResource;
use crate::resources::shader_resource::ShaderResourceSettings;
use fruity_core::resource::resource_container::ResourceContainer;
use maplit::hashmap;
use std::sync::Arc;

pub fn load_default_resources(resource_container: Arc<ResourceContainer>) {
    load_squad_mesh(resource_container.clone());
    load_draw_line_shader(resource_container.clone());
    load_draw_line_material(resource_container.clone());
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
                        position: [-0.5, -0.5, 0.0],
                        tex_coords: [0.0, 1.0],
                        normal: [0.0, 0.0, -1.0],
                    },
                    Vertex {
                        position: [0.5, -0.5, 0.0],
                        tex_coords: [1.0, 1.0],
                        normal: [0.0, 0.0, -1.0],
                    },
                    Vertex {
                        position: [0.5, 0.5, 0.0],
                        tex_coords: [1.0, 0.0],
                        normal: [0.0, 0.0, -1.0],
                    },
                    Vertex {
                        position: [-0.5, 0.5, 0.0],
                        tex_coords: [0.0, 0.0],
                        normal: [0.0, 0.0, -1.0],
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
        struct ColorBuffer {
            color: vec4<f32>;
        };

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
        };
        
        struct VertexOutput {
            [[builtin(position)]] position: vec4<f32>;
        };

        [[group(0), binding(0)]]
        var<uniform> color_buffer: ColorBuffer;

        [[group(1), binding(0)]]
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
            // out.position = model_matrix * vec4<f32>(model.position, 1.0);
            // out.position = camera.view_proj * vec4<f32>(model.position, 1.0);
            // out.position = vec4<f32>(model.position, 1.0);
            return out;
        }

        [[stage(fragment)]]
        fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
            return color_buffer.color;
        }"
    .to_string();

    let resource = graphic_service
        .create_shader_resource(
            "Shaders/Draw Line",
            code,
            ShaderResourceSettings {
                binding_groups: vec![
                    ShaderBindingGroup {
                        bindings: vec![ShaderBinding {
                            visibility: ShaderBindingVisibility::Fragment,
                            ty: ShaderBindingType::Uniform,
                        }],
                    },
                    ShaderBindingGroup {
                        bindings: vec![ShaderBinding {
                            visibility: ShaderBindingVisibility::Vertex,
                            ty: ShaderBindingType::Uniform,
                        }],
                    },
                ],
            },
        )
        .unwrap();

    resource_container.add("Shaders/Draw Line", resource);
}

pub fn load_draw_line_material(resource_container: Arc<ResourceContainer>) {
    let shader = resource_container.get::<dyn ShaderResource>("Shaders/Draw Line");

    let resource = Box::new(MaterialResource {
        shader,
        bindings: hashmap! {
            "color".to_string() => vec![MaterialBinding::Color {
                default: RED,
                bind_group: 0,
                bind: 0,
            }],
            "camera".to_string() => vec![MaterialBinding::Camera {
                bind_group: 1
            }],
        },
    });

    resource_container.add("Materials/Draw Line", resource);
}

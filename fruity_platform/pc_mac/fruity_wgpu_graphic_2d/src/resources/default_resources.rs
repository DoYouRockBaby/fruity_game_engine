use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use maplit::hashmap;
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;

pub fn load_default_resources(resource_container: Arc<ResourceContainer>) {
    load_draw_line_shader(resource_container.clone());
    load_draw_line_material(resource_container.clone());
}

pub fn load_draw_line_shader(resource_container: Arc<ResourceContainer>) {
    let settings = Settings::Object(hashmap! {
        "binding_groups".to_string() => Settings::Array(vec![
            Settings::Array(vec![
                Settings::Object(hashmap!{
                    "visibility".to_string() => Settings::String("fragment".to_string()),
                    "type".to_string() => Settings::String("uniform".to_string()),
                })
            ]),
            Settings::Array(vec![
                Settings::Object(hashmap!{
                    "visibility".to_string() => Settings::String("vertex".to_string()),
                    "type".to_string() => Settings::String("uniform".to_string()),
                })
            ])
        ])
    });

    let shader_src = "
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

    let mut shader_src = Cursor::new(shader_src);
    resource_container
        .load_resource("Shaders/Draw Line", "wgsl", &mut shader_src, settings)
        .unwrap();
}

pub fn load_draw_line_material(resource_container: Arc<ResourceContainer>) {
    let settings = Settings::Object(HashMap::default());

    let material_src = "
shader: \"Shaders/Draw Line\"
fields:
- type: color
  name: color
  default: #0F0
  bind_group: 0
  bind: 0
- name: camera
  type: camera
  bind_group: 1"
        .to_string();

    let mut material_src = Cursor::new(material_src);
    resource_container
        .load_resource(
            "Materials/Draw Line",
            "material",
            &mut material_src,
            settings,
        )
        .unwrap();
}

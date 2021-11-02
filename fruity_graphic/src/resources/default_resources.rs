use fruity_core::resource::resources_manager::ResourceIdentifier;
use fruity_core::resource::resources_manager::ResourcesManager;
use fruity_core::service::service_rwlock::ServiceRwLock;
use fruity_core::settings::Settings;
use maplit::hashmap;
use std::collections::HashMap;
use std::io::Cursor;

pub fn load_default_resources(resources_manager: ServiceRwLock<ResourcesManager>) {
    load_draw_line_shader(resources_manager.clone());
    load_draw_line_material(resources_manager.clone());
}

pub fn load_draw_line_shader(mut resources_manager: ServiceRwLock<ResourcesManager>) {
    let settings = Settings::Object(hashmap! {
        "bindings".to_string() => Settings::Array(vec![
            Settings::Object(hashmap!{
                "id".to_string() => Settings::I64(0),
                "visibility".to_string() => Settings::String("vertex".to_string()),
                "type".to_string() => Settings::String("uniform".to_string()),
            })
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
        };

        struct VertexOutput {
            [[location(0)]] color: vec4<f32>;
            [[builtin(position)]] position: vec4<f32>;
        };

        [[group(0), binding(0)]]
        var<uniform> color_buffer: ColorBuffer;

        [[group(1), binding(0)]]
        var<uniform> camera: CameraUniform;

        [[stage(vertex)]]
        fn main(model: VertexInput) -> VertexOutput {
            var out: VertexOutput;
            out.position = camera.view_proj * vec4<f32>(model.position, 1.0);
            out.color = color_buffer.color;
            return out;
        }

        [[stage(fragment)]]
        fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
            return in.color;
        }"
    .to_string();

    let mut shader_src = Cursor::new(shader_src);
    resources_manager
        .load_resource(
            ResourceIdentifier("Shaders/Draw Line".to_string()),
            "wgsl",
            &mut shader_src,
            settings,
        )
        .unwrap();
}

pub fn load_draw_line_material(mut resources_manager: ServiceRwLock<ResourcesManager>) {
    let settings = Settings::Object(HashMap::default());

    let material_src = "
shader: \"Shaders/Draw Line\"
binding_groups:
- type: custom
  index: 0
  bindings:
  - type: uniform
    index: 0
- type: camera
  index: 1"
        .to_string();

    let mut material_src = Cursor::new(material_src);
    resources_manager
        .load_resource(
            ResourceIdentifier("Materials/Draw Line".to_string()),
            "material",
            &mut material_src,
            settings,
        )
        .unwrap();
}

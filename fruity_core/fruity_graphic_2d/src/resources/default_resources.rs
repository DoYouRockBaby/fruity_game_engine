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

        struct VertexInput {
            [[location(0)]] position: vec3<f32>;
            [[location(1)]] tex_coords: vec2<f32>;
        };

        struct VertexOutput {
            [[builtin(position)]] position: vec4<f32>;
            [[location(0)]] color: vec4<f32>;
        };

        [[group(0), binding(0)]]
        var<uniform> color_buffer: ColorBuffer;

        [[stage(vertex)]]
        fn main(model: VertexInput) -> VertexOutput {
            var out: VertexOutput;
            out.position = vec4<f32>(model.position, 1.0);
            out.color = color_buffer.color;
            return out;
        }

        [[stage(fragment)]]
        fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
            return in.color;
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
binding_groups:
- type: custom
  index: 0
  bindings:
  - type: uniform
    index: 0"
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

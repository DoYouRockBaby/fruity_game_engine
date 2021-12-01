use fruity_core::serialize::serialized::SerializableObject;
use fruity_editor::fields::resource_reference::draw_editor_resource_reference;
use fruity_editor::ui_element::UIElement;
use fruity_graphic::resources::shader_resource::ShaderResource;

pub fn draw_editor_shader_reference(
    name: &str,
    value: Box<dyn SerializableObject>,
    on_update: impl Fn(Box<dyn SerializableObject>) + Send + Sync + 'static,
) -> UIElement {
    draw_editor_resource_reference::<dyn ShaderResource>(name, value, Box::new(on_update))
}

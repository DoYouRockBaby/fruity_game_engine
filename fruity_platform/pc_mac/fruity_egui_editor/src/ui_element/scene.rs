use crate::ui_element::DrawContext;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_editor::hooks::topo;
use fruity_editor::hooks::use_global;
use fruity_editor::hooks::use_memo;
use fruity_editor::state::world::WorldState;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::math::Color;
use fruity_graphic::resources::texture_resource::TextureResource;
use fruity_wgpu_graphic::graphic_service::WgpuGraphicService;
use fruity_wgpu_graphic::resources::texture_resource::WgpuTextureResource;
use std::sync::Arc;
use std::sync::RwLock;

#[topo::nested]
pub fn draw_scene(ui: &mut egui::Ui, ctx: &mut DrawContext) {
    let rect = ui.available_rect_before_wrap();
    let width = (rect.width() / ui.input().physical_pixel_size()) as u32;
    let height = (rect.height() / ui.input().physical_pixel_size()) as u32;

    // Build the rendering texture
    let (resource, rendering_texture_id) = use_memo(
        |(width, height)| {
            // Get all what we need to initialize
            let world_state = use_global::<WorldState>();
            let graphic_service = world_state
                .resource_container
                .require::<dyn GraphicService>();
            let graphic_service = graphic_service.read();
            let graphic_service = graphic_service.downcast_ref::<WgpuGraphicService>();

            let device = graphic_service.get_device();
            let surface_config = graphic_service.get_config();

            // Create the rendering texture resource
            let resource = ResourceReference::new(
                "Rendering View",
                Arc::new(RwLock::new(Box::new(WgpuTextureResource::render(
                    device,
                    surface_config,
                    width,
                    height,
                    "Rendering View",
                )) as Box<dyn TextureResource>)),
                world_state.resource_container.clone(),
            );

            // Use the texture as the rendering texture
            let image = resource.read();
            let image = image.downcast_ref::<WgpuTextureResource>();

            // Get the egui identifier for the texture
            (
                resource,
                ctx.egui_rpass.egui_texture_from_wgpu_texture(
                    ctx.device,
                    &image.texture,
                    wgpu::FilterMode::Linear,
                ),
            )
        },
        (width, height),
    );

    // Get all what we need to draw
    let world_state = use_global::<WorldState>();
    let graphic_service = world_state
        .resource_container
        .require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();
    let view_proj = Matrix4::from_rect(-1.5, 1.5, -1.0, 1.0, -1.0, 1.0);

    // Draw the scene on the texture
    let background_color = ui.style().visuals.faint_bg_color;
    let background_color = Color::new(
        background_color.r() as f32 / 255.0,
        background_color.g() as f32 / 255.0,
        background_color.b() as f32 / 255.0,
        background_color.a() as f32 / 255.0,
    );
    graphic_service.render_scene(view_proj, background_color, Some(resource.clone()));

    // Display the scene
    ui.add_sized(
        rect.size(),
        egui::Image::new(rendering_texture_id, rect.size()),
    );
}

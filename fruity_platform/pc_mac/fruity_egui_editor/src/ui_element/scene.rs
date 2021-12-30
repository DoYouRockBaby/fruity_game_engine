use crate::ui_element::DrawContext;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_editor::hooks::topo;
use fruity_editor::hooks::use_global;
use fruity_editor::hooks::use_memo;
use fruity_editor::state::world::WorldState;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::resources::texture_resource::TextureResource;
use fruity_wgpu_graphic::graphic_service::WgpuGraphicService;
use fruity_wgpu_graphic::resources::texture_resource::WgpuTextureResource;
use std::sync::Arc;
use std::sync::RwLock;

#[topo::nested]
pub fn draw_scene(ui: &mut egui::Ui, ctx: &mut DrawContext) {
    let width = (ui.available_width() / ui.input().physical_pixel_size()) as u32;
    let height = (ui.available_height() / ui.input().physical_pixel_size()) as u32;

    // Build the rendering texture
    let rendering_texture = use_memo(
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
            graphic_service.set_default_camera_rendering_texture(resource.clone());

            resource
        },
        (width, height),
    );

    // Get the egui identifier for the texture
    let image = rendering_texture.read();
    let image = image.downcast_ref::<WgpuTextureResource>();

    let egui_texture_id = ctx.egui_rpass.egui_texture_from_wgpu_texture(
        ctx.device,
        &image.texture,
        wgpu::FilterMode::Nearest,
    );

    // Display the scene
    ui.image(egui_texture_id, ui.available_size());
}

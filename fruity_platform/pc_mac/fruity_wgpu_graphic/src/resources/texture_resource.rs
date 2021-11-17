use crate::WgpuGraphicManager;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::resources::texture_resource::TextureResource;
use image::load_from_memory;
use image::GenericImageView;
use std::io::Read;
use std::num::NonZeroU32;
use std::sync::Arc;

#[derive(Debug, FruityAny)]
pub struct WgpuTextureResource {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl TextureResource for WgpuTextureResource {}

impl WgpuTextureResource {
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &image::DynamicImage,
        label: Option<&str>,
    ) -> Result<WgpuTextureResource, String> {
        let rgba = image.as_rgba8().unwrap();
        let dimensions = image.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * dimensions.0),
                rows_per_image: NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(WgpuTextureResource {
            texture,
            view,
            sampler,
        })
    }

    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float; // 1.

    pub fn new_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        label: &str,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT // 3.
                | wgpu::TextureUsages::TEXTURE_BINDING,
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }
}

impl Resource for WgpuTextureResource {}

impl IntrospectObject for WgpuTextureResource {
    fn get_class_name(&self) -> String {
        "TextureResource".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

pub fn load_texture(
    identifier: &str,
    reader: &mut dyn Read,
    _settings: Settings,
    resource_container: Arc<ResourceContainer>,
) {
    // read the whole file
    let mut buffer = Vec::new();
    if let Err(err) = reader.read_to_end(&mut buffer) {
        log::error!("{}", err.to_string());
        return;
    }

    // Load the image
    let image = load_from_memory(&buffer).unwrap();

    // Get the graphic manager state
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();
    let graphic_service = graphic_service.downcast_ref::<WgpuGraphicManager>();

    let device = graphic_service.get_device();
    let queue = graphic_service.get_queue();

    // Create the texture
    let resource = if let Ok(value) =
        WgpuTextureResource::from_image(device, queue, &image, Some(&identifier))
    {
        value
    } else {
        log::error!("Couldn't parse a texture");
        return;
    };

    // Store the texture
    if let Err(_) =
        resource_container.add::<dyn TextureResource>(identifier.clone(), Box::new(resource))
    {
        log::error!(
            "Couldn't add a resource cause the identifier \"{}\" already exists",
            identifier
        );
    }
}

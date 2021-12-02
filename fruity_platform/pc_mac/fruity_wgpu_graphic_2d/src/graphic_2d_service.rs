use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::material::Binding;
use fruity_graphic::math::material::BindingGroup;
use fruity_graphic::math::material::Material;
use fruity_graphic::math::matrix3::Matrix3;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;
use fruity_graphic::math::RED;
use fruity_graphic::resources::shader_resource::ShaderResource;
use fruity_graphic_2d::graphic_2d_service::Graphic2dService;
use fruity_wgpu_graphic::graphic_service::WgpuGraphicManager;
use fruity_wgpu_graphic::math::vertex::Vertex;
use fruity_wgpu_graphic::resources::shader_resource::WgpuShaderResource;
use fruity_wgpu_graphic::resources::texture_resource::WgpuTextureResource;
use fruity_windows::window_service::WindowService;
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[derive(Debug, FruityAny)]
pub struct WgpuGraphic2dManager {
    window_service: ResourceReference<dyn WindowService>,
    graphic_service: ResourceReference<dyn GraphicService>,
    resource_container: Arc<ResourceContainer>,
}

impl WgpuGraphic2dManager {
    pub fn new(resource_container: Arc<ResourceContainer>) -> WgpuGraphic2dManager {
        let window_service = resource_container.require::<dyn WindowService>();
        let graphic_service = resource_container.require::<dyn GraphicService>();

        WgpuGraphic2dManager {
            window_service,
            graphic_service,
            resource_container,
        }
    }
}

impl Graphic2dService for WgpuGraphic2dManager {
    fn start_pass(&self, view_proj: Matrix4) {
        let mut graphic_service = self.graphic_service.write();
        graphic_service.update_camera(view_proj);
        graphic_service.start_pass();
    }

    fn end_pass(&self) {
        let graphic_service = self.graphic_service.read();
        graphic_service.end_pass();
    }

    fn draw_square(&self, transform: Matrix3, z_index: usize, material: &Material) {
        let graphic_service = self.graphic_service.read();
        let graphic_service = graphic_service.downcast_ref::<WgpuGraphicManager>();

        let device = graphic_service.get_device();
        let config = graphic_service.get_config();

        // Get resources
        let shader = if let Some(shader) = &material.shader {
            shader
        } else {
            return;
        };

        let shader_reader = shader.read();
        let shader_reader = shader_reader.downcast_ref::<WgpuShaderResource>();

        // Create the main render pipeline
        let bottom_left = transform * Vector2d::new(-0.5, -0.5);
        let top_right = transform * Vector2d::new(0.5, 0.5);

        let vertices: &[Vertex] = &[
            Vertex {
                position: [bottom_left.x, bottom_left.y, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [top_right.x, bottom_left.y, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [top_right.x, top_right.y, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [bottom_left.x, top_right.y, 0.0],
                tex_coords: [0.0, 0.0],
            },
        ];

        let indices: &[u16] = &[0, 1, 2, 3, 0, 2, /* padding */ 0];
        let num_indices = indices.len() as u32;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let mut encoder =
            device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                label: Some("draw_square_bundle"),
                color_formats: &[config.format],
                depth_stencil: None,
                sample_count: 1,
            });

        let bind_groups = self.build_bind_groups(material);

        encoder.set_pipeline(&shader_reader.render_pipeline);
        bind_groups
            .iter()
            .enumerate()
            .for_each(|(index, bind_group)| {
                encoder.set_bind_group(index as u32, &bind_group, &[]);
            });
        encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
        encoder.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        encoder.draw_indexed(0..num_indices, 0, 0..1);
        let bundle = encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some("main"),
        });

        graphic_service.push_render_bundle(bundle, z_index);
    }

    fn draw_line(&self, pos1: Vector2d, pos2: Vector2d, width: u32, color: Color, z_index: usize) {
        let window_service = self.window_service.read();
        let graphic_service = self.graphic_service.read();
        let graphic_service = graphic_service.downcast_ref::<WgpuGraphicManager>();

        let device = graphic_service.get_device();
        let config = graphic_service.get_config();
        let camera_transform = graphic_service.get_camera_transform().clone();
        let viewport_size = window_service.get_size().clone();

        // Get resources
        let shader = self
            .resource_container
            .get::<dyn ShaderResource>("Shaders/Draw Line")
            .unwrap();
        let shader_reader = shader.read();
        let shader_reader = shader_reader.downcast_ref::<WgpuShaderResource>();

        // Calculate the geometry
        let diff = pos2 - pos1;
        let mut thick_vec = diff.normal().normalise() * width as f32;
        thick_vec.x /= viewport_size.0 as f32;
        thick_vec.y /= viewport_size.1 as f32;

        let pos1 = camera_transform * pos1;
        let pos2 = camera_transform * pos2;

        let vertices: &[Vertex] = &[
            Vertex {
                position: [pos1.x - thick_vec.x, pos1.y - thick_vec.y, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [pos1.x + thick_vec.x, pos1.y + thick_vec.y, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [pos2.x + thick_vec.x, pos2.y + thick_vec.y, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [pos2.x - thick_vec.x, pos2.y - thick_vec.y, 0.0],
                tex_coords: [0.0, 0.0],
            },
        ];

        let indices: &[u16] = &[2, 1, 0, 2, 0, 3, /* padding */ 0];
        let num_indices = indices.len() as u32;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Update material color
        let color_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[color.clone()]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Color buffer"),
            layout: &shader_reader.binding_groups_layout.get(0).unwrap(),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: color_buffer.as_entire_binding(),
            }],
        });

        // Draw
        let mut encoder =
            device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                label: None,
                color_formats: &[config.format],
                depth_stencil: None,
                sample_count: 1,
            });

        encoder.set_pipeline(&shader_reader.render_pipeline);
        encoder.set_bind_group(0, &bind_group, &[]);
        encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
        encoder.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        encoder.draw_indexed(0..num_indices, 0, 0..1);
        let bundle = encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some("main"),
        });

        graphic_service.push_render_bundle(bundle, z_index);
    }

    fn get_cursor_position(&self) -> Vector2d {
        let window_service = self.window_service.read();
        let graphic_service = self.graphic_service.read();

        // Get informations from the resource dependencies
        let cursor_position = window_service.get_cursor_position();
        let viewport_size = window_service.get_size();
        let camera_transform = graphic_service.get_camera_transform().clone();
        std::mem::drop(graphic_service);
        std::mem::drop(window_service);

        // Transform the cursor in the engine world (especialy taking care of camera)
        let cursor_pos = Vector2d::new(
            (cursor_position.0 as f32 / viewport_size.0 as f32) * 2.0 - 1.0,
            (cursor_position.1 as f32 / viewport_size.1 as f32) * -2.0 + 1.0,
        );

        camera_transform.invert() * cursor_pos
    }
}

impl WgpuGraphic2dManager {
    fn build_bind_groups(&self, material: &Material) -> Vec<wgpu::BindGroup> {
        let graphic_service = self.graphic_service.read();
        let graphic_service = graphic_service.downcast_ref::<WgpuGraphicManager>();
        let device = graphic_service.get_device();

        let shader = if let Some(shader) = material.shader.as_ref().map(|shader| shader.read()) {
            shader
        } else {
            return Vec::new();
        };
        let shader = shader.downcast_ref::<WgpuShaderResource>();

        material
            .binding_groups
            .iter()
            .enumerate()
            .map(|(index, binding_group)| {
                match binding_group {
                    BindingGroup::Camera => device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &graphic_service.get_camera_bind_group_layout(),
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: graphic_service.get_camera_buffer().as_entire_binding(),
                        }],
                        label: Some("camera_bind_group"),
                    }),
                    BindingGroup::Custom(bindings) => {
                        device.create_bind_group(&wgpu::BindGroupDescriptor {
                            // TODO: Error message in case the material don't match the shader
                            layout: &shader.binding_groups_layout.get(index).unwrap(),
                            entries: &bindings
                                .into_iter()
                                .enumerate()
                                .map(|(index, binding)| {
                                    match binding {
                                        Binding::Texture(texture) => {
                                            let texture = texture.read();
                                            let texture =
                                                texture.downcast_ref::<WgpuTextureResource>();

                                            // TODO: Find a way to remove it
                                            let texture = unsafe {
                                                std::mem::transmute::<
                                                    &WgpuTextureResource,
                                                    &WgpuTextureResource,
                                                >(
                                                    texture
                                                )
                                            };

                                            wgpu::BindGroupEntry {
                                                binding: index as u32,
                                                resource: wgpu::BindingResource::TextureView(
                                                    &texture.view,
                                                ),
                                            }
                                        }
                                        Binding::Sampler(texture) => {
                                            let texture = texture.read();
                                            let texture =
                                                texture.downcast_ref::<WgpuTextureResource>();

                                            // TODO: Find a way to remove it
                                            let texture = unsafe {
                                                std::mem::transmute::<
                                                    &WgpuTextureResource,
                                                    &WgpuTextureResource,
                                                >(
                                                    texture
                                                )
                                            };

                                            wgpu::BindGroupEntry {
                                                binding: index as u32,
                                                resource: wgpu::BindingResource::Sampler(
                                                    &texture.sampler,
                                                ),
                                            }
                                        }
                                        Binding::Uniform => {
                                            let color = RED;
                                            let color_buffer = device.create_buffer_init(
                                                &wgpu::util::BufferInitDescriptor {
                                                    label: Some("Uniform Buffer"),
                                                    contents: bytemuck::cast_slice(
                                                        &[color.clone()],
                                                    ),
                                                    usage: wgpu::BufferUsages::UNIFORM
                                                        | wgpu::BufferUsages::COPY_DST,
                                                },
                                            );

                                            // TODO: Find a way to remove it
                                            let color_buffer = unsafe {
                                                std::mem::transmute::<&wgpu::Buffer, &wgpu::Buffer>(
                                                    &color_buffer,
                                                )
                                            };

                                            wgpu::BindGroupEntry {
                                                binding: 0,
                                                resource: color_buffer.as_entire_binding(),
                                            }
                                        }
                                    }
                                })
                                .collect::<Vec<_>>(),
                            label: None,
                        })
                    }
                }
            })
            .collect::<Vec<_>>()
    }
}

impl IntrospectObject for WgpuGraphic2dManager {
    fn get_class_name(&self) -> String {
        "Graphic2dManager".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for WgpuGraphic2dManager {}

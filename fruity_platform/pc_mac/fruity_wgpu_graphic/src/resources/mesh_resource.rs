use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::resource::resource::Resource;
use fruity_graphic::resources::mesh_resource::MeshResource;
use fruity_graphic::resources::mesh_resource::MeshResourceSettings;
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[derive(Debug, FruityAny)]
pub struct WgpuMeshResource {
    pub params: MeshResourceSettings,
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: usize,
    pub index_buffer: wgpu::Buffer,
    pub index_count: usize,
}

impl WgpuMeshResource {
    pub fn new(
        device: &wgpu::Device,
        label: &str,
        params: &MeshResourceSettings,
    ) -> WgpuMeshResource {
        // Create the buffers
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{}_vertex", label)),
            contents: bytemuck::cast_slice(&params.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{}_index", label)),
            contents: bytemuck::cast_slice(&params.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        WgpuMeshResource {
            params: params.clone(),
            vertex_buffer,
            vertex_count: params.vertices.len(),
            index_buffer,
            index_count: params.indices.len(),
        }
    }
}

impl MeshResource for WgpuMeshResource {}

impl Resource for WgpuMeshResource {}

impl IntrospectObject for WgpuMeshResource {
    fn get_class_name(&self) -> String {
        "ShaderResource".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![FieldInfo {
            name: "params".to_string(),
            serializable: true,
            getter: Arc::new(|this| {
                this.downcast_ref::<WgpuMeshResource>()
                    .unwrap()
                    .params
                    .clone()
                    .fruity_into()
            }),
            setter: SetterCaller::Mut(std::sync::Arc::new(|this, value| {
                let this = this.downcast_mut::<WgpuMeshResource>().unwrap();

                match MeshResourceSettings::fruity_try_from(value) {
                    Ok(value) => this.params = value,
                    Err(_) => {
                        log::error!("Expected a ShaderParams for property params");
                    }
                }
            })),
        }]
    }
}

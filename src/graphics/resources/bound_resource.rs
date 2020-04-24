use crate::AssetManager;
use super::BindGroup;

pub trait BoundResource {
    fn create_bind_group<'a>(&mut self, asset_manager: &AssetManager, device: &wgpu::Device, pipeline_layouts: &'a Vec<wgpu::BindGroupLayout>) -> BindGroup;
}
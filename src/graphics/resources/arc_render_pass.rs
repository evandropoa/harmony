use super::BindGroup;
use crate::graphics::pipeline_manager::Pipeline;
use std::{ops::Range, sync::Arc};
use typed_arena::Arena;

pub struct ArcRenderPass<'a> {
    buffer_arena: &'a Arena<Arc<wgpu::Buffer>>,
    internal_bind_group_arena: &'a Arena<Arc<BindGroup>>,
    render_pass: wgpu::RenderPass<'a>,
}

impl<'a> ArcRenderPass<'a> {
    pub fn new(
        buffer_arena: &'a Arena<Arc<wgpu::Buffer>>,
        internal_bind_group_arena: &'a Arena<Arc<BindGroup>>,
        render_pass: wgpu::RenderPass<'a>,
    ) -> Self {
        Self {
            buffer_arena,
            internal_bind_group_arena,
            render_pass,
        }
    }

    pub fn set_bind_group_internal(&mut self, bind_group: Arc<BindGroup>) {
        let bind_group = self.internal_bind_group_arena.alloc(bind_group);
        self.render_pass
            .set_bind_group(bind_group.index, &bind_group.group, &[]);
    }

    pub fn set_bind_group(
        &mut self,
        slot: u32,
        bind_group: &'a wgpu::BindGroup,
        offset: &[wgpu::DynamicOffset],
    ) {
        self.render_pass.set_bind_group(slot, bind_group, offset);
    }

    pub fn set_vertex_buffer(&mut self, slot: u32, buffer: Arc<wgpu::Buffer>) {
        let buffer = self.buffer_arena.alloc(buffer);
        self.render_pass.set_vertex_buffer(slot, buffer.slice(..));
    }

    pub fn set_index_buffer(&mut self, buffer: Arc<wgpu::Buffer>) {
        let buffer = self.buffer_arena.alloc(buffer);
        self.render_pass.set_index_buffer(buffer.slice(..));
    }

    pub fn set_pipeline(&mut self, pipeline: &'a Pipeline) {
        self.render_pass.set_pipeline(&pipeline.render_pipeline);
    }

    pub fn set_push_constants(&mut self, stages: wgpu::ShaderStage, offset: u32, data: &[u32]) {
        self.render_pass.set_push_constants(stages, offset, data);
    }

    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.render_pass
            .draw_indexed(indices, base_vertex, instances);
    }

    pub fn set_viewport(&mut self, x: f32, y: f32, w: f32, h: f32, min_depth: f32, max_depth: f32) {
        self.render_pass.set_viewport(x, y, w, h, min_depth, max_depth);
    }
}

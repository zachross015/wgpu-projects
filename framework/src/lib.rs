pub mod wgpu_context;

pub struct RenderPassBuilder<'tex> {
    view: &'tex wgpu::TextureView,
    color_attachments: Vec<Option<wgpu::RenderPassColorAttachment<'tex>>>
}

impl<'tex> RenderPassBuilder<'tex> {

    pub fn new(view: &'tex wgpu::TextureView) -> Self {
        Self { 
            view,
            color_attachments: Vec::new()
        }
    }

    pub fn clear(mut self, color: wgpu::Color) -> Self {
        let attachment = wgpu::RenderPassColorAttachment {
            view: self.view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(color),
                store: wgpu::StoreOp::Store,
            }
        };

        self.color_attachments.push(Some(attachment));
        self
    }

    pub fn build<'desc>(&'desc self) -> wgpu::RenderPassDescriptor<'tex, 'desc> {
        let desc = wgpu::RenderPassDescriptor::<'tex, 'desc> {
            label: None,
            color_attachments: &self.color_attachments,
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None
        };
        desc
    }

}


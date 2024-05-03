pub struct RenderPassBuilder<'tex> {
    color_attachments: Vec<Option<wgpu::RenderPassColorAttachment<'tex>>>
}


impl<'tex> RenderPassBuilder<'tex> {
    pub fn new() -> Self {
        Self { 
            color_attachments: Vec::new()
        }
    }

    pub fn clear(mut self, view: &'tex wgpu::TextureView, color: wgpu::Color) -> Self {
        let attachment = wgpu::RenderPassColorAttachment::<'tex> {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(color),
                store: wgpu::StoreOp::Store,
            }
        };

        self.color_attachments.push(Some(attachment));
        self
    }

    pub fn build(self, encoder: &'tex mut wgpu::CommandEncoder) -> wgpu::RenderPass {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &self.color_attachments,
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None
            })
    }
}


use crate::shader::Shader;

pub struct RenderPassInitBuilder<'tex> {
    color_attachments: Vec<Option<wgpu::RenderPassColorAttachment<'tex>>>
}


impl<'tex> RenderPassInitBuilder<'tex> {
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

    pub fn begin(self, encoder: &'tex mut wgpu::CommandEncoder) -> RenderPass {
        RenderPass {
            render_pass: encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &self.color_attachments,
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None
            })
        }
    }
}

pub struct RenderPass<'tex> {
    render_pass: wgpu::RenderPass<'tex>,
}

impl<'tex> RenderPass<'tex> {
    pub fn new() -> RenderPassInitBuilder<'tex> {
        RenderPassInitBuilder { 
            color_attachments: Vec::new()
        }
    }

    pub fn draw<S: Shader>(&mut self, shader: &'tex S) {
        shader.render_to(&mut self.render_pass)
    }
}

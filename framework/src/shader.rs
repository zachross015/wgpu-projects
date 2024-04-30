pub trait Shader {
    fn render_to<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}

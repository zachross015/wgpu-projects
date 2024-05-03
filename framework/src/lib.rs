pub mod wgpu_context;
pub mod builder;

pub use builder::*;
pub use wgpu_context::*;


/* 
   Reduces the boiler plate involved in cosntructing a very basic render pass by taking the
   context, render color, and render function, and expanding them into code.

   ## Parameters

   - `$context`: the `WgpuContext` to apply the render pass to
   - `$clear`: the `wgpu::Color` to clear the screen with at the beginning of the pass
   - `$rpass in $code`: expression denotine the name of the variable to assign the render pass
   variable along with the code itself to give this variable

   ## Examples

   ```
   basic_render_pass!(context, BLUE, rpass in {
   rpass.push_debug_group("Setting pipeline");
   rpass.set_pipeline(&shader.pipeline);
   rpass.set_bind_group(0, &shader.bind_group, &[]);
   rpass.set_vertex_buffer(0, shader.vertex_buffer.slice(..));
   rpass.set_index_buffer(shader.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
   rpass.pop_debug_group();
   rpass.push_debug_group("Preparing to draw");
   rpass.draw_indexed(0..24, 0, 0..1);
   rpass.pop_debug_group();
   });
   ```
   */ 
#[macro_export]
macro_rules! basic_render_pass {
    ($context:ident, $clear:ident, $rpass:ident in $code:expr) => {

        let (frame, frame_view) = $context.frame_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = $context.command_encoder();

        {
            let mut $rpass = RenderPassBuilder::new()
                .clear(&frame_view, wgpu::Color::$clear)
                .build(&mut encoder);

            $code
        }

        $context.queue.submit(Some(encoder.finish()));
        frame.present();
    };
}

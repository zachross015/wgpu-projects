use std::sync::Arc;

use wgpu::RequestAdapterOptions;
use winit::window::Window;


pub struct WgpuContext {
    pub window: Arc<Window>,
    pub surface: wgpu::Surface<'static>,
    pub adapter: wgpu::Adapter,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}


impl WgpuContext {
    pub async fn from_window(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&RequestAdapterOptions{
                power_preference: wgpu::PowerPreference::None,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height).unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        WgpuContext {
            window: window,
            surface: surface,
            adapter: adapter,
            surface_config: surface_config,
            device: device,
            queue: queue,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);
        self.window.request_redraw();
    }

    /** Uses the surface and adapter 
     *
     */
    pub fn swapchain_format(&self) -> wgpu::TextureFormat {
       self.surface.get_capabilities(&self.adapter).formats[0]
    }

    pub fn frame_view(&self, descriptor: &wgpu::TextureViewDescriptor) -> (wgpu::SurfaceTexture, wgpu::TextureView) {
        let frame = self.surface.get_current_texture().expect("Failed to acquire next swap chain texture.");
        let view = frame.texture.create_view(descriptor);
        (frame, view)
    }

    pub fn command_encoder(&self) -> wgpu::CommandEncoder {
        self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default())
    }

    /** Performs a render pass using the parameterized function `render_pass` under the current context. The code simplifies some of the boilerplate code by collecting the frame, view, encoder, and queue from the context or other defaults, then passes the `frame_view: &wgpu::TextureView` and `command_encoder: &mut wgpu::CommandEncoder` to the paramterized function. 


     */
    pub fn perform_render_pass<F>(&self, render_pass: F) where F: Fn(&wgpu::TextureView, &mut wgpu::CommandEncoder) {
        let (frame, frame_view) = self.frame_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.command_encoder();

        render_pass(&frame_view, &mut encoder);

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}


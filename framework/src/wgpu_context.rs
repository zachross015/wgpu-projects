use std::sync::Arc;

use wgpu::RequestAdapterOptions;
use winit::window::Window;

use crate::RenderPassBuilder;

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
            .expect("Error in getting the underlying adapter for the WgpuContext");
        let surface_config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .expect("Error in retrieving the surface config fromt he adapter");
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .expect("Error in retrieving the device and queue from the adapter");

        surface.configure(&device, &surface_config);

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
}

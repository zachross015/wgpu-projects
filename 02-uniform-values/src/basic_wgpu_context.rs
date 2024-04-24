use std::sync::Arc;

use wgpu::RequestAdapterOptions;
use winit::window::Window;


pub struct BasicWgpuContext {
    pub window: Arc<Window>,
    pub surface: wgpu::Surface<'static>,
    pub adapter: wgpu::Adapter,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}


impl BasicWgpuContext {
    pub async fn new(window: Arc<Window>) -> Self {

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
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
            }, None)
            .await
            .unwrap();

        BasicWgpuContext {
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
}

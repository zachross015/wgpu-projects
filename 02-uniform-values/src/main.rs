// IDEA: Build a "builder" program for constructing simple pipelines that packs them automatically
// E.g.
// SimplePipelineBuilder::new()
//     .add_uniform::<AppState>() * calls create_uniform_buffer and automatically adds this to the
//                                  bind group layout and bind group
//
// Any bufferable object must derive from a trait which can specify its byte requirements 

mod app_state;

use std::sync::Arc;

use framework::wgpu_context::WgpuContext;
use app_state::AppState;
use wgpu::{BindGroupEntry, BindGroupLayout, Buffer, CommandEncoderDescriptor, Device, FragmentState, PipelineLayout, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, ShaderModule, TextureFormat, TextureViewDescriptor, VertexState};
use winit::{dpi::LogicalSize, event::{Event, KeyEvent, WindowEvent}, event_loop::EventLoop, keyboard::{Key, NamedKey}, window::{Window, WindowBuilder}};

struct ShaderProgram {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
}


impl ShaderProgram {

    fn create_shader_module(device: &Device) -> ShaderModule {
        device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl")))
            })
    }

    fn create_uniform_buffer(device: &Device) -> Buffer {
        device.create_buffer(
            &wgpu::BufferDescriptor {
                label: None,
                size: std::mem::size_of::<AppState>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
    }

    fn create_bind_group(device: &Device, uniform_buffer: &wgpu::Buffer) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor { 
                label: None, 
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer { 
                            ty: wgpu::BufferBindingType::Uniform, 
                            has_dynamic_offset: false, 
                            min_binding_size: None 
                        },
                        count: None,
                    }
                ]
            });

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor { 
                label: None, 
                layout: &layout, 
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer: uniform_buffer,
                            offset: 0,
                            size: None,
                        })
                    }
                ]
            });
        (layout, bind_group)
    }

    fn get_swapchain_texture_format(context: &WgpuContext) -> wgpu::TextureFormat {
        let swapchain_capabilities = context.surface.get_capabilities(&context.adapter);
        swapchain_capabilities.formats[0]
    }

    fn create_pipeline_layout(device: &Device, bind_group_layout: BindGroupLayout) -> PipelineLayout {
        device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor { 
                label: None, 
                bind_group_layouts: &[&bind_group_layout], 
                push_constant_ranges: &[] 
            })
    }

    fn create_render_pipeline(
        device: &Device, 
        pipeline_layout: PipelineLayout, 
        shader_module: ShaderModule, 
        swapchain_format: TextureFormat) -> RenderPipeline {

        device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor { 
                label: None, 
                layout: Some(&pipeline_layout), 
                vertex: VertexState {
                    module: &shader_module,
                    entry_point: "vs_main",
                    buffers: &[]
                }, 
                fragment: Some(FragmentState {
                    module: &shader_module,
                    entry_point: "fs_main",
                    targets: &[Some(swapchain_format.into())]
                }), 
                primitive: wgpu::PrimitiveState::default(), 
                depth_stencil: None, 
                multisample: wgpu::MultisampleState::default(), 
                multiview: None, 
            })
        
    }

    fn new(context: &WgpuContext) -> Self {

        let device = &context.device;
        
        let uniform_buffer = ShaderProgram::create_uniform_buffer(&device);
        let (bind_group_layout, bind_group) = ShaderProgram::create_bind_group(&device, &uniform_buffer);
        let pipeline_layout = ShaderProgram::create_pipeline_layout(&device, bind_group_layout);

        // Create the shader module on the device from the passed program
        let shader_module = ShaderProgram::create_shader_module(&device);
        let swapchain_format = ShaderProgram::get_swapchain_texture_format(context);

        let render_pipeline = ShaderProgram::create_render_pipeline(&device, pipeline_layout, shader_module, swapchain_format);

        Self {
            pipeline: render_pipeline,
            bind_group: bind_group,
            uniform_buffer: uniform_buffer
        }
    }
}

async fn run(event_loop: EventLoop<()>, window: Arc<Window>) {
    let mut context = Some(WgpuContext::from_window(window).await);
    let mut state = Some(AppState::default());
    let mut shader_program = Some(ShaderProgram::new(context.as_ref().unwrap()));
    let main_window_id = context.as_ref().unwrap().window.id();

    event_loop.run(move |event, target| {
        match event {
            Event::LoopExiting => {
                context = None;
                state = None;
                shader_program = None;
            }
            Event::WindowEvent { window_id, event } if window_id == main_window_id => {
                match event {
                    WindowEvent::CloseRequested => {
                        target.exit();
                    }
                    WindowEvent::Resized(new_size) => {
                        let context = context.as_mut().unwrap();
                        context.resize(new_size);
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        let change = match delta {
                            winit::event::MouseScrollDelta::LineDelta(_x, y) => y,
                            winit::event::MouseScrollDelta::PixelDelta(pos) => { pos.y as f32 }
                        } / 20.0;
                        let state = state.as_mut().unwrap();
                        let context = context.as_ref().unwrap();

                        state.zoom(change);
                        context.window.request_redraw();
                    }
                    WindowEvent::KeyboardInput { event: KeyEvent { logical_key, text, .. }, .. } => {
                        
                        let context = context.as_ref().unwrap();
                        let state = state.as_mut().unwrap();

                        if let Key::Named(key) = logical_key {
                            match key {
                                NamedKey::Escape => target.exit(),
                                NamedKey::ArrowUp => state.translate_view(1, 1),
                                NamedKey::ArrowDown => state.translate_view(-1, 1),
                                NamedKey::ArrowLeft => state.translate_view(1, 0),
                                NamedKey::ArrowRight => state.translate_view(-1, 0),
                                _ => {}
                            }
                        }

                        if let Some(text) = text {
                            if text == "u" {
                                state.max_iterations += 3;
                            } else if text == "v" {
                                state.max_iterations -= 3;
                            }
                        }

                        context.window.request_redraw();

                    }
                    WindowEvent::RedrawRequested => {

                        let context = context.as_ref().unwrap();
                        let state = state.as_ref().unwrap();
                        let shader_program = shader_program.as_ref().unwrap();
                        let frame = context.surface.get_current_texture().expect("Unable to retrieve surface texture");
                        let view = frame.texture.create_view(&TextureViewDescriptor::default());

                        // Build the actual render pass
                        context.queue.write_buffer(&shader_program.uniform_buffer, 0, &state.as_wgsl_bytes().expect("Error in translating AppState to wgsl bytes."));
                        let mut encoder = context.device.create_command_encoder(&CommandEncoderDescriptor::default());
                        {
                            let mut rpass = encoder.begin_render_pass(
                                &RenderPassDescriptor { 
                                    label: None, 
                                    color_attachments: &[
                                        Some(RenderPassColorAttachment {
                                            view: &view,
                                            resolve_target: None,
                                            ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::GREEN), store: wgpu::StoreOp::Store }
                                        })
                                    ], 
                                    depth_stencil_attachment: None, 
                                    timestamp_writes: None, 
                                    occlusion_query_set: None
                                });

                            rpass.set_pipeline(&shader_program.pipeline);
                            rpass.set_bind_group(0, &shader_program.bind_group, &[]);
                            rpass.draw(0..3, 0..1);
                        }
                        context.queue.submit(Some(encoder.finish()));
                        frame.present();
                    }
                    _ => {},
                }
            }
            _ => {},
        }
    }).unwrap();
}


fn main() {
    let event_loop = EventLoop::new().unwrap();
    let builder = WindowBuilder::new().with_title("Working with uniforms").with_inner_size(LogicalSize::new(900, 900));
    let window = builder.build(&event_loop).unwrap();
    let window = Arc::new(window);

    pollster::block_on(run(event_loop, window));
}

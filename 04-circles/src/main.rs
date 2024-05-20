use std::sync::Arc;

use framework::{basic_render_pass, RenderPassBuilder, WgpuContext};
use wgpu::{include_wgsl, vertex_attr_array};
use winit::{event::WindowEvent, event_loop::EventLoop, window::Window};


struct Shader {
    bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
}


fn create_shader_pipeline(context: &WgpuContext) -> Shader {

    // Create vertex buffer
    let device = &context.device;
    let formats = context.swapchain_format();
    
    // Shader
    let positions = [
        -1.0f32, -1.0,
        -1.0, 1.0,
        1.0, 1.0,
        1.0, -1.0,
    ];
    let vertex_buffer = framework::BufferBuilder::vertex(&positions)
        .build(&device);

    let index: [u16; 6] = [0, 1, 2, 2, 3, 0];
    let index_buffer = framework::BufferBuilder::index(&index)
        .build(&device);

    let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));

    let instance_centers_and_radii = [
        300f32, 300.0, 100.0,
        100.0, 100.0, 400.0,
        0.0, 0.0, 550.0,
    ];
    let instance_buffer = framework::BufferBuilder::slice_of(&instance_centers_and_radii)
        .usage(wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST)
        .build(&device);


    let vertex_state = wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &[
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &vertex_attr_array![ 0 => Float32x2 ],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &vertex_attr_array![ 1 => Float32x2, 2 => Float32 ],
            },
        ]
    };

    let fragment_state = wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[Some(formats.into())],
    };

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                count: None
            }
        ]
    });

    let layout = framework::PipelineLayoutBuilder::new()
        .add_bind_group_layout(&bind_group_layout)
        .build(&device);

    let width = context.surface_config.width as f32;
    let height = context.surface_config.height as f32;
    let resolution = [width, height];
    let uniform_buffer = framework::BufferBuilder::slice_of(&resolution)
        .usage(wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST)
        .build(&device);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &bind_group_layout, entries: &[
        wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding()
        }
    ] });


    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { 
        label: None, 
        layout: Some(&layout), 
        vertex: vertex_state, 
        primitive: wgpu::PrimitiveState::default(), 
        depth_stencil: None, 
        multisample: wgpu::MultisampleState::default(), 
        fragment: Some(fragment_state), 
        multiview: None 
    });

    Shader {
        bind_group,
        uniform_buffer,
        instance_buffer,
        vertex_buffer,
        index_buffer,
        pipeline,
    }
}


pub async fn run(event_loop: EventLoop<()>, window: Arc<Window>) {
    use winit::event::Event;

    let mut context = Some(WgpuContext::from_window(window).await);
    let mut shader = Some(create_shader_pipeline(&context.as_ref().unwrap()));

    event_loop.run(move |event, target| {
        match event {
            Event::LoopExiting => {
                context = None;
                shader = None;
            }
            Event::WindowEvent { window_id: _window_id, event } => {
                match event {
                    WindowEvent::CloseRequested => {
                        target.exit();
                    }
                    WindowEvent::Resized(new_size) => {
                        let context = context.as_mut().unwrap();
                        context.resize(new_size);

                        let width = context.surface_config.width as f32;
                        let height = context.surface_config.height as f32;
                        let resolution = [width, height];
                        let shader = shader.as_ref().unwrap();
                        context.queue.write_buffer(&shader.uniform_buffer, 0, bytemuck::bytes_of(&resolution));

                    }
                    WindowEvent::RedrawRequested => {

                        let context = context.as_mut().unwrap();
                        let shader = shader.as_ref().unwrap();

                        basic_render_pass!(context, BLACK, rpass in {
                            rpass.push_debug_group("Setting pipeline");
                            rpass.set_pipeline(&shader.pipeline);
                            rpass.set_bind_group(0, &shader.bind_group, &[]);
                            rpass.set_vertex_buffer(0, shader.vertex_buffer.slice(..));
                            rpass.set_vertex_buffer(1, shader.instance_buffer.slice(..));
                            rpass.set_index_buffer(shader.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                            rpass.pop_debug_group();
                            rpass.push_debug_group("Preparing to draw");
                            rpass.draw_indexed(0..6, 0, 0..3);
                            // rpass.draw
                            rpass.pop_debug_group();
                        });

                        // For shader updates
                        context.window.request_redraw();

                    }
                    _ => {}
                }
            }
            _ => {}
        }

    }).unwrap();
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    #[allow(unused_mut)]
    let mut builder = winit::window::WindowBuilder::new()
        .with_title("Remember: Use U/D to change sample count!")
        .with_inner_size(winit::dpi::LogicalSize::new(900, 900));

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        builder = builder.with_canvas(Some(canvas));
    }
    let window = builder.build(&event_loop).unwrap();

    let window = Arc::new(window);
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::builder().format_timestamp_nanos().init();
        pollster::block_on(run(event_loop, window));
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");

        let document = web_sys::window()
            .and_then(|win| win.document())
            .expect("Failed to get document.");
        let body = document.body().unwrap();
        let controls_text = document
            .create_element("p")
            .expect("Failed to create controls text as element.");
        controls_text.set_inner_html(
            "Controls: <br/>
Up, Down, Left, Right: Move view, <br/>
Scroll: Zoom, <br/>
U, D: Increase / decrease sample count.",
);
        body.append_child(&controls_text)
            .expect("Failed to append controls text to body.");

        wasm_bindgen_futures::spawn_local(run(event_loop, window));
    }
}

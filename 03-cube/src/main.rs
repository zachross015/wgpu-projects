// Improvements: Build a texture atlas dynamically via creating texel lookup
//
use std::sync::Arc;

use encase::ShaderType;
use framework::{WgpuContext, BufferBuilder, RenderPassBuilder, basic_render_pass};
use wgpu::{include_wgsl, vertex_attr_array, Buffer};
use winit::{event::WindowEvent, event_loop::EventLoop, window::Window};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(ShaderType, Copy, Clone, Pod, Zeroable)]
struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
}

impl Vertex {
    fn new((x, y, z): (f32, f32, f32) , c: (f32, f32, f32, f32)) -> Self {
        Vertex {
            position: [x, y, z, 0.0],
            color: c.into()
        }
    }
}



fn cube() -> (Vec<Vertex>, Vec<u16>) {
    let positions = [
        // front
        (-1.0, -1.0, 1.0), // 0
        (1.0, -1.0, 1.0), // 1
        (1.0, 1.0, 1.0), // 2
        (-1.0, 1.0, 1.0), // 3
                         // bottom
        (-1.0, 1.0, -1.0), // 4
        (1.0, 1.0, -1.0), // 5
        (1.0, -1.0, -1.0), // 6
        (-1.0, -1.0, -1.0), // 7
                         // right
        (1.0, -1.0, -1.0), // 8
        (1.0, 1.0, -1.0), // 9
        (1.0, 1.0, 1.0), // 10
        (1.0, -1.0, 1.0), // 11
                         // top
        (-1.0, -1.0, 1.0), // 12
        (-1.0, 1.0, 1.0), // 13
        (-1.0, 1.0, -1.0), // 14
        (-1.0, -1.0, -1.0), // 15
                         // bottom
        (1.0, 1.0, -1.0), // 16
        (-1.0, 1.0, -1.0), // 17
        (-1.0, 1.0, 1.0), // 18
        (1.0, 1.0, 1.0), // 19
                         // back
        (1.0, -1.0, 1.0), // 20
        (-1.0, -1.0, 1.0), // 21
        (-1.0, -1.0, -1.0), // 22
        (1.0, -1.0, -1.0), // 23
        ];

    let colors = [
        // front
        (1.0, 0.0, 0.0, 1.0), // 0
        (1.0, 0.0, 0.0, 1.0), // 1
        (1.0, 0.0, 0.0, 1.0), // 2
        (1.0, 0.0, 0.0, 1.0), // 3
                              // left
        (0.0, 1.0, 0.0, 1.0), // 4
        (0.0, 1.0, 0.0, 1.0), // 5
        (0.0, 1.0, 0.0, 1.0), // 6
        (0.0, 1.0, 0.0, 1.0), // 7
                              // right
        (0.0, 0.0, 1.0, 1.0), // 8
        (0.0, 0.0, 1.0, 1.0), // 9
        (0.0, 0.0, 1.0, 1.0), // 10
        (0.0, 0.0, 1.0, 1.0), // 11
                              // top
        (1.0, 1.0, 0.0, 1.0), // 12
        (1.0, 1.0, 0.0, 1.0), // 13
        (1.0, 1.0, 0.0, 1.0), // 14
        (1.0, 1.0, 0.0, 1.0), // 15
                              // bottom
        (1.0, 0.0, 1.0, 1.0), // 16
        (1.0, 0.0, 1.0, 1.0), // 17
        (1.0, 0.0, 1.0, 1.0), // 18
        (1.0, 0.0, 1.0, 1.0), // 19
                              // back
        (0.0, 1.0, 1.0, 1.0), // 20
        (0.0, 1.0, 1.0, 1.0), // 21
        (0.0, 1.0, 1.0, 1.0), // 22
        (0.0, 1.0, 1.0, 1.0), // 23

        ];

    let indices = [
        0, 1, 2, 2, 3, 0,
        4, 5, 6, 6, 7, 4,
        8, 9, 10, 10, 11, 8,
        12, 13, 14, 14, 15, 12,
        16, 17, 18, 18, 19, 16,
        20, 21, 22, 22, 23, 20
    ];

    let vertices = positions.iter().zip(colors.iter()).map(|(p, c)| { Vertex::new(*p, *c)}).collect();

    (vertices, indices.to_vec())
}

struct Shader {
    bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
}

impl Shader {

    fn new(wgpu_context: &WgpuContext) -> Self {
        // Pre-Initialize shortcuts

        let device = &wgpu_context.device;
        let format = wgpu_context.swapchain_format();

        // construct the module

        let module = device.create_shader_module(include_wgsl!("shader.wgsl"));

        // Construct the pipeline by building the various layout requirements

        let buffer_layout = wgpu::VertexBufferLayout { 
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, 
            step_mode: wgpu::VertexStepMode::Vertex, 
            attributes: &vertex_attr_array![ 0 => Float32x4, 1 => Float32x4 ],
        };

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    count: None,
                    ty: wgpu::BindingType::Buffer { 
                        has_dynamic_offset: false, 
                        ty: wgpu::BufferBindingType::Uniform,
                        min_binding_size: wgpu::BufferSize::new(4),
                    }
                }
            ]
        });

        let time: f32 = 0.0;

        let uniform_buffer = BufferBuilder::bytes_of(&time)
            .usage(wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST)
            .build(device);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }
            ]
        });

        let layout = framework::PipelineLayoutBuilder::new()
            .add_bind_group_layout(&layout)
            .build(device);


        // Build the initial buffers
        let (vertex_data, index_data) = cube();

        let vertex_buffer = BufferBuilder::slice_of(&vertex_data)
            .usage(wgpu::BufferUsages::VERTEX)
            .build(device);

        let index_buffer = BufferBuilder::slice_of(&index_data)
            .usage(wgpu::BufferUsages::INDEX)
            .build(device);


        let vertex_state = wgpu::VertexState {
            module: &module,
            entry_point: "vs_main",
            buffers: &[buffer_layout],
        };

        let fragment_state = wgpu::FragmentState {
            module: &module,
            entry_point: "fs_main",
            targets: &[Some(format.into())]
        };

        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor { 
                label: None, 
                layout: Some(&layout), 
                vertex: vertex_state, 
                primitive: wgpu::PrimitiveState::default(), 
                depth_stencil: None, 
                multisample: wgpu::MultisampleState::default(), 
                fragment: Some(fragment_state), 
                multiview: None 
            });

        Shader { bind_group, uniform_buffer, vertex_buffer, index_buffer, pipeline }
    }
}



pub async fn run(event_loop: EventLoop<()>, window: Arc<Window>) {
    use winit::event::Event;

    let mut context = Some(WgpuContext::from_window(window).await);
    let mut shader = Some(Shader::new(context.as_ref().unwrap()));

    let start = std::time::Instant::now();

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
                    }
                    WindowEvent::RedrawRequested => {

                        let context = context.as_mut().unwrap();
                        let shader = shader.as_ref().unwrap();

                        let time: f32 = std::time::Instant::now().duration_since(start).as_secs_f32();
                        context.queue.write_buffer(&shader.uniform_buffer, 0, bytemuck::bytes_of(&time));

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

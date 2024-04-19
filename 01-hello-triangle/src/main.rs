use winit::{event::{Event, WindowEvent}, event_loop::EventLoop, window::Window};


async fn run(event_loop: EventLoop<()>, window: Window)  {
    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);

    let instance = wgpu::Instance::default();


    let surface = instance.create_surface(&window).unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter that can render to our surface 
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter"); // Will fail if device is not suppported
    
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can
                // support images the size of the swapchain
                required_limits: wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits())
            }, 
            None,
        )
        .await
        .expect("Failed to create device");

    // Load shaders. include_str! loads the contents of the pass file, while Cow::Borrowed does
    // a clone-on-write procedure, essentially allowing lazy-loading for the passed file into
    // memory. This line of code effectively loads the shader into memory without compilation.
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    // Creates a pipeline layout abstracted from the architecture of the underlying device.
    let pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
    
    // The swapchain is a queue specifying which image is going to be drawn to the screen. The
    // surface is agnostic to the adapter so combining the two gets the capabilites of the
    // swapchain. Some devices may be more limited in specific areas than others.
    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    // Establish the pipeline for the main shader by specifying the layout, vertex shader (with the
    // files entry point), fragment shader (with the file's entry point), the targeted window, and
    // various other properties that need not be covered here.
    let render_pipeline = device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor { 
            label: None, 
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState { 
                module: &shader, 
                entry_point: "vs_main",  // Entry point within the specified file
                buffers: &[],
            }, 
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(swapchain_format.into())],
            }), 
            primitive: wgpu::PrimitiveState::default(), 
            depth_stencil: None, 
            multisample: wgpu::MultisampleState::default(), 
            multiview: None 
        });

    // Get the configuration for the targeted surface. This will be updated as the window is
    // updated.
    let mut config = surface
        .get_default_config(&adapter, size.width, size.height)
        .unwrap();
    surface.configure(&device, &config);

    let window = &window;

    // Finally set up the event loop
    event_loop
        .run(move |event, target| {
            // Have the closure take ownership of the events and the target
            // `event_loop.run` never returns, so we must do this to ensure the resources are
            // properly cleaned up.
            let _ = (&instance, &adapter, &shader, &pipeline_layout);
            
            // If the event is a window event, get the window id and the underlying event for
            // polling
            if let Event::WindowEvent { window_id: _, event, } = event {
                match event {
                    WindowEvent::Resized(new_size) => {
                        // Resize the surface
                        config.width = new_size.width.max(1);
                        config.height = new_size.height.max(1);
                        surface.configure(&device, &config);
                        window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {

                        // Get whats on the screen as a texture. Contains information such as
                        // format, tiling arrangement, mip level count, array slice count,
                        // dimensionality
                        let frame = surface
                            .get_current_texture()
                            .expect("Failed to acquire next swap chain texture");

                        // Create a view from the texture, eg a view mapping, that acts as an
                        // adapter to the contents of the image
                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        // Initialize an encoder for the device which converts our semantics into
                        // something meaningful
                        let mut encoder = 
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: None,
                            });

                        // Enclose in block so memory is immediately freed
                        {
                            // Start the render pass
                            let mut rpass = 
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: None,
                                    // Color attachment is another image view but with specfic
                                    // instructions on loading, storing, and resolution. In this
                                    // case, the image view can either use the LoadOp::Clear to
                                    // clear the screen, or LoadOp::Load to use the existing frame.
                                    // Storing can either save the operation or immediately discard
                                    // it. Not exactly sure when that would be necessary but might
                                    // be worth looking into.
                                    //
                                    // In this specific case, we give it the original view and it
                                    // can either load in the original view or discard its contents
                                    // and then either store the results back in the original view
                                    // or (again) discard its contents.
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view: &view,
                                        resolve_target: None,
                                        // Clear the screen
                                        ops: wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                            store: wgpu::StoreOp::Store,
                                        },
                                    })],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                });
                            
                            // Set the pipeline to the one created earlier
                            rpass.set_pipeline(&render_pipeline);

                            // Draw the first 3 vertices and the first instance
                            rpass.draw(0..3, 0..1);
                        }

                        // Add the frame to the end of the swapchain
                        queue.submit(Some(encoder.finish()));

                        // Schedule the frame for presentation on the owning surface
                        frame.present();

                    }

                WindowEvent::CloseRequested => target.exit(),
                _ => {}

                };

            }

        }).unwrap();
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    #[allow(unused_mut)]
    let mut builder = winit::window::WindowBuilder::new();
    let window = builder.build(&event_loop).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run(event_loop, window));
    }
}

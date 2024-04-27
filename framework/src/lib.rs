use wgpu::util::DeviceExt;

pub mod wgpu_context;

pub struct RenderPassBuilder<'tex> {
    color_attachments: Vec<Option<wgpu::RenderPassColorAttachment<'tex>>>
}

impl<'tex> RenderPassBuilder<'tex> {
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

    pub fn build(self, encoder: &'tex mut wgpu::CommandEncoder) -> wgpu::RenderPass {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &self.color_attachments,
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None
        })
    }
}

pub struct BufferBuilder;
impl BufferBuilder {
    pub fn size_of<'a, Type>() -> SizedBufferBuilder<'a> {
        SizedBufferBuilder { 
            label: None, 
            size: std::mem::size_of::<Type>() as wgpu::BufferAddress, 
            usage: None, 
            mapped_at_creation: None 
        }

    }

    pub fn bytes_of<'a, Type: bytemuck::Pod>(contents: &'a Type) -> ContentsBufferBuilder<'a> {
        ContentsBufferBuilder {
            contents: bytemuck::bytes_of(contents),
            label: None,
            usage: None
        }
    }

    pub fn slice_of<'a, Type: bytemuck::Pod>(contents: &'a [Type]) -> ContentsBufferBuilder<'a> {
        ContentsBufferBuilder {
            contents: bytemuck::cast_slice(contents),
            label: None,
            usage: None
        }
    }
}

pub struct SizedBufferBuilder<'a> {
    size: wgpu::BufferAddress,
    label: Option<&'a str>,
    usage: Option<wgpu::BufferUsages>,
    mapped_at_creation: Option<bool>,
}

impl<'a> SizedBufferBuilder<'a> {
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn usage(mut self, usage: wgpu::BufferUsages) -> Self {
        self.usage = Some(usage);
        self
    }

    pub fn mapped_at_creation(mut self, mapped_at_creation: bool) -> Self {
        self.mapped_at_creation = Some(mapped_at_creation);
        self
    }

    pub fn build(self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer(
            &wgpu::BufferDescriptor {
                label: self.label,
                size: self.size,
                usage: self.usage.unwrap_or(wgpu::BufferUsages::empty()),
                mapped_at_creation: self.mapped_at_creation.unwrap_or(false),
            }
            )
    }
}

pub struct ContentsBufferBuilder<'a> {
    contents: &'a [u8],
    label: Option<&'a str>,
    usage: Option<wgpu::BufferUsages>,
}


impl<'a> ContentsBufferBuilder<'a> {
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn usage(mut self, usage: wgpu::BufferUsages) -> Self {
        self.usage = Some(usage);
        self
    }

    pub fn build(self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: self.label,
                contents: self.contents,
                usage: self.usage.unwrap_or(wgpu::BufferUsages::empty()),
            }
            )
    }
}



pub struct PipelineLayoutBuilder<'dev> {
    label: Option<&'dev str>,
    bind_group_layouts: Vec<&'dev wgpu::BindGroupLayout>,
    push_constant_ranges: Vec<wgpu::PushConstantRange>
}

impl<'dev> PipelineLayoutBuilder<'dev> {
    pub fn new() -> Self {
        Self {
            label: None,
            bind_group_layouts: Vec::new(),
            push_constant_ranges: Vec::new(),
        }
    }

    pub fn label(mut self, label: &'dev str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn add_bind_group_layout(mut self, layout: &'dev wgpu::BindGroupLayout) -> Self {
        self.bind_group_layouts.push(layout);
        self
    }

    pub fn add_push_constant_range(mut self, range: wgpu::PushConstantRange) -> Self {
        self.push_constant_ranges.push(range);
        self
    }

    pub fn build(self, device: &wgpu::Device) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor { 
                label: self.label, 
                bind_group_layouts: &self.bind_group_layouts, 
                push_constant_ranges: &self.push_constant_ranges 
            })

    }
}

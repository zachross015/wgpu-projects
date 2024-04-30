use wgpu::util::DeviceExt;

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

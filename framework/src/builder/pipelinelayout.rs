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

pub struct BindGroup {
    bind_group: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,
}

impl BindGroup {
    pub fn new(
        device: &wgpu::Device,
        bindings: (&[wgpu::BindGroupLayoutEntry], &[wgpu::BindGroupEntry]),
        name: &str,
    ) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: bindings.0,
            label: Some(&format!("{}_bind_group_layout", name.to_ascii_lowercase())),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: bindings.1,
            label: Some(&format!("{}_bind_group", name.to_ascii_lowercase())),
        });

        Self {
            bind_group,
            layout: bind_group_layout,
        }
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }
}

use wgpu::{util::DeviceExt, Buffer};

pub struct Uniform<T>
where
    T: bytemuck::Pod,
{
    uniforms: T,
    buffer: Buffer,
    layout_entry: wgpu::BindGroupLayoutEntry,
}

impl<T> Uniform<T>
where
    T: bytemuck::Pod,
{
    pub fn new(device: &wgpu::Device, uniforms: T, name: &str, binding: u32) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Buffer", name)),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self::create_uniform(device, binding, name, buffer, uniforms)
    }

    pub fn update(&mut self, device: &wgpu::Device) {
        // self.buffer = device.create_buffer_with_data(
        //     bytemuck::cast_slice(&[self.uniforms]),
        //     wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        // );
    }

    pub fn mut_uniforms(&mut self) -> &mut T {
        &mut self.uniforms
    }

    pub fn uniforms(&self) -> &T {
        &self.uniforms
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn layout_entry(&self) -> &wgpu::BindGroupLayoutEntry {
        &self.layout_entry
    }

    fn create_uniform(
        device: &wgpu::Device,
        binding: u32,
        name: &str,
        buffer: Buffer,
        uniforms: T,
    ) -> Uniform<T> {
        let layout_entry = wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };

        Self {
            uniforms,
            buffer,
            layout_entry,
        }
    }

    pub fn entry(&self) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: self.layout_entry.binding,
            resource: self.buffer.as_entire_binding(),
        }
    }
}

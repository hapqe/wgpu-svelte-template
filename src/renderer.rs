use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use winit::window::Window;

use crate::app::AppProps;
use crate::bindgroup::BindGroup;
use crate::{app::App, pipeline::Pipeline, uniform::Uniform};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
struct GlobalData {
    time: f32,
    screen_width: f32,
    screen_height: f32,
    dpi: f32,
    mouse_x: f32,
    mouse_y: f32,
    aspect_ratio: f32,
    mouse_down: f32,
}

pub struct SetupData {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    rect_pipeline: Pipeline,

    bindgroup: BindGroup,

    global_uniforms: Uniform<GlobalData>,
}

pub struct RenderSystem {
    data: SetupData,
    app_props: Rc<RefCell<AppProps>>,
}

impl RenderSystem {
    async fn load_shader_str(path: &str) -> String {
        let path = String::from("shaders/".to_owned() + path);
        App::get_server_file_str(path.as_str()).await
    }

    pub async fn new(app: Rc<RefCell<AppProps>>) -> Rc<RefCell<RenderSystem>> {
        let app_props = app.clone();
        let mut app_props = app_props.borrow_mut();

        let size = app_props.window().inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL);
        let surface = unsafe { instance.create_surface(app_props.window()) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find a compatible adapter");

        // (device, queue)
        let divice_request = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),

                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: wgpu::Limits::downlevel_webgl2_defaults(),
                label: None,
            },
            None,
        );

        let (device_queue, rect_str) =
            futures::join!(divice_request, Self::load_shader_str("square.wgsl"),);

        let (device, queue) = device_queue.expect("Failed to create device");

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let mut global_data = GlobalData::default();
        global_data.screen_width = size.width as f32;
        global_data.screen_height = size.height as f32;

        let global_uniforms = Uniform::new(&device, global_data, "Global", 0);

        let bindgroup = BindGroup::new(
            &device,
            (
                &[*global_uniforms.layout_entry()],
                &[global_uniforms.entry()],
            ),
            "Render",
        );

        let rect_pipeline = Pipeline::new(
            rect_str,
            &device,
            &config,
            &[bindgroup.layout()],
            &[],
            false,
        );

        let renderer = RenderSystem {
            data: SetupData {
                surface,
                device,
                queue,
                config,
                size,

                rect_pipeline,

                bindgroup,

                global_uniforms,
            },
            app_props: app,
        };

        let renderer = Rc::new(RefCell::new(renderer));

        let update = Rc::downgrade(&renderer);
        app_props.update.add(move |()| {
            if let Some(renderer) = update.upgrade() {
                renderer.borrow_mut().render();
            }
        });
        let resize = Rc::downgrade(&renderer);
        app_props.resize.add(move |size| {
            if let Some(renderer) = resize.upgrade() {
                renderer.borrow_mut().resize(size);
            }
        });
        // app.update.add(|_| {
        //     let mut renderer = update_renderer.borrow_mut();
        //     renderer.render();
        // });

        renderer
    }

    pub fn resize(&mut self, new_size: cgmath::Vector2<u32>) {
        let data = &mut self.data;

        data.size = winit::dpi::PhysicalSize::new(new_size.x, new_size.y);
        data.config.width = new_size.x;
        data.config.height = new_size.y;
        data.surface.configure(&data.device, &data.config);

        let global_uniforms = data.global_uniforms.mut_uniforms();
        global_uniforms.screen_width = new_size.x as f32;
        global_uniforms.screen_height = new_size.y as f32;

        data.queue.write_buffer(
            data.global_uniforms.buffer(),
            0,
            bytemuck::cast_slice(&[*data.global_uniforms.uniforms()]),
        );
    }

    pub fn resize_no_change(&mut self) {
        self.data.config.width = self.data.size.width;
        self.data.config.height = self.data.size.height;
        self.data
            .surface
            .configure(&self.data.device, &self.data.config);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let data = &mut self.data;

        let output = data.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = data
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[
                // This is what [[location(0)]] in the fragment shader targets
                wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                },
            ],
            depth_stencil_attachment: None,
        });

        // let global_uniforms = data.global_uniforms.mut_uniforms();
        // data.queue
        //     .write_buffer(data.global_uniforms.buffer(), 0, global_uniforms.unwrap());

        render_pass.set_bind_group(0, data.bindgroup.bind_group(), &[]);

        // render_pass.set_pipeline(&data.shadow_pipeline.get_pipeline());
        // render_pass.draw(0..4, 0..scene.nodes.len() as _);

        render_pass.set_pipeline(&data.rect_pipeline.get_pipeline());
        render_pass.draw(0..4, 0..1);

        drop(render_pass);

        self.data.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

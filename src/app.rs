use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use wasm_bindgen::JsCast;

use winit::event::ElementState;
use winit::event::Event;
use winit::event::KeyboardInput;
use winit::event::VirtualKeyCode;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::platform::web::WindowBuilderExtWebSys;

use cgmath::Vector2;
use winit::window::WindowBuilder;

use crate::action::Action;

#[wasm_bindgen(raw_module = "./../site/src/lib/Canvas.svelte")]
extern "C" {
    #[wasm_bindgen(js_name = "getServerFile")]
    async fn get_server_file(path: String) -> JsValue;
}

pub struct AppProps {
    window: winit::window::Window,
    pub update: Action<()>,
    pub resize: Action<Vector2<u32>>,
}

impl AppProps {
    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }
}

pub struct App {
    event_loop: EventLoop<()>,
    pub props: Rc<RefCell<AppProps>>,
}

impl App {
    pub fn new() -> Self {
        let str = Self::get_server_file_str("shaders/square.wgsl");
        let dom_window = web_sys::window().expect("There is no window in the DOM");
        let canvas = (|| {
            let canvas = dom_window.document()?.get_element_by_id("canvas")?;
            match canvas.dyn_into::<web_sys::HtmlCanvasElement>() {
                Ok(canvas) => Some(canvas),
                Err(_) => None,
            }
        })()
        .expect("Failed to get canvas element with id 'canvas'");

        let window_size = Vector2::new(canvas.client_width() as u32, canvas.client_height() as u32);

        let event_loop = EventLoop::new();
        let physical_size =
            winit::dpi::PhysicalSize::new(window_size.x as u32, window_size.y as u32);
        let window = WindowBuilder::new()
            .with_canvas(Some(canvas))
            .with_inner_size(physical_size)
            .build(&event_loop)
            .expect("Failed to create window");

        let props = AppProps {
            window,
            update: Action::new(),
            resize: Action::new(),
        };
        let props = Rc::new(RefCell::new(props));
        App { event_loop, props }
    }

    pub fn run(self) {
        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.props.clone().borrow_mut().window.id() => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        log::info!("Pressed esc")
                    }
                    _ => {}
                },
                Event::RedrawRequested(window_id)
                    if window_id == self.props.clone().borrow_mut().window.id() =>
                {
                    // Potential update call here;
                    self.props.clone().borrow_mut().update.invoke(|| ());
                }
                Event::MainEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    self.props.clone().borrow_mut().window.request_redraw();
                }
                _ => {}
            });
    }

    pub async fn get_server_file_str(path: &str) -> String {
        get_server_file(path.to_string())
            .await
            .as_string()
            .expect(format!("Failed to get server file {}", path).as_str())
    }
}

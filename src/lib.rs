mod action;
mod app;
mod bindgroup;
mod pipeline;
mod renderer;
mod texture;
mod uniform;

use app::App;
use renderer::RenderSystem;

use wasm_bindgen::prelude::*;

use crate::texture::get_image_bytes;

#[wasm_bindgen(start)]
pub async fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    let image_bytes = get_image_bytes("images/wasm".to_string()).await;
    log::info!("{:?}", image_bytes);
    let app = App::new();
    let renderer = renderer::RenderSystem::new(app.props.clone()).await;
    app.run();
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "./../site/src/lib/Canvas.svelte")]
extern "C" {
    #[wasm_bindgen(js_name = "getImageBytes")]
    pub async fn get_image_bytes(path: String) -> JsValue;
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

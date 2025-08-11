use leptos::web_sys::{ImageData, ImageBitmap};
use leptos::wasm_bindgen::{JsValue, JsCast};

pub async fn fetch_image_data(url: &str) -> anyhow::Result<ImageData, JsValue> {
    let image_bitmap = load_image_bitmap(url).await?;

    let width = image_bitmap.width();
    let height = image_bitmap.height();

    // use offscreen canvas to handle multiple image loadings
    let canvas = leptos::web_sys::OffscreenCanvas::new(width, height)?;

    let ctx = canvas.get_context("2d")?.unwrap().dyn_into::<leptos::web_sys::OffscreenCanvasRenderingContext2d>().unwrap();
    ctx.translate(0.0, height as f64)?;
    ctx.scale(1.0, -1.0)?;
    ctx.draw_image_with_image_bitmap(&image_bitmap, 0.0, 0.0)?;

    let image_data = ctx.get_image_data(0.0, 0.0, width as f64, height as f64)?;

    Ok(image_data)
}

async fn load_image_bitmap(url: &str) -> anyhow::Result<ImageBitmap, JsValue> {
    let opts = leptos::web_sys::RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(leptos::web_sys::RequestMode::Cors);

    let url = format!("http://127.0.0.1:3000/{}", url);

    let request = leptos::web_sys::Request::new_with_str_and_init(&url, &opts)?;

    let window = leptos::web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: leptos::web_sys::Response = resp_value.dyn_into().unwrap();

    let blob_promise = resp.blob()?;
    let blob = wasm_bindgen_futures::JsFuture::from(blob_promise).await?;

    let window = leptos::web_sys::window().unwrap();
    let image_bitmap_promise = window.create_image_bitmap_with_blob(&blob.unchecked_into())?;
    let image_bitmap_js_value = wasm_bindgen_futures::JsFuture::from(image_bitmap_promise).await?;

    let image_bitmap = ImageBitmap::from(image_bitmap_js_value);

    Ok(image_bitmap)
}
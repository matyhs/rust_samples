mod app;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}
#![cfg(target_arch = "wasm32")]

use codectrl_gui::{run, WebHandle};
use eframe::wasm_bindgen::{self, prelude::*};

#[wasm_bindgen]
pub async fn start(canvas_id: &str) -> Result<WebHandle, JsValue> {
    let host = option_env!("HOST").unwrap_or("127.0.0.1");
    let port = option_env!("PORT").unwrap_or("3002");

    run(host, port, Some(canvas_id)).await
}

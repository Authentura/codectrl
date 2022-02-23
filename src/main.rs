use codectrl_gui::run;

#[cfg(not(target_arch = "wasm32"))]
fn main() { run(); }

#[cfg(target_arch = "wasm32")]
fn main() -> Result<(), eframe::wasm_bindgen::JsValue> { run() }

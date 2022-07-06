use codectrl_gui::run;

#[cfg(not(target_arch = "wasm32"))]
fn main() { run(); }

#[cfg(target_arch = "wasm32")]
fn main() -> Result<(), eframe::wasm_bindgen::JsValue> {
    let host = if let Some(host) = option_env!("HOST") {
        host.clone()
    } else {
        "127.0.0.1"
    };

    let port = if let Some(port) = option_env!("PORT") {
        port.clone()
    } else {
        "3002"
    };

    let _res = run(&host, &port);

    Ok(())
}

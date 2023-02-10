use codectrl_gui::run;

#[cfg(not(target_arch = "wasm32"))]
use dotenv::dotenv;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    dotenv().ok();
    env_logger::try_init().ok();

    run();
}

#[cfg(target_arch = "wasm32")]
fn main() -> Result<(), eframe::wasm_bindgen::JsValue> {
    let host = if let Some(host) = option_env!("HOST") {
        host
    } else {
        "127.0.0.1"
    };

    let port = if let Some(port) = option_env!("PORT") {
        port
    } else {
        "3002"
    };

    wasm_bindgen_futures::spawn_local(async move {
        _ = run(host, port).await;
    });

    Ok(())
}

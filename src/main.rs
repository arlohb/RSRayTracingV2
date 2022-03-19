#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = rs_ray_tracing_v2::TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

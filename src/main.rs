use std::thread;

fn render_thread() {
  loop {
    rs_ray_tracing_v2::ray_tracer::render_image();
  }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
  thread::spawn(render_thread);

  let app = rs_ray_tracing_v2::App::new(400, 300);
  let native_options = eframe::NativeOptions {
    initial_window_size: Some(eframe::epaint::Vec2 { x: 1000., y: 800. }),
    ..eframe::NativeOptions::default()
  };
  eframe::run_native(Box::new(app), native_options);
}

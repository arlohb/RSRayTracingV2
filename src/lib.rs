mod app;
pub use app::TemplateApp;

pub mod ray_tracer;
pub mod movement;
pub mod linker;

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_rayon::init_thread_pool;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{self, prelude::*};

use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{console, Worker};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
  // Make sure panics are logged using `console.error`.
  console_error_panic_hook::set_once();

  // Redirect tracing to console.log and friends:
  tracing_wasm::set_as_global_default();

  console::log_1(&"Creating worker from wasm".into());
  let worker_handle = Rc::new(RefCell::new(Worker::new("./wasm-worker.js")?));
  console::log_1(&"Created worker from wasm".into());

  let app = TemplateApp::default();
  eframe::start_web(canvas_id, Box::new(app))
}

mod app;
pub use app::App;

pub mod ray_tracer;
pub mod movement;
pub mod panels;

use once_cell::sync::Lazy;
use std::sync::Mutex;

use ray_tracer::Options;

static OPTIONS: Lazy<Mutex<Options>> = Lazy::new(||
  Mutex::new(Options::new(400, 300))
);
static IMAGE: Lazy<Mutex<eframe::epaint::image::ColorImage>> = Lazy::new(||
  Mutex::new(eframe::epaint::image::ColorImage::new([400, 300], eframe::epaint::Color32::BLACK))
);
static FRAME_TIMES: Lazy<Mutex<eframe::egui::util::History<f32>>> = Lazy::new(||
  Mutex::new(eframe::egui::util::History::new(0..usize::MAX, 20.))
);

#[wasm_bindgen]
extern "C" {
  #[no_mangle]
  #[used]
  pub static performance:web_sys::Performance;
}

#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! log {
  ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_rayon::init_thread_pool;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
  // Make sure panics are logged using `console.error`.
  console_error_panic_hook::set_once();

  // Redirect tracing to console.log and friends:
  tracing_wasm::set_as_global_default();

  log!("{}", performance.now());

  let app = App::default();
  eframe::start_web(canvas_id, Box::new(app))
}

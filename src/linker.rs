use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Linker {
    number: f64,
}

#[wasm_bindgen]
impl Linker {
  pub fn new() -> Linker {
    Linker {
      number: 0.,
    }
  }

  pub fn increment(&mut self) {
    self.number += 1.;
  }
}

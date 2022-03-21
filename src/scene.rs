use crate::objects::{
  Light,
  Object,
};

pub struct Scene {
  pub objects: Vec<Object>,
  pub lights: Vec<Light>,
  pub background_colour: (f64, f64, f64),
  pub ambient_light: (f64, f64, f64),
  pub reflection_limit: u32,
  pub do_objects_spin: bool,
}

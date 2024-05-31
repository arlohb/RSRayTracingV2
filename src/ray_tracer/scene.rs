use crate::ray_tracer::{Light, Object};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub background_colour: (f64, f64, f64),
    pub ambient_light: (f64, f64, f64),
    pub reflection_limit: u32,
    pub do_objects_spin: bool,
}

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
use eframe::egui;
use crate::ray_tracer::*;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub struct Linker {
  ray_tracer: RayTracer,
  image: eframe::epaint::ColorImage,
  texture: Option<eframe::epaint::TextureHandle>,
  frame_times: egui::util::History<f32>,
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
impl Linker {
  pub fn new(width: u32, height: u32) -> Linker {
    Self {
      ray_tracer: RayTracer {
        camera: Vec3 { x: 5., y: 5., z: 5. },
        rotation: Vec3 { x: 0.7, y: -std::f64::consts::PI / 4., z: 0. },
        fov: 70.,
        width,
        height,
        scene: Scene {
          objects: vec![
            Object {
                name: "sphere".to_string(),
                material: Material {
                    colour: (
                        1.0,
                        0.5212054252624512,
                        0.0,
                    ),
                    specular: 5.0,
                    metallic: 1.0,
                },
                geometry: Geometry::Sphere {
                    center: Vec3 {
                        x: 1.5,
                        y: 0.0,
                        z: 0.0,
                    },
                    radius: 1.0,
                },
            },
            Object {
                name: "sphere".to_string(),
                material: Material {
                    colour: (
                        1.0,
                        0.3486607074737549,
                        0.0,
                    ),
                    specular: 800.0,
                    metallic: 0.2,
                },
                geometry: Geometry::Sphere {
                    center: Vec3 {
                        x: 3.1,
                        y: 0.0,
                        z: 2.1,
                    },
                    radius: 1.0,
                },
            },
            Object {
                name: "sphere".to_string(),
                material: Material {
                    colour: (
                        0.0,
                        0.6445307731628418,
                        1.0,
                    ),
                    specular: 80.0,
                    metallic: 0.,
                },
                geometry: Geometry::Sphere {
                    center: Vec3 {
                        x: -8.3,
                        y: 0.0,
                        z: 0.0,
                    },
                    radius: 1.0,
                },
            },
            Object {
              name: "plane".to_string(),
              material: Material {
                colour: (0.8, 0.8, 1.),
                specular: 50.,
                metallic: 0.2,
              },
              geometry: Geometry::Plane {
                center: Vec3 { x: 0., y: -1.5, z: 0. },
                normal: Vec3 { x: 0., y: 1., z: 0. },
                size: 5.,
              },
            },
        ],
          lights: vec![
            Light::Direction {
              intensity: (0.4, 0.4, 0.4),
              direction: Vec3 { x: -1., y: -1.5, z: -0.5 }.normalize(),
            },
            Light::Point {
              intensity: (0.4, 0.4, 0.4),
              position: Vec3 { x: 0., y: 2., z: 0., },
            },
          ],
          background_colour: (0.5, 0.8, 1.),
          ambient_light: (0.2, 0.2, 0.2),
          reflection_limit: 4,
          do_objects_spin: false,
        },
      },
      frame_times: egui::util::History::new(0..usize::MAX, 20.),
      image: eframe::epaint::ColorImage::new([width as usize, height as usize], eframe::epaint::Color32::BLACK),
      texture: None,
    }
  }

  pub fn render_frame(&mut self) {
    self.ray_tracer.rs_render(&mut self.image);
  }
}

impl Linker {
  pub fn add_frame_time(&mut self, current_time: f64, frame_time: f32) {
    self.frame_times.add(current_time, frame_time);
  }

  pub fn average_frame_time(&self) -> f32 {
    self.frame_times.average().unwrap_or(1.)
  }

  pub fn get_texture(&self) -> &Option<eframe::epaint::TextureHandle> {
    &self.texture
  }

  pub fn get_texture_as_mut(&mut self) -> &mut Option<eframe::epaint::TextureHandle> {
    &mut self.texture
  }

  pub fn create_texture(&mut self, texture: eframe::epaint::TextureHandle) {
    self.texture = Some(texture);
  }

  pub fn set_texture(&mut self, texture: eframe::epaint::ImageData) {
    self.texture.as_mut().unwrap().set(texture);
  }

  pub fn get_image(&self) -> &eframe::epaint::ColorImage {
    &self.image
  }

  pub fn get_image_as_mut(&mut self) -> &mut eframe::epaint::ColorImage {
    &mut self.image
  }

  pub fn get_ray_tracer(&self) -> &RayTracer {
    &self.ray_tracer
  }

  pub fn get_ray_tracer_as_mut(&mut self) -> &mut RayTracer {
    &mut self.ray_tracer
  }
}

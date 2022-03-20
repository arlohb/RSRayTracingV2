#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use crate::{
  vec3::Vec3,
  objects::{
    Material,
    Object,
  },
  camera::Camera,
  ray::Ray,
  scene::Scene,
};

pub struct Hit<'a> {
  pub distance: f64,
  pub point: Vec3,
  pub object: &'a Object,
}

pub struct RayTracer {
  pub from: Vec3,
  pub to: Vec3,
  pub fov: f64,
  pub width: u32,
  pub height: u32,
  pub scene: Scene,
}

impl RayTracer {
  fn calculate_light(
    &self,
    point: Vec3,
    normal: Vec3,
    camera_pos: Vec3,
    material: &Material,
  ) -> (f64, f64, f64) {
    let mut result = (
      self.scene.ambient_light.0,
      self.scene.ambient_light.1,
      self.scene.ambient_light.2,
    );

    for light in self.scene.lights.iter() {
      let intensity = light.intensity(point);
      let vec_to_light = light.vec_to_light(point);

      let strength = (normal.dot(vec_to_light)
        / (normal.length() * vec_to_light.length())).clamp(0., 1.);
      
      let reflection_vector = (normal * normal.dot(vec_to_light)) * 2. - vec_to_light;
      let camera_vector = camera_pos - point;

      let specular = (reflection_vector.dot(camera_vector)
        / (reflection_vector.length() * camera_vector.length())).clamp(0., 1.).powf(material.specular);

      result.0 += intensity.0 * (strength + specular);
      result.1 += intensity.1 * (strength + specular);
      result.2 += intensity.2 * (strength + specular);
    }

    result
  }

  fn trace_ray(
    &self,
    ray: &Ray,
  ) -> Option<(&Object, Vec3)> {
    let mut hit: Option<Hit> = None;

    for object in &self.scene.objects {
      match object.geometry.intersect(ray) {
        Some((distance, hit_point)) => {
          match &hit {
            Some(h) => {
              if distance < h.distance {
                hit = Some(Hit {
                  distance,
                  point: hit_point,
                  object,
                });
              }
            },
            None => {
              hit = Some(Hit {
                distance,
                point: hit_point,
                object,
              });
            }
          }
        },
        None => continue
      };      
    }

    match &hit {
      Some(h) => Some((h.object, h.point)),
      None => None,
    }
  }

  fn render_pixel(
    &self,
    x: u32,
    y: u32,
    top_left: Vec3,
    width_world_space: f64,
    height_world_space: f64,
    right: Vec3,
    up: Vec3,
    camera: &Camera,
  ) -> (f64, f64, f64) {
    let x_screen_space = (x as f64 + 0.5) / self.width as f64;
    let y_screen_space = (y as f64 + 0.5) / self.height as f64;

    let x_offset = right * (x_screen_space * width_world_space);
    // mul -1 because it's offset down
    let y_offset = (-up) * (y_screen_space * height_world_space);

    let pixel_world_space = top_left + x_offset + y_offset;

    let direction = (pixel_world_space - camera.from).normalize();

    let ray = Ray {
      origin: camera.from,
      direction
    };

    match self.trace_ray(&ray) {
      Some((object, hit_point)) => {
        let normal = object.geometry.normal_at_point(hit_point);

        let brightness = self.calculate_light(hit_point, normal, camera.from, &object.material);
        (
            brightness.0 * object.material.colour.0,
            brightness.1 * object.material.colour.1,
            brightness.2 * object.material.colour.2,
        )
      },
      None => self.scene.background_colour,
    }
  }

  pub fn rs_render(&self, image: &mut eframe::epaint::ColorImage) {
    let camera = Camera {
      from: self.from,
      to: self.to,
      fov: self.fov,
      width: self.width,
      height: self.height,
    };

    if image.width() != self.width as usize || image.height() != self.height as usize {
      *image = eframe::epaint::ColorImage::new([self.width as usize, self.height as usize], eframe::epaint::Color32::BLACK);
    }

    let image_plane = camera.get_image_plane();

    // working for this in whiteboard
    let top_left_point = image_plane.left + image_plane.top - image_plane.center;

    let width_world_space = (image_plane.right - image_plane.left).length();
    let height_world_space = (image_plane.top - image_plane.bottom).length();
    let (right, up, _) = camera.get_vectors();

    #[cfg(not(target_arch = "wasm32"))]
    image.pixels.par_iter_mut().enumerate().for_each(|(index, colour)| {
      let y = (index as u32) / (self.width as u32);
      let x = index as u32 % self.width;

      let pixel = self.render_pixel(x, y, top_left_point, width_world_space, height_world_space, right, up, &camera);

      *colour = eframe::epaint::Color32::from_rgb(
        (pixel.0 * 255.) as u8,
        (pixel.1 * 255.) as u8,
        (pixel.2 * 255.) as u8,
      );
    });

    #[cfg(target_arch = "wasm32")]
    image.pixels.iter_mut().enumerate().for_each(|(index, colour)| {
      let y = (index as u32) / (self.width as u32);
      let x = index as u32 % self.width;

      let pixel = self.render_pixel(x, y, top_left_point, width_world_space, height_world_space, right, up, &camera);

      *colour = eframe::epaint::Color32::from_rgb(
        (pixel.0 * 255.) as u8,
        (pixel.1 * 255.) as u8,
        (pixel.2 * 255.) as u8,
      );
    });
  }
}

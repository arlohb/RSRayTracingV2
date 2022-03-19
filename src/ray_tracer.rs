use crate::{
  vec3::Vec3,
  objects::{
    Light,
    Material,
    Sphere,
  },
  camera::Camera,
  ray::Ray,
};

const BACKGROUND_COLOUR: (f64, f64, f64) = (0.5, 0.8, 1.);
const AMBIENT_LIGHT: (f64, f64, f64) = (0.2, 0.2, 0.2);

pub struct RayTracer {
  pub from: Vec3,
  pub to: Vec3,
  pub fov: f64,
  pub width: u32,
  pub height: u32,
  pub scene: (Vec<Sphere>, Vec<Light>),
}

impl RayTracer {
  fn calculate_light(
    &self,
    point: Vec3,
    normal: Vec3,
    camera_pos: Vec3,
    material: Material,
  ) -> (f64, f64, f64) {
    let mut result = (
      AMBIENT_LIGHT.0,
      AMBIENT_LIGHT.1,
      AMBIENT_LIGHT.2,
    );

    for light in self.scene.1.iter() {
      let intensity = light.intensity(point);
      let light_direction = light.direction(point);

      let strength = (normal.dot(light_direction)
        / (normal.length() * light_direction.length())).clamp(0., 1.);
      
      let reflection_vector = (normal * normal.dot(light_direction)) * 2. - light_direction;
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
  ) -> Option<(Sphere, Vec3)> {
    let mut min_hit_distance = 1e9;
    let mut min_hit_object: Option<&Sphere> = None;

    for object in &self.scene.0 {
      let distance = match object.intersect(&ray) {
        Some(d) => d,
        None => continue
      };

      if distance < min_hit_distance {
        min_hit_distance = distance;
        min_hit_object = Some(object);
      }
    }

    match min_hit_object {
      Some(object) => {
        let hit_point = ray.origin + (ray.direction * min_hit_distance);

        Some((*object, hit_point))
      }
      None => None
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
        let normal = object.normal_at_point(hit_point);

        let brightness = self.calculate_light(hit_point, normal, camera.from, object.material);
        (
            brightness.0 * object.material.colour.0,
            brightness.1 * object.material.colour.1,
            brightness.2 * object.material.colour.2,
        )
      },
      None => BACKGROUND_COLOUR
    }
  }

  pub fn rs_render(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let camera = Camera {
      from: self.from,
      to: self.to,
      fov: self.fov,
      width: self.width,
      height: self.height,
    };

    let image_plane = camera.get_image_plane();

    // working for this in whiteboard
    let top_left_point = image_plane.left + image_plane.top - image_plane.center;

    let width_world_space = (image_plane.right - image_plane.left).length();
    let height_world_space = (image_plane.top - image_plane.bottom).length();
    let (right, up, _) = camera.get_vectors();
    
    let mut image = vec![0u8; (self.width * self.height * 4u32) as usize];

    for x in 0..self.width {
      for y in 0..self.height {
        let pixel = self.render_pixel(x, y, top_left_point, width_world_space, height_world_space, right, up, &camera);

        let index = 4 * (x + (y * self.width)) as usize;
        image[index] = (pixel.0 * 255.) as u8;
        image[index + 1] = (pixel.1 * 255.) as u8;
        image[index + 2] = (pixel.2 * 255.) as u8;
        image[index + 3] = 255;
      }
    }

    Ok(image)
  }
}

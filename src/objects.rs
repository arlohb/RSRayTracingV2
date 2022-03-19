use crate::vec3::Vec3;
use crate::ray::Ray;
pub use crate::solver::solve_quadratic;

#[derive(Clone, Copy)]
pub struct Material {
  pub colour: (f64, f64, f64),
  pub specular: f64,
}

#[derive(Clone, Copy)]
pub struct Sphere {
  pub center: Vec3,
  pub radius: f64,
  pub material: Material,
}

impl Sphere {
  pub fn intersect (&self, ray: &Ray) -> Option<f64> {
    // working out in whiteboard
    let new_origin = ray.origin - self.center;

    let a = 1.;
    let b = 2. * ray.direction.dot(new_origin);
    let c = new_origin.dot(new_origin) - self.radius.powi(2);

    let solution = solve_quadratic(a, b, c);

    match solution {
        Some(solution) => {
          if solution.0 < solution.1 {
            Some(solution.0)
          } else {
            Some(solution.1)
          }
        }
        None => None
    }
  }

  pub fn normal_at_point (&self, point: Vec3) -> Vec3 {
    // simple circle stuff
    (point - self.center).normalize()
  }
}

pub enum Light {
  Direction { intensity: (f64, f64, f64), direction: Vec3},
  Point { intensity: (f64, f64, f64), position: Vec3},
}

impl Light {
  pub fn intensity(&self, _point: Vec3) -> (f64, f64, f64) {
    match self {
      Light::Direction { intensity, direction: _ } => *intensity,
      Light::Point { intensity, position: _ } => *intensity,
    }
  }

  pub fn direction(&self, point: Vec3) -> Vec3 {
    match self {
      Light::Direction { intensity: _, direction } => *direction,
      Light::Point { intensity: _, position } => *position - point,
    }
  }
}

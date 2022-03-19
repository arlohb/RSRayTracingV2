pub use crate::vec3::Vec3;

#[derive (Debug)]
pub struct ImagePlane {
  pub left: Vec3,
  pub right: Vec3,
  pub bottom: Vec3,
  pub top: Vec3,
  pub center: Vec3,
}

pub struct Camera {
  pub from: Vec3,
  pub to: Vec3,
  pub fov: f64,
  pub width: u32,
  pub height: u32,
}

impl Camera {
    pub fn get_vectors(&self) -> (Vec3, Vec3, Vec3) {
      let forward = (self.from - self.to).normalize();

      let temp = Vec3 { x: 0., y: 1., z: 0. };
      let right = (temp.normalize() * forward).normalize();

      let up = (forward * right).normalize();

      (right, up, forward)
    }

    pub fn get_image_plane(&self) -> ImagePlane {
      // working for this is in whiteboard
      let fov_rad = self.fov * (std::f64::consts::PI / 180.);
      let width = 2. * f64::tan(fov_rad / 2.);
      let half_width = width / 2.;

      let height = (width / (self.width as f64)) * (self.height as f64);
      let half_height = height / 2.;

      let (right, up, forward) = self.get_vectors();

      // the image plane is 1 unit away from the camera
      // this is - not + because the camera point in the -forward direction
      let center = self.from - forward;

      ImagePlane {
        left: center - (right * half_width),
        right: center + (right * half_width),
        bottom: center - (up * half_height),
        top: center + (up * half_height),
        center,
      }
    }
}
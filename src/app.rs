use eframe::{egui, epi};
use crate::ray_tracer::RayTracer;
use crate::{
  vec3::Vec3,
  objects::{
    Light,
    Sphere,
    Material,
  }
};

pub struct TemplateApp {
  ray_tracer: RayTracer,
  image: eframe::epaint::ColorImage,
  texture: Option<eframe::epaint::TextureHandle>,
  frame_times: egui::util::History<f32>,
}

impl Default for TemplateApp {
  fn default() -> Self {
    let width = 400;
    let height = 300;
    Self {
      ray_tracer: RayTracer {
        from: Vec3 { x: 5., y: 5., z: 5. },
        to: Vec3 { x: 0., y: 0., z: 0. },
        fov: 70.,
        width,
        height,
        scene: (
          vec![
            Sphere {
              center: Vec3 { x: 0., y: 0., z: 0., },
              radius: 1.,
              material: Material {
                colour: (1., 0., 0.),
                specular: 500.,
              },
            },
          ],
          vec![
            Light::Direction {
              intensity: (0.8, 0.8, 0.8),
              direction: Vec3 { x: -1., y: 1.5, z: -0.5 }.normalize(),
            },
          ],
        )
      },
      frame_times: egui::util::History::new(0..usize::MAX, 20.),
      image: eframe::epaint::ColorImage::new([width as usize, height as usize], eframe::epaint::Color32::BLACK),
      texture: None,
    }
  }
}

impl epi::App for TemplateApp {
  fn name(&self) -> &str {
    "RSRayTracingV2"
  }

  /// Called once before the first frame.
  fn setup(
    &mut self,
    ctx: &egui::Context,
    _frame: &epi::Frame,
    _storage: Option<&dyn epi::Storage>,
  ) {
    ctx.set_style({
      let mut style: egui::Style = (*ctx.style()).clone();
      style.visuals = egui::Visuals::dark();
      style
    });
    self.texture = Some(ctx.load_texture("canvas", self.image.clone()));
  }

  /// Called each time the UI needs repainting, which may be many times per second.
  /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
  fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
    let Self {
      ray_tracer,
      frame_times,
      image,
      texture,
    } = self;

    let previous_frame_time = frame.info().cpu_usage.unwrap_or(0.);
    frame_times.add(ctx.input().time, previous_frame_time);

    ray_tracer.rs_render(image);

    egui::SidePanel::right("side_panel").show(ctx, |ui| {
      ui.heading("Inspector");

      ui.label(format!("fps: {}", 1. / frame_times.average().unwrap_or(1.)));

      if ui.add(egui::Button::new("Add sphere")).clicked() {
        ray_tracer.scene.0.push(Sphere {
          center: Vec3 { x: 0., y: 0., z: 0., },
          radius: 1.,
          material: Material {
            colour: (1., 1., 1.),
            specular: 500.,
          },
        });
      }

      ui.separator();

      for object in &mut ray_tracer.scene.0 {
        ui.horizontal(|ui| {
          ui.add(egui::DragValue::new(&mut object.center.x)
            .fixed_decimals(1usize)
            .speed(0.1));
          ui.add(egui::DragValue::new(&mut object.center.y)
            .fixed_decimals(1usize)
            .speed(0.1));
          ui.add(egui::DragValue::new(&mut object.center.z)
            .fixed_decimals(1usize)
            .speed(0.1));
        });

        ui.separator();
      }
    });

    egui::CentralPanel::default().show(ctx, |ui| {
      match texture {
        Some(texture) => {
          texture.set(eframe::epaint::ImageData::Color(image.clone()));
          ui.add(egui::Image::new(&*texture, texture.size_vec2()));
        },
        None => (),
      }
    });

    ctx.request_repaint();
  }
}

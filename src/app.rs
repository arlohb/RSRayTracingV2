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
}

impl Default for TemplateApp {
  fn default() -> Self {
    Self {
      ray_tracer: RayTracer {
        from: Vec3 { x: 5., y: 5., z: 5. },
        to: Vec3 { x: 0., y: 0., z: 0. },
        fov: 70.,
        width: 400,
        height: 300,
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
              intensity: (1., 1., 1.),
              direction: Vec3 { x: 1., y: 1., z: 1. }.normalize(),
            },
          ],
        )
      }
    }
  }
}

impl epi::App for TemplateApp {
  fn name(&self) -> &str {
    "eframe template"
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
  }

  /// Called each time the UI needs repainting, which may be many times per second.
  /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
  fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
    let Self { ray_tracer } = self;
    let image = ray_tracer.rs_render().expect("Rendering failed");

    egui::SidePanel::right("side_panel").show(ctx, |ui| {
      ui.heading("Inspector");

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
      let colour_image = eframe::epaint::ColorImage::from_rgba_unmultiplied([ray_tracer.width as usize, ray_tracer.height as usize], &image as &[u8]);
      let image = egui::ImageData::Color(colour_image);
      let texture = ctx.load_texture("name", image);
      ui.add(egui::Image::new(&texture, texture.size_vec2()));
    });
  }
}

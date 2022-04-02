use eframe::{egui, epi};
use rayon::prelude::*;
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
use crate::{ray_tracer::*, linker::Linker, panels::*};

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn thread_test() -> u64 {
  let range: Vec<u64> = (0..=1000000).collect();

  let x: u64 = range.par_iter().sum();

  x
}

pub struct TemplateApp {
  // ray_tracer: RayTracer,
  // image: eframe::epaint::ColorImage,
  // texture: Option<eframe::epaint::TextureHandle>,
  // frame_times: egui::util::History<f32>,
  linker: Linker
}

impl Default for TemplateApp {
  fn default() -> Self {
    Self {
      linker: Linker::new(400, 300),
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
    let image = self.linker.get_image().clone();
    self.linker.create_texture(ctx.load_texture("canvas", image));
  }

  /// Called each time the UI needs repainting, which may be many times per second.
  /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
  fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {    
    let screen_rect = ctx.input().screen_rect;
    let is_portrait = screen_rect.height() > screen_rect.width();
    
    let previous_frame_time = frame.info().cpu_usage.unwrap_or(0.);
    self.linker.add_frame_time(ctx.input().time, previous_frame_time);
    let delta_time = previous_frame_time.max(1. / 60.) as f64;
    
    let mut has_size_changed = false;

    {
      let ray_tracer = self.linker.get_ray_tracer_as_mut();
      
      let forward = ray_tracer.forward();
      let right = ray_tracer.right();
      let up = ray_tracer.up();

      crate::movement::move_and_rotate(
        &ctx.input(),
        &mut ray_tracer.camera,
        &mut ray_tracer.rotation,
        forward,
        right,
        up,
        delta_time * 1.5,
        delta_time * 4.,
      );

      if ray_tracer.scene.do_objects_spin {
        ray_tracer.scene.objects.iter_mut().for_each(|object| {
          let position = object.geometry.position_as_mut();
          let length = position.length();

          let theta: f64 = 0.5 * std::f64::consts::PI * delta_time;

          *position = position.transform_point(Mat44::create_rotation(Axis::Y, theta));

          // fix rounding errors?
          *position = *position * (length / position.length());
        });
      }
    }

    if is_portrait {
      egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        egui::SidePanel::left("object_panel")
          .show_inside(ui, |ui| object_panel(ui, &mut self.linker.get_ray_tracer_as_mut().scene));
        egui::SidePanel::right("settings_panel")
          .show_inside(ui, |ui| settings_panel(ui, &mut self.linker, &mut has_size_changed));
      });
    } else {
      egui::SidePanel::right("settings_panel")
        .show(ctx, |ui| settings_panel(ui, &mut self.linker, &mut has_size_changed));
      egui::SidePanel::right("object_panel")
        .show(ctx, |ui| object_panel(ui, &mut self.linker.get_ray_tracer_as_mut().scene));
    }

    egui::CentralPanel::default().show(ctx, |ui| {
      ui.set_max_width(f32::INFINITY);
      ui.set_max_height(f32::INFINITY);
      let texture = self.linker.get_texture();
      match texture {
        Some(_) => {
          egui::Resize::default()
            .default_size((self.linker.get_ray_tracer().width as f32, self.linker.get_ray_tracer().height as f32))
            .show(ui, |ui| {
              if !has_size_changed {
                let ray_tracer = self.linker.get_ray_tracer_as_mut();
                ray_tracer.width = ui.available_width() as u32;
                ray_tracer.height = ui.available_height() as u32;
              }

              // self.linker.render_frame();

              let image: Vec<(u8, u8, u8, u8)> = serde_json::from_str(
                &web_sys::window().expect("No window")
                  .get("rayTracerImage").expect("Property doesn't exist on window")
                  .as_string().unwrap()
              ).expect("Failed to parse image");

              let mut data: Vec<u8> = vec![];

              image.iter().for_each(|(r, g, b, a)| {
                data.push(*r);
                data.push(*g);
                data.push(*b);
                data.push(*a);
              });

              let image = eframe::epaint::ColorImage::from_rgba_unmultiplied([400, 300], &data);

              self.linker.set_texture(eframe::epaint::ImageData::Color(image));

              // (*texture).set(eframe::epaint::ImageData::Color(self.linker.get_image().clone()));
              let texture = self.linker.get_texture().as_ref().unwrap();

              ui.add(egui::Image::new(texture.id(), texture.size_vec2()));
            });
        },
        None => (),
      }
    });

    ctx.request_repaint();
  }
}

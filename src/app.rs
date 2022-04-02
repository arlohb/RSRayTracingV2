use eframe::{egui, epi};
use rayon::prelude::*;
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
use crate::{ray_tracer::*, panels::*};

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn thread_test() -> u64 {
  let range: Vec<u64> = (0..=1000000).collect();

  let x: u64 = range.par_iter().sum();

  x
}

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
    let image = self.image.clone();
    self.texture = Some(ctx.load_texture("canvas", image));
  }

  /// Called each time the UI needs repainting, which may be many times per second.
  /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
  fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {    
    let screen_rect = ctx.input().screen_rect;
    let is_portrait = screen_rect.height() > screen_rect.width();
    
    let previous_frame_time = frame.info().cpu_usage.unwrap_or(0.);
    self.frame_times.add(ctx.input().time, previous_frame_time);
    let delta_time = previous_frame_time.max(1. / 60.) as f64;
    
    let mut has_size_changed = false;

    {
      let ray_tracer = &mut self.ray_tracer;
      
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
          .show_inside(ui, |ui| object_panel(ui, &mut self.ray_tracer.scene));
        egui::SidePanel::right("settings_panel")
          .show_inside(ui, |ui| settings_panel(ui, self.frame_times.average().unwrap_or(1.), &mut self.ray_tracer, &mut has_size_changed));
      });
    } else {
      egui::SidePanel::right("settings_panel")
        .show(ctx, |ui| settings_panel(ui, self.frame_times.average().unwrap_or(1.), &mut self.ray_tracer, &mut has_size_changed));
      egui::SidePanel::right("object_panel")
        .show(ctx, |ui| object_panel(ui, &mut self.ray_tracer.scene));
    }

    egui::CentralPanel::default().show(ctx, |ui| {
      ui.set_max_width(f32::INFINITY);
      ui.set_max_height(f32::INFINITY);
      let texture = &mut self.texture;
      match texture {
        Some(texture) => {
          egui::Resize::default()
            .default_size((self.ray_tracer.width as f32, self.ray_tracer.height as f32))
            .show(ui, |ui| {
              if !has_size_changed {
                let ray_tracer = &mut self.ray_tracer;
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

              texture.set(eframe::epaint::ImageData::Color(image));

              ui.add(egui::Image::new(texture.id(), texture.size_vec2()));
            });
        },
        None => (),
      }
    });

    let options: Options = Options {
      rotation: self.ray_tracer.rotation,
      fov: self.ray_tracer.fov,
      width: self.ray_tracer.width,
      height: self.ray_tracer.height,
      scene: self.ray_tracer.scene.clone(),
      camera: self.ray_tracer.camera,
    };

    js_sys::Reflect::set(
      &JsValue::from(web_sys::window().unwrap()),
      &JsValue::from("rayTracerOptions"),
      &JsValue::from_str(serde_json::to_string(&options).unwrap().as_str()),
    ).unwrap();

    // crate::TEST.lock().unwrap().add();
    // crate::log!("{}", crate::TEST.lock().unwrap().values.len());

    ctx.request_repaint();
  }
}

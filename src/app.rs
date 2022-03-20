use eframe::{egui, epi};
use crate::ray_tracer::RayTracer;
use crate::{
  vec3::Vec3,
  objects::{
    Light,
    Material,
    Geometry,
    Object,
  },
  scene::Scene,
  mat44::{
    Mat44,
    Axis,
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
        },
      },
      frame_times: egui::util::History::new(0..usize::MAX, 20.),
      image: eframe::epaint::ColorImage::new([width as usize, height as usize], eframe::epaint::Color32::BLACK),
      texture: None,
    }
  }
}

fn vec3_widget(ui: &mut egui::Ui, label: impl Into<egui::WidgetText>, vec3: &mut Vec3) {
  ui.horizontal(|ui| {
    ui.label(label);

    ui.add(egui::DragValue::new(&mut vec3.x)
      .fixed_decimals(1)
      .speed(0.1));
    ui.add(egui::DragValue::new(&mut vec3.y)
      .fixed_decimals(1)
      .speed(0.1));
    ui.add(egui::DragValue::new(&mut vec3.z)
      .fixed_decimals(1)
      .speed(0.1));
  });
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

    let screen_rect = ctx.input().screen_rect;
    let is_portrait = screen_rect.height() > screen_rect.width();

    let previous_frame_time = frame.info().cpu_usage.unwrap_or(0.);
    frame_times.add(ctx.input().time, previous_frame_time);
    let delta_time = previous_frame_time.max(1. / 60.) as f64;

    let mut has_size_changed = false;

    let look_speed = delta_time * 1.5;
    let move_speed = delta_time * 4.;

    if ctx.input().key_down(egui::Key::ArrowRight) {
      ray_tracer.rotation.y += look_speed;
    }
    if ctx.input().key_down(egui::Key::ArrowLeft) {
      ray_tracer.rotation.y -= look_speed;
    }
    if ctx.input().key_down(egui::Key::ArrowUp) {
      ray_tracer.rotation.x -= look_speed;
    }
    if ctx.input().key_down(egui::Key::ArrowDown) {
      ray_tracer.rotation.x += look_speed;
    }

    ray_tracer.rotation.x = ray_tracer.rotation.x.clamp(-0.5 * std::f64::consts::PI, 0.5 * std::f64::consts::PI);
    ray_tracer.rotation.y %= 2. * std::f64::consts::PI;
    ray_tracer.rotation.z = ray_tracer.rotation.z.clamp(-std::f64::consts::PI, std::f64::consts::PI);

    if ctx.input().key_down(egui::Key::W) {
      ray_tracer.camera -= ray_tracer.forward() * move_speed;
    }
    if ctx.input().key_down(egui::Key::S) {
      ray_tracer.camera += ray_tracer.forward() * move_speed;
    }
    if ctx.input().key_down(egui::Key::D) {
      ray_tracer.camera += ray_tracer.right() * move_speed;
    }
    if ctx.input().key_down(egui::Key::A) {
      ray_tracer.camera -= ray_tracer.right() * move_speed;
    }
    if ctx.input().key_down(egui::Key::E) {
      ray_tracer.camera += ray_tracer.up() * move_speed;
    }
    if ctx.input().key_down(egui::Key::Q) {
      ray_tracer.camera -= ray_tracer.up() * move_speed;
    }

    ray_tracer.scene.objects.iter_mut().for_each(|object| {
      let position = object.geometry.position_as_mut();
      let length = position.length();

      let theta: f64 = 0.5 * std::f64::consts::PI * delta_time;

      *position = position.transform_point(Mat44::create_rotation(Axis::Y, theta));

      // fix rounding errors?
      *position = *position * (length / position.length());
    });

    let settings_panel = |ui: &mut egui::Ui| {
      ui.heading("Settings");

      ui.label(format!("fps: {}", 1. / frame_times.average().unwrap_or(1.)));

      ui.separator();

      ui.horizontal(|ui| {
        let mut new_width = ray_tracer.width;
        let mut new_height = ray_tracer.height;

        ui.label("width");
        ui.add(egui::DragValue::new(&mut new_width)
          .speed(20));
        ui.label("height");
        ui.add(egui::DragValue::new(&mut new_height)
          .speed(20));

        if new_width != ray_tracer.width || new_height != ray_tracer.height {
          has_size_changed = true;
          ray_tracer.width = new_width;
          ray_tracer.height = new_height;
        }
      });

      ui.separator();

      vec3_widget(ui, "pos", &mut ray_tracer.camera);
      vec3_widget(ui, "rot", &mut ray_tracer.rotation);

      ui.separator();

      ui.horizontal(|ui| {
        ui.label("bounces");
        ui.add(egui::DragValue::new(&mut ray_tracer.scene.reflection_limit)
          .clamp_range::<u32>(0..=10));
      });
    };

    let object_panel = |ui: &mut egui::Ui| {
      ui.horizontal(|ui| {
        if ui.add(egui::Button::new("➕ sphere")).clicked() {
          ray_tracer.scene.objects.push(Object {
            name: String::from("sphere"),
            material: Material {
              colour: (1., 0., 0.),
              specular: 500.,
              metallic: 0.5,
            },
            geometry: Geometry::Sphere {
              center: Vec3 { x: 0., y: 0., z: 0., },
              radius: 1.,
            },
          });
        }
        if ui.add(egui::Button::new("➕ plane")).clicked() {
          ray_tracer.scene.objects.push(Object {
            name: String::from("plane"),
            material: Material {
              colour: (1., 0., 0.),
              specular: 500.,
              metallic: 0.5,
            },
            geometry: Geometry::Plane {
              center: Vec3 { x: 0., y: 0., z: 0., },
              normal: Vec3 { x: 0., y: 1., z: 0., },
              size: 5.,
            },
          });
        }
        if ui.add(egui::Button::new("print")).clicked() {
          println!("{:#?}", ray_tracer.scene.objects);
        }
      });

      ui.separator();

      let mut has_removed_object = false;

      for i in 0..ray_tracer.scene.objects.len() {
        let index = if has_removed_object { i - 1 } else { i };

        ui.horizontal(|ui| {
          ui.label(&ray_tracer.scene.objects[index].name);

          if ui.add(egui::Button::new("❌")).clicked() {
            ray_tracer.scene.objects.remove(index);
            has_removed_object = true;
          }
        });

        vec3_widget(ui, "pos", ray_tracer.scene.objects[index].geometry.position_as_mut());

        if has_removed_object {
          continue;
        }

        let object = &mut ray_tracer.scene.objects[index];

        match &mut object.geometry {
          Geometry::Sphere { center: _, radius } => {
            ui.horizontal(|ui| {
              ui.label("radius");
              ui.add(egui::DragValue::new(radius)
                .fixed_decimals(1)
                .speed(0.1));
            });
          },
          Geometry::Plane { center: _, normal, size } => {
            ui.horizontal(|ui| {
              ui.label("normal");
              ui.add(egui::DragValue::new(&mut normal.x)
                .fixed_decimals(1)
                .speed(0.1));
              ui.add(egui::DragValue::new(&mut normal.y)
                .fixed_decimals(1)
                .speed(0.1));
              ui.add(egui::DragValue::new(&mut normal.z)
                .fixed_decimals(1)
                .speed(0.1));
              
              *normal = normal.normalize();
            });

            ui.horizontal(|ui| {
              ui.label("size");
              ui.add(egui::DragValue::new(size)
                .fixed_decimals(1)
                .speed(0.1));
            });
          }
        }

        ui.horizontal(|ui| {
          ui.label("col");

          let mut colour = [object.material.colour.0 as f32, object.material.colour.1 as f32, object.material.colour.2 as f32];

          ui.color_edit_button_rgb(&mut colour);

          object.material.colour = (colour[0] as f64, colour[1] as f64, colour[2] as f64);

          ui.label("spec");
          ui.add(egui::DragValue::new(&mut object.material.specular)
            .clamp_range::<f64>(0.0..=1000.));
          
          ui.label("met");
          ui.add(egui::DragValue::new(&mut object.material.metallic)
            .clamp_range::<f64>(0.0..=1.)
            .speed(0.1));
        });

        ui.separator();
      };
    };

    if is_portrait {
      egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        egui::SidePanel::left("object_panel").show_inside(ui, object_panel);
        egui::SidePanel::right("settings_panel").show_inside(ui, settings_panel);
      });
    } else {
      egui::SidePanel::right("settings_panel").show(ctx, settings_panel);
      egui::SidePanel::right("object_panel").show(ctx, object_panel);
    }

    egui::CentralPanel::default().show(ctx, |ui| {
      ui.set_max_width(f32::INFINITY);
      ui.set_max_height(f32::INFINITY);
      match texture {
        Some(texture) => {
          egui::Resize::default()
            .default_size((ray_tracer.width as f32, ray_tracer.height as f32))
            .show(ui, |ui| {
              if !has_size_changed {
                ray_tracer.width = ui.available_width() as u32;
                ray_tracer.height = ui.available_height() as u32;
              }

              ray_tracer.rs_render(image);

              texture.set(eframe::epaint::ImageData::Color(image.clone()));
              ui.add(egui::Image::new(&*texture, texture.size_vec2()));
            });
        },
        None => (),
      }
    });

    ctx.request_repaint();
  }
}

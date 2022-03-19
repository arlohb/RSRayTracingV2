use eframe::{egui, epi};

pub struct TemplateApp {
  // Example stuff:
  label: String,

  value: f32,
}

impl Default for TemplateApp {
  fn default() -> Self {
    Self {
      // Example stuff:
      label: "Hello World!".to_owned(),
      value: 2.7,
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
  fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
    let Self { label, value } = self;

    // Examples of how to create different panels and windows.
    // Pick whichever suits you.
    // Tip: a good default choice is to just keep the `CentralPanel`.
    // For inspiration and more examples, go to https://emilk.github.io/egui

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
      // The top panel is often a good place for a menu bar:
      egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
          if ui.button("Quit").clicked() {
            frame.quit();
          }
        });
      });
    });

    egui::SidePanel::left("side_panel").show(ctx, |ui| {
      ui.heading("Side Panel");

      ui.horizontal(|ui| {
        ui.label("Write something: ");
        ui.text_edit_singleline(label);
      });

      ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
      if ui.button("Increment").clicked() {
        *value += 1.0;
      }

      ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        ui.horizontal(|ui| {
          ui.spacing_mut().item_spacing.x = 0.0;
          ui.label("powered by ");
          ui.hyperlink_to("egui", "https://github.com/emilk/egui");
          ui.label(" and ");
          ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
        });
      });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
      // The central panel the region left after adding TopPanel's and SidePanel's

      ui.heading("eframe template");
      ui.hyperlink("https://github.com/emilk/eframe_template");
      ui.add(egui::github_link_file!(
        "https://github.com/emilk/eframe_template/blob/master/",
        "Source code."
      ));
    });
  }
}

use crate::git::git_settings::set_git_env;
use crate::git::git_version::load_git_version;
use eframe::egui;
use eframe::egui::ComboBox;
use eframe::egui::{Style, Visuals};

mod config;
pub(crate) mod git;
mod index;
mod parser;
mod server;
mod util;

fn main() -> eframe::Result {
  set_git_env();
  load_git_version();
  // start_async_server();

  start_gui()
}

fn start_gui() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
    ..Default::default()
  };

  eframe::run_native(
    "GitFiend2",
    options,
    Box::new(|cc| {
      egui_extras::install_image_loaders(&cc.egui_ctx);
      cc.egui_ctx.set_style(Style {
        visuals: Visuals::light(),
        ..Style::default()
      });
      Ok(Box::<MyApp>::default())
    }),
  )
}

struct MyApp {
  name: String,
  age: u32,
  repos: Vec<String>,
  selected: usize,
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
      name: "Arthur".to_owned(),
      age: 42,
      repos: vec![
        String::from("gitfiend-core"),
        String::from("egui"),
        String::from("cottontail-js"),
      ],
      selected: 0,
    }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ComboBox::from_id_salt("repo-selector")
        .selected_text(self.repos[self.selected].to_string())
        .show_ui(ui, |ui| {
          for i in 0..self.repos.len() {
            let value = ui.selectable_value(
              &mut &self.repos[i],
              &self.repos[self.selected],
              &self.repos[i],
            );
            if value.clicked() {
              self.selected = i;
            }
          }
        });
      ui.horizontal(|ui| {
        // ui
        let _ = ui.button("Repo");
        let _ = ui.button("Branch");
      });

      ui.horizontal(|ui| {
        let name_label = ui.label("Your name: ");
        ui.text_edit_singleline(&mut self.name)
          .labelled_by(name_label.id);
      });

      ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
      if ui.button("Increment").clicked() {
        self.age += 1;
      }
      ui.add_space(14.0);
      ui.label(format!("Hello '{}', age {}", self.name, self.age));

      // ui.image(egui::include_image!(
      //           "../../../crates/egui/assets/ferris.png"
      //       ));
    });
  }
}

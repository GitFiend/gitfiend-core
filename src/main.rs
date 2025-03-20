use crate::git::git_settings::set_git_env;
use crate::git::git_version::load_git_version;
use crate::server::requests::start_async_server;
use eframe::egui;
use std::thread;

mod config;
pub(crate) mod git;
mod index;
mod parser;
mod server;
mod util;

fn main() -> eframe::Result {
  set_git_env();
  load_git_version();

  thread::spawn(|| {
    start_async_server();
  });

  start_gui()
}

fn start_gui() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
    ..Default::default()
  };

  eframe::run_native(
    "My egui App",
    options,
    Box::new(|cc| {
      // This gives us image support:
      egui_extras::install_image_loaders(&cc.egui_ctx);

      Ok(Box::<MyApp>::default())
    }),
  )
}

struct MyApp {
  name: String,
  age: u32,
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
      name: "Arthur".to_owned(),
      age: 42,
    }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("My egui Application");
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

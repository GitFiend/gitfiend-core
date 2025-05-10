use crate::ui::window::{App, Message};
use iced::widget::{Space, Text, button, column, row, svg};
use iced::{Element, Length};
use std::path::Path;

pub fn toolbar(app: &App) -> Element<Message> {
  let side_width = (app.window_size.width - 180.0) / 2.0;

  let row = row![
    row![
      Space::with_width(Length::Fill),
      repo_button(app),
      button("Branches"),
      Space::with_width(Length::Fill),
    ]
    .width(side_width),
    row![
      nav_button(
        include_bytes!("../../resources/changes-view.svg"),
        "Changes"
      ),
      button("Commits"),
      button("Search")
    ]
    .width(180),
    row![
      Space::with_width(Length::Fill),
      button("Pull"),
      button("Push"),
      button("Fetch"),
      Space::with_width(Length::Fill),
    ]
    .width(side_width),
  ];

  row.into()
}

fn repo_button(app: &App) -> Element<Message> {
  if let Some(repo) = &app.repo {
    let name = Path::new(&repo.repo_path)
      .file_name()
      .unwrap()
      .to_str()
      .unwrap();

    return column![
      button(Text::new(name).size(13)),
      button(Text::new("Branch").size(12))
    ]
    .into();
  }

  button("Recent...").into()
}

fn nav_button<'a>(icon: &'a [u8], text: &'a str) -> Element<'a, Message> {
  let b = include_bytes!("../../resources/changes-view.svg");

  let icon = svg(svg::Handle::from_memory(b)).width(21);

  button(column![icon, Text::new(text).size(12)])
    .width(60)
    .into()
}

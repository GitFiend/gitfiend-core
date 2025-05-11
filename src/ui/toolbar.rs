use crate::ui::window::{App, Message};
use iced::widget::{Button, Space, Svg, Text, button, column, row, svg};
use iced::{Alignment, Element, Length};
use std::path::Path;

const CHANGES_SVG: &[u8] = include_bytes!("../../resources/changes-view.svg");
const COMMITS_SVG: &[u8] = include_bytes!("../../resources/commits-view.svg");
const SEARCH_SVG: &[u8] = include_bytes!("../../resources/search.svg");

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
      nav_button(CHANGES_SVG, "Changes"),
      nav_button(COMMITS_SVG, "Commits"),
      nav_button(SEARCH_SVG, "Search"),
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

fn nav_button<'a>(icon: &'static [u8], text: &'a str) -> Element<'a, Message> {
  let icon: Svg = svg(svg::Handle::from_memory(icon)).width(21).height(19);

  button(
    row![
      column![icon, Space::with_height(3), Text::new(text).size(12)]
        .width(60)
        .align_x(Alignment::Center),
    ]
    .height(48)
    .align_y(Alignment::Center),
  )
  .padding(0)
  .width(60)
  .into()
}

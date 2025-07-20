mod repo_selector;

use crate::ui::window::{App, Message};
use iced::widget::button::text;
use iced::widget::{Space, Svg, Text, button, column, pick_list, row, svg};
use iced::{Alignment, Element, Length, Task};
use std::path::Path;

const CHANGES_SVG: &[u8] = include_bytes!("../../../resources/changes-view.svg");
const COMMITS_SVG: &[u8] = include_bytes!("../../../resources/commits-view.svg");
const SEARCH_SVG: &[u8] = include_bytes!("../../../resources/search.svg");
const PULL_SVG: &[u8] = include_bytes!("../../../resources/pull.svg");
const PUSH_SVG: &[u8] = include_bytes!("../../../resources/push.svg");
const FETCH_SVG: &[u8] = include_bytes!("../../../resources/fetch.svg");

#[derive(Debug, Clone, Copy)]
pub enum ToolbarMsg {
  Changes,
  Commits,
  Search,
  Pull,
  Push,
  Fetch,
}

#[derive(Default)]
pub enum CurrentView {
  Changes,
  #[default]
  Commits,
  Branches,
}

pub fn on_toolbar_message(app: &mut App, msg: ToolbarMsg) -> Task<Message> {
  println!("Toolbar message: {:?}", msg);
  Task::none()
}

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
        CHANGES_SVG,
        "Changes",
        Message::Toolbar(ToolbarMsg::Changes)
      ),
      nav_button(
        COMMITS_SVG,
        "Commits",
        Message::Toolbar(ToolbarMsg::Commits)
      ),
      nav_button(SEARCH_SVG, "Search", Message::Toolbar(ToolbarMsg::Search)),
    ]
    .width(180),
    row![
      Space::with_width(Length::Fill),
      icon_button(PULL_SVG, "Pull", Message::Toolbar(ToolbarMsg::Pull)),
      icon_button(PUSH_SVG, "Push", Message::Toolbar(ToolbarMsg::Push)),
      icon_button(FETCH_SVG, "Fetch", Message::Toolbar(ToolbarMsg::Fetch)),
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

fn icon_button<'a>(
  icon: &'static [u8],
  label: &'a str,
  msg: Message,
) -> Element<'a, Message> {
  let icon: Svg = svg(svg::Handle::from_memory(icon)).width(21).height(19);

  button(
    row![
      column![icon, Space::with_height(3), Text::new(label).size(12)]
        .width(50)
        .align_x(Alignment::Center),
    ]
    .height(48)
    .align_y(Alignment::Center),
  )
  .padding(0)
  .width(60)
  .style(text)
  .on_press(msg)
  .into()
}

fn nav_button<'a>(
  icon: &'static [u8],
  label: &'a str,
  msg: Message,
) -> Element<'a, Message> {
  let icon: Svg = svg(svg::Handle::from_memory(icon)).width(21).height(19);

  button(
    row![
      column![icon, Space::with_height(3), Text::new(label).size(12)]
        .width(60)
        .align_x(Alignment::Center),
    ]
    .height(48)
    .align_y(Alignment::Center),
  )
  .padding(0)
  .width(60)
  .style(text)
  .on_press(msg)
  .into()
}

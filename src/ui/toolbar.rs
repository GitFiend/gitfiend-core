use crate::ui::window::{App, Message};
use iced::widget::{Space, button, row};
use iced::{Element, Length};

pub fn toolbar<'a>(app: &App) -> Element<'a, Message> {
  let side_width = (app.window_size.width - 180.0) / 2.0;

  let row = row![
    row![
      Space::with_width(Length::Fill),
      button("Repo"),
      button("Branches"),
      Space::with_width(Length::Fill),
    ]
    .width(side_width),
    row![button("Changes"), button("Commits"), button("Search")].width(180),
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

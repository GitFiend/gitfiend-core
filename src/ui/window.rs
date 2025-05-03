use iced::Theme;
use iced::widget::{Space, button, row, text};

struct App {
  count: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
  IncrementCount,
  DecrementCount,
}

impl App {
  fn new() -> Self {
    Self { count: 0 }
  }

  fn update(&mut self, message: Message) -> iced::Task<Message> {
    // handle emitted messages
    match message {
      Message::IncrementCount => self.count += 1,
      Message::DecrementCount => self.count -= 1,
    }
    iced::Task::none()
  }

  fn view(&self) -> iced::Element<'_, Message> {
    let row = row![
      button("-").on_press(Message::DecrementCount),
      Space::with_width(100),
      text(self.count.to_string()),
      Space::with_width(100),
      button("+").on_press(Message::IncrementCount)
    ];
    row.into()
  }
}

pub fn make_application_window() -> iced::Result {
  iced::application("GitFiend", App::update, App::view)
    .theme(|_| Theme::Dark)
    .run_with(|| (App::new(), iced::Task::none()))
}

use iced::widget::{button, column, row, text};
use iced::{Element, Theme};

#[derive(Debug, Clone)]
enum Message {
  Increment,
}

#[derive(Default)]
struct State {
  pub counter: u64,
}

pub fn make_application_window() -> iced::Result {
  iced::application("GitFiend", update, view)
    .theme(|_| Theme::Dark)
    .centered()
    .run()
}

fn view(state: &State) -> Element<Message> {
  column! {
    row! {
      button(text("Hello2")).on_press(Message::Increment),
      button(text(state.counter)).on_press(Message::Increment)
    }
  }
  .into()
}

fn update(state: &mut State, message: Message) {
  match message {
    Message::Increment => state.counter += 1,
  }
}

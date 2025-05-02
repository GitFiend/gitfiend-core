use crate::git::git_settings::set_git_env;
use crate::git::git_version::load_git_version;
use crate::server::requests::start_async_server;
use iced::Element;
use iced::widget::{button, column, row, text};
use std::thread;

mod config;
pub mod git;
mod index;
mod parser;
mod server;
mod util;

fn main() -> iced::Result {
  set_git_env();
  load_git_version();

  thread::spawn(|| {
    start_async_server();
  });

  iced::run("GitFiend", update, view)
}

#[derive(Debug, Clone)]
enum Message {
  Increment,
}

fn view(counter: &u64) -> Element<Message> {
  column! {
    row! {
      button(text("Hello2")).on_press(Message::Increment),
      button(text(counter)).on_press(Message::Increment)
    }
  }
  .into()
}

fn update(counter: &mut u64, message: Message) {
  match message {
    Message::Increment => *counter += 1,
  }
}

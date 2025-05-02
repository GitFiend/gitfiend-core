use crate::git::git_settings::set_git_env;
use crate::git::git_version::load_git_version;
use crate::server::requests::start_async_server;
use iced::widget::{button, column, row, text};
use iced::{Application, Element, Theme};
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

  let no_server = true;

  thread::spawn(move || {
    if !no_server {
      start_async_server();
    }
  });

  println!("{:?}", Theme::default());

  iced::run("GitFiend", update, view)
}

#[derive(Debug, Clone)]
enum Message {
  Increment,
}

#[derive(Default)]
struct State {
  pub counter: u64,
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

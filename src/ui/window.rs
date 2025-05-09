use crate::git::queries::scan_workspace::{ScanOptions, scan_workspace};
use iced::widget::{Space, button, row, text};
use iced::{Element, Result, Subscription, Task, Theme, application, time};
use std::time::Duration;
use std::{env, thread};

struct App {
  count: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
  IncrementCount,
  Add(i32),
  DecrementCount,
  Tick(chrono::DateTime<chrono::Local>),
  SlowTick,
}

impl App {
  fn new() -> Self {
    if let Ok(repo) = env::current_dir() {
      let repo_path = repo.to_str().unwrap();
      scan_workspace(&ScanOptions {
        repo_path: repo_path.to_string(),
        workspaces_enabled: false,
      });
    }

    Self { count: 0 }
  }

  fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::Add(n) => {
        self.count += n;
        Task::none()
      }
      Message::IncrementCount => {
        self.count += 1;
        Task::none()
      }
      Message::DecrementCount => {
        self.count -= 1;
        Task::none()
      }
      Message::Tick(time) => {
        println!("Tick at {}", time);
        Task::none()
      }
      Message::SlowTick => Task::future(async {
        let handle = thread::spawn(|| {
          thread::sleep(Duration::from_secs(2));
          42
        });

        let result = handle.join().unwrap();
        Message::Add(result)
      }),
    }
  }

  fn view(&self) -> Element<'_, Message> {
    let row = row![
      button("-").on_press(Message::DecrementCount),
      Space::with_width(100),
      text(self.count.to_string()),
      Space::with_width(100),
      button("+").on_press(Message::IncrementCount),
      button("add 2").on_press(Message::Add(2))
    ];
    row.padding(10).into()
  }

  fn subscription(&self) -> Subscription<Message> {
    Subscription::batch(vec![
      time::every(Duration::from_millis(500))
        .map(|_| Message::Tick(chrono::offset::Local::now())),
      time::every(Duration::from_secs(2)).map(|_| Message::SlowTick),
    ])
  }
}

pub fn make_application_window() -> Result {
  application("GitFiend", App::update, App::view)
    .theme(|_| Theme::Dark)
    .subscription(App::subscription)
    .run_with(|| (App::new(), Task::none()))
}

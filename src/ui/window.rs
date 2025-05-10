use crate::git::git_types::{Commit, RefInfo};
use crate::git::queries::commits::{ReqCommitsOptions2, load_commits_and_refs};
use crate::git::queries::scan_workspace::{ScanOptions, scan_workspace};
use crate::git::queries::workspace::repo_status::{RepoStatus, load_repo_status};
use crate::server::git_request::ReqOptions;
use iced::widget::{Space, button, row, text};
use iced::{Element, Result, Subscription, Task, Theme, application, time};
use std::time::Duration;
use std::{env, thread};

#[derive(Default)]
struct App {
  count: i32,
  repo: Option<Repo>,
}

#[derive(Debug)]
struct Repo {
  status: RepoStatus,
  commits: Vec<Commit>,
  refs: Vec<RefInfo>,
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
    let mut app = App::default();

    if let Ok(repo) = env::current_dir() {
      let repo_path = repo.to_str().unwrap().to_string();

      scan_workspace(&ScanOptions {
        repo_path: repo_path.clone(),
        workspaces_enabled: false,
      });

      if let Ok(status) = load_repo_status(&ReqOptions {
        repo_path: repo_path.clone(),
      }) {
        if let Ok((commits, refs)) = load_commits_and_refs(&ReqCommitsOptions2 {
          repo_path,
          num_commits: 1000,
          filters: Vec::default(),
          fast: false,
          skip_stashes: false,
        }) {
          app.repo = Some(Repo {
            status,
            commits,
            refs,
          });
        }
      }
    }

    app
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
    row.height(48).padding(10).into()
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
    // .subscription(App::subscription)
    .run_with(|| (App::new(), Task::none()))
}

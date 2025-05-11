use crate::git::git_types::{Commit, RefInfo};
use crate::git::queries::commits::{ReqCommitsOptions2, load_commits_and_refs};
use crate::git::queries::scan_workspace::{ScanOptions, scan_workspace};
use crate::git::queries::workspace::repo_status::{RepoStatus, load_repo_status};
use crate::server::git_request::ReqOptions;
use crate::ui::toolbar::{CurrentView, ToolbarMsg, on_toolbar_message, toolbar};
use iced::widget::column;
use iced::{Element, Result, Size, Subscription, Task, Theme, application, window};
use std::env;

#[derive(Default)]
pub struct App {
  pub repo: Option<Repo>,
  pub window_size: Size,
  pub view: CurrentView,
}

#[derive(Debug)]
pub struct Repo {
  pub status: RepoStatus,
  pub commits: Vec<Commit>,
  pub refs: Vec<RefInfo>,
  pub repo_path: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  WindowResized(Size),
  Toolbar(ToolbarMsg),
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
          repo_path: repo_path.clone(),
          num_commits: 1000,
          filters: Vec::default(),
          fast: false,
          skip_stashes: false,
        }) {
          app.repo = Some(Repo {
            status,
            commits,
            refs,
            repo_path,
          });
        }
      }
    }

    app
  }

  fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::WindowResized(size) => {
        self.window_size = size;
        Task::none()
      }
      Message::Toolbar(toolbar) => on_toolbar_message(self, toolbar),
    }
  }

  fn view(&self) -> Element<Message> {
    let row = column![toolbar(self)];
    row.into()
  }

  fn subscription(&self) -> Subscription<Message> {
    Subscription::batch(vec![
      window::resize_events().map(|event| Message::WindowResized(event.1)),
    ])
  }
}

pub fn make_application_window() -> Result {
  application("GitFiend", App::update, App::view)
    .theme(|_| Theme::Dark)
    .subscription(App::subscription)
    .run_with(|| (App::new(), Task::none()))
}

use crate::git::git_types::GitConfig;
use crate::git::queries::config::config_file_parser::make_config_log;
use crate::git::queries::config::config_output_parser::P_SUBMODULE_NAME;
use crate::git::run_git::{run_git_err, RunGitOptions};
use crate::git::store::CONFIG;
use crate::parser::{parse_all_err, run_parser, ParseOptions};
use crate::server::git_request::ReqOptions;
use crate::server::request_util::R;
use config_output_parser::{P_CONFIG, P_REMOTE_NAME};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

mod config_file_parser;
mod config_output_parser;

impl GitConfig {
  pub fn new() -> GitConfig {
    GitConfig {
      entries: HashMap::new(),
      remotes: HashMap::new(),
      submodules: HashMap::new(),
    }
  }

  // We take short_name because this is the same between remote and local refs.
  pub fn get_remote_for_branch(&self, short_name: &str) -> String {
    let GitConfig { entries, .. } = self;

    if let Some(push_remote) = entries.get(&format!("branch.{}.pushremote", short_name)) {
      return push_remote.clone();
    }

    if let Some(push_default) = entries.get("remote.pushdefault") {
      return push_default.clone();
    }

    if let Some(remote) = entries.get(&format!("branch.{}.remote", short_name)) {
      return remote.clone();
    }

    String::from("origin")
  }

  pub fn get_tracking_branch_name(&self, local_branch: &str) -> String {
    let remote = self.get_remote_for_branch(local_branch);

    format!("refs/remotes/{}/{}", remote, local_branch)
  }
}

// Use this version on focus of GitFiend only. Get it from the store otherwise.
pub fn load_full_config(options: &ReqOptions) -> R<GitConfig> {
  let ReqOptions { repo_path } = options;

  let config_path = Path::new(repo_path).join(".git").join("config");

  let result_text = if let Ok(text) = read_to_string(config_path) {
    make_config_log(&text)
  } else {
    // If new config parser fails, fallback to the old one.
    Ok(
      run_git_err(RunGitOptions {
        repo_path,
        args: ["config", "--list"],
      })?
      .stdout,
    )
  };

  let config_result = parse_all_err(P_CONFIG, result_text?.as_str());
  let entries = config_result?;
  let mut remotes = HashMap::new();
  let mut submodules = HashMap::new();

  for (key, value) in entries.iter() {
    if key.starts_with("remote") {
      let name = run_parser(
        P_REMOTE_NAME,
        key,
        ParseOptions {
          must_parse_all: true,
          print_error: false,
        },
      );

      if let Some(name) = name {
        remotes.insert(name, value.clone());
      }
    } else if key.starts_with("submodule") {
      let name = run_parser(
        P_SUBMODULE_NAME,
        key,
        ParseOptions {
          must_parse_all: true,
          print_error: false,
        },
      );

      if let Some(name) = name {
        submodules.insert(name, value.clone());
      }
    }
  }

  let config = GitConfig {
    entries,
    remotes,
    submodules,
  };

  CONFIG.insert(repo_path.clone(), config.clone());

  Ok(config)
}

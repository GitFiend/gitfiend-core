use crate::server::git_request::ReqOptions;
use run_git::RunGitOptions;

pub(crate) mod git_types;
pub(crate) mod git_types_extra_impl;
pub(crate) mod queries;
pub(crate) mod run_git;
pub(crate) mod store;

// Expect this to return none if Git is not installed.
pub fn git_version(_: &ReqOptions) -> Option<String> {
  run_git::run_git(RunGitOptions {
    repo_path: ".",
    args: ["--version"],
  })
}

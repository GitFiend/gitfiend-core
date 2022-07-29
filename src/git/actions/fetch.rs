use crate::git::run_git_action::{run_git_action, RunGitActionOptions};
use crate::git::store::GIT_VERSION;
use crate::server::git_request::ReqOptions;

pub fn fetch_all(options: &ReqOptions) -> Option<()> {
  let out = run_git_action(RunGitActionOptions {
    repo_path: &options.repo_path,
    git_version: GIT_VERSION.get()?,
    args: ["fetch", "--all", "--prune"],
  })?;

  eprintln!("{:?} {:?}", out.stdout, out.stderr);

  Some(())
}

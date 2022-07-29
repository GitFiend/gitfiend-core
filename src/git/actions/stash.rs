use crate::git::run_git_action::{run_git_action, ActionResult, RunGitActionOptions};
use crate::git::store::GIT_VERSION;
use crate::server::git_request::ReqOptions;

pub fn stash_changes(options: &ReqOptions) -> Option<()> {
  let version = GIT_VERSION.get()?;

  let out = run_git_action(RunGitActionOptions {
    git_version: version.clone(),
    repo_path: &options.repo_path,
    args: ["add", "--all"],
  })?;

  println!("{:?} {:?}", out.stdout, out.stderr);

  let out = run_git_action(RunGitActionOptions {
    git_version: version,
    repo_path: &options.repo_path,
    args: ["stash", "push"],
  })?;

  println!("{:?} {:?}", out.stdout, out.stderr);

  Some(())
}

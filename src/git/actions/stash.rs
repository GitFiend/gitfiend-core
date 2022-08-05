use crate::git::git_version::GitVersion;
use crate::git::run_git_action::{run_git_action, ActionError, ActionOutput, RunGitActionOptions};
use crate::git::store::GIT_VERSION;
use crate::server::git_request::ReqOptions;

pub fn stash_changes(options: &ReqOptions) -> Result<ActionOutput, ActionError> {
  let version = GIT_VERSION.get().unwrap_or_else(GitVersion::new);

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

  Ok(out)
}

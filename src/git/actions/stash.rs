use crate::git::git_version::GitVersion;
use crate::git::run_git_action::{run_git_action3, RunGitActionOptions2};
use crate::git::store::GIT_VERSION;
use crate::server::git_request::ReqOptions;

pub fn stash_changes(options: &ReqOptions) -> u32 {
  // let version = GIT_VERSION.get().unwrap_or_else(GitVersion::new);
  //
  // let out = run_git_action(RunGitActionOptions {
  //   git_version: version.clone(),
  //   repo_path: &options.repo_path,
  //   args: ["add", "--all"],
  // })?;
  //
  // println!("{:?} {:?}", out.stdout, out.stderr);
  //
  // let out = run_git_action(RunGitActionOptions {
  //   git_version: version.clone(),
  //   repo_path: &options.repo_path,
  //   args: ["stash", "push"],
  // })?;

  run_git_action3(RunGitActionOptions2 {
    git_version: GIT_VERSION.get().unwrap_or_else(GitVersion::new),
    repo_path: &options.repo_path,
    commands: [vec!["add", "--all"], vec!["stash", "push"]],
  })

  // println!("{:?} {:?}", out.stdout, out.stderr);
  //
  // Ok(out)
}

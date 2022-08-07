use crate::git::git_version::GitVersion;
use crate::git::run_git_action::{run_git_action3, RunGitActionOptions2};
use crate::git::store::GIT_VERSION;
use crate::server::git_request::ReqOptions;

// pub fn fetch_allOld(options: &ReqOptions) -> Option<()> {
//   let out = run_git_action(RunGitActionOptions {
//     repo_path: &options.repo_path,
//     git_version: GIT_VERSION.get()?,
//     args: ["fetch", "--all", "--prune"],
//   })?;
//
//   eprintln!("{:?} {:?}", out.stdout, out.stderr);
//
//   Some(())
// }

pub fn fetch_all(options: &ReqOptions) -> u32 {
  run_git_action3(RunGitActionOptions2 {
    repo_path: &options.repo_path,
    git_version: GIT_VERSION.get().unwrap_or_else(GitVersion::new),
    commands: [vec!["fetch", "--all", "--prune"]],
  })
}

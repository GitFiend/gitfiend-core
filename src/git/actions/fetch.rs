use crate::git::run_git_action::{run_git_action, ActionError, ActionOutput, RunGitActionOptions};
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

pub fn fetch_all(options: &ReqOptions) -> Result<ActionOutput, ActionError> {
  run_git_action(RunGitActionOptions {
    repo_path: &options.repo_path,
    git_version: GIT_VERSION
      .get()
      .ok_or_else(|| ActionError::IO("GIT_VERSION.get() returned None".to_string()))?,
    args: ["fetch", "--all", "--prune"],
  })
}

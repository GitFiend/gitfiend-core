use crate::git::git_types::WipPatchType;
use crate::git::{run_git, RunGitOptions};
use crate::server::git_request::ReqOptions;

#[derive(Debug, PartialEq)]
pub struct WipPatchInfo {
  pub old_file: String,
  pub new_file: String,
  pub staged: WipPatchType,
  pub un_staged: WipPatchType,
}

pub fn load_wip_patches(options: &ReqOptions) {
  let out = run_git(RunGitOptions {
    repo_path: &options.repo_path,
    args: ["status", "--porcelain", "-uall", "-z"],
  });
}

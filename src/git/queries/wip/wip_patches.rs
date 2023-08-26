use crate::git::git_types::{WipPatch, WipPatchType};
use crate::git::queries::patches::file_is_image;
use crate::git::queries::wip::read_merge_head;
use crate::git::queries::wip::wip_patch_parsers::P_WIP_PATCHES;
use crate::git::run_git::RunGitOptions;
use crate::git::run_git::{run_git_err, GitOut};
use crate::parser::parse_all_err;
use crate::server::git_request::ReqOptions;
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, PartialEq, Eq)]
pub struct WipPatchInfo {
  pub old_file: String,
  pub new_file: String,
  pub staged: WipPatchType,
  pub un_staged: WipPatchType,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct WipPatches {
  pub patches: Vec<WipPatch>,
  pub conflict_commit_id: Option<String>,
}

pub fn load_wip_patches(options: &ReqOptions) -> Result<WipPatches, String> {
  let GitOut { stdout, .. } = run_git_err(RunGitOptions {
    repo_path: &options.repo_path,
    args: ["status", "--porcelain", "-uall", "-z"],
  })?;

  let info = parse_all_err(P_WIP_PATCHES, &stdout)?;

  let (patches, conflicted) = get_patches_from_info(info);

  if conflicted {
    if let Some(id) = read_merge_head(&options.repo_path) {
      return Ok(WipPatches {
        patches,
        conflict_commit_id: Some(id),
      });
    }
  }

  Ok(WipPatches {
    patches,
    conflict_commit_id: None,
  })
}

fn get_patches_from_info(info: Vec<WipPatchInfo>) -> (Vec<WipPatch>, bool) {
  let mut patches: Vec<WipPatch> = Vec::new();
  let mut have_conflict = false;

  for WipPatchInfo {
    old_file,
    new_file,
    un_staged,
    staged,
  } in info
  {
    let conflicted = is_conflicted(&staged, &un_staged);

    if conflicted {
      have_conflict = true;
    }
    let patch_type = pick_type_from_patch(&un_staged, &staged);

    patches.push(WipPatch {
      old_file: old_file.clone(),
      new_file: new_file.clone(),
      patch_type: patch_type.clone(),
      staged_type: staged,
      un_staged_type: un_staged,
      conflicted,
      id: format!("{}{}", &new_file, patch_type),
      is_image: file_is_image(&new_file),
    })
  }

  patches.sort_by_key(|p| p.new_file.to_lowercase());

  if have_conflict {
    // We aren't interested in any other patches when there's a conflict.
    return (patches.into_iter().filter(|p| p.conflicted).collect(), true);
  }

  (patches, false)
}

fn is_conflicted(left: &WipPatchType, right: &WipPatchType) -> bool {
  *left == WipPatchType::U
    || *right == WipPatchType::U
    || (*left == WipPatchType::A && *right == WipPatchType::A)
    || (*left == WipPatchType::D && *right == WipPatchType::D)
}

fn pick_type_from_patch(un_staged: &WipPatchType, staged: &WipPatchType) -> WipPatchType {
  if un_staged != &WipPatchType::Empty {
    if un_staged == &WipPatchType::Question {
      return WipPatchType::A;
    }
    return un_staged.clone();
  }
  if staged == &WipPatchType::Question {
    return WipPatchType::A;
  }

  staged.clone()
}

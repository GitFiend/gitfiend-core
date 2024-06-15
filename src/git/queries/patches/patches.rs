use std::collections::HashMap;

use crate::{dprintln, f};
use loggers::elapsed;

use crate::git::git_types::{Commit, Patch};
use crate::git::queries::patches::cache::{load_patches_cache, write_patches_cache};
use crate::git::queries::patches::patch_parsers::{
  map_data_to_patch, P_MANY_PATCHES_WITH_COMMIT_IDS, P_PATCHES,
};
use crate::git::queries::COMMIT_0_ID;
use crate::git::run_git::{run_git_err, RunGitOptions};
use crate::parser::parse_all_err;
use crate::server::request_util::R;

#[elapsed]
pub fn load_patches(
  repo_path: &str,
  commits: &Vec<Commit>,
) -> R<HashMap<String, Vec<Patch>>> {
  let mut commits_without_patches = Vec::<&Commit>::new();
  let mut stashes_or_merges_without_patches = Vec::<&Commit>::new();
  let mut new_patches = HashMap::<String, Vec<Patch>>::new();

  if let Some(patches) = load_patches_cache(repo_path) {
    for c in commits.iter() {
      if let Some(patch) = patches.get(&c.id) {
        new_patches.insert(c.id.clone(), patch.clone());
      } else if c.stash_id.is_none() && !c.is_merge {
        commits_without_patches.push(c);
      } else {
        stashes_or_merges_without_patches.push(c);
      }
    }
  } else {
    // No existing patch cache.
    for c in commits {
      if c.stash_id.is_none() && !c.is_merge {
        commits_without_patches.push(c);
      } else {
        stashes_or_merges_without_patches.push(c);
      }
    }
  }

  if cfg!(debug_assertions) {
    let num_commits_with_no_patches =
      new_patches.iter().filter(|(_, p)| p.is_empty()).count();

    dprintln!("******* {} commits_without_patches, {} stashes_or_merges_without_patches, {} commits with 0 *******",
              commits_without_patches.len(), stashes_or_merges_without_patches.len(), num_commits_with_no_patches);

    if num_commits_with_no_patches > 0 {
      dprintln!(
        "Commits with no patches: {:?}",
        new_patches
          .iter()
          .filter(|(_, p)| p.is_empty())
          .map(|(id, _)| id)
          .collect::<Vec<&String>>()
      );
    }
  }

  if commits_without_patches.is_empty() && stashes_or_merges_without_patches.is_empty() {
    // All patches were loaded from cache.
    return Ok(new_patches);
  }

  if !commits_without_patches.is_empty() {
    if let Ok(patches) =
      load_normal_patches(repo_path, &commits_without_patches, commits.len() as u32)
    {
      new_patches.extend(patches);
    }
  }

  for c in stashes_or_merges_without_patches.into_iter() {
    if let Ok((id, patches)) = load_patches_for_commit(repo_path, c) {
      new_patches.insert(id, patches);
    } else {
      // TODO: Some commits have no patches. We should probably save it anyway?
      new_patches.insert(c.id.clone(), Vec::new());
      // Maybe not if we aren't sure our method is correct?
      dprintln!("Failed to get patches for commit {}", c.id);
    }
  }

  write_patches_cache(repo_path, &new_patches);

  Ok(new_patches)
}

fn load_normal_patches(
  repo_path: &str,
  commits_without_patches: &Vec<&Commit>,
  num_commits: u32,
) -> R<HashMap<String, Vec<Patch>>> {
  if commits_without_patches.len() > 20 {
    // Assume we now have all the plain commits.
    load_all_patches_for_normal_commits(repo_path, num_commits)
  } else {
    // We can't handle many commit ids with this command.
    let mut ids: Vec<&str> = commits_without_patches
      .iter()
      .map(|c| c.id.as_str())
      .collect();

    ids.insert(0, "show");
    ids.extend(["--name-status", "--pretty=format:%H,", "-z"]);

    let out = run_git_err(RunGitOptions {
      repo_path,
      args: ids,
    })?
    .stdout;

    let commit_patches = parse_all_err(P_MANY_PATCHES_WITH_COMMIT_IDS, &out)?;

    Ok(commit_patches.into_iter().collect())
  }
}

/// This doesn't include stashes and merges.
fn load_all_patches_for_normal_commits(
  repo_path: &str,
  num_commits: u32,
) -> R<HashMap<String, Vec<Patch>>> {
  let out = run_git_err(RunGitOptions {
    args: [
      "log",
      "--remotes",
      "--branches",
      "--all",
      "--name-status",
      "--pretty=format:%H,",
      // Can't get correct patch info for merges with this command.
      "--no-merges",
      "-z",
      &format!("-n{}", num_commits),
    ],
    repo_path,
  })?
  .stdout;

  let commit_patches = parse_all_err(P_MANY_PATCHES_WITH_COMMIT_IDS, &out)?;

  Ok(commit_patches.into_iter().collect())
}

// without cache
fn load_patches_for_commit(repo_path: &str, commit: &Commit) -> R<(String, Vec<Patch>)> {
  let diff = String::from("diff");
  let name_status = String::from("--name-status");
  let no_colour = f!("--no-color");
  let z = String::from("-z");

  let out = match commit {
    Commit {
      is_merge: true,
      parent_ids,
      ..
    } => run_git_err(RunGitOptions {
      repo_path,
      // args: [diff, name_status, z, format!("{}^1", id), id.to_string()],
      args: [
        diff,
        name_status,
        z,
        no_colour,
        format!("{}...{}", parent_ids[0], parent_ids[1]),
      ],
    }),
    Commit {
      stash_id: Some(_),
      parent_ids,
      id,
      ..
    } => run_git_err(RunGitOptions {
      repo_path,
      args: [
        diff,
        format!("{}..{}", parent_ids[0], id),
        no_colour,
        name_status,
        z,
      ],
    }),
    Commit { id, .. } => run_git_err(RunGitOptions {
      repo_path,
      args: [
        diff,
        format!("{}..{}", COMMIT_0_ID, id),
        no_colour,
        name_status,
        z,
      ],
    }),
  };

  let patch_data = parse_all_err(P_PATCHES, &out?.stdout)?;

  Ok((
    commit.id.clone(),
    patch_data
      .into_iter()
      .map(|data| map_data_to_patch(data, commit.id.clone()))
      .collect(),
  ))
}

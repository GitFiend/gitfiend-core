use crate::git::git_types::{CommitInfo, GitConfig, RefInfo, RefLocation, RefType};
use crate::git::store::{RepoPath, CONFIG};
use crate::parser::standard_parsers::WS;
use crate::parser::Parser;
use crate::{and, character, map, rep_parser_sep, rep_sep, take_char_while, word};
use crate::{map2, or};
use loggers::elapsed;

pub(crate) mod head_info;
pub(crate) mod ref_diffs;

const REF_NAME_PARSER: Parser<String> =
  take_char_while!(|c: char| { !c.is_whitespace() && c != ',' && c != '(' && c != ')' });

pub const P_REF_NAME: Parser<RefInfoPart> = map!(REF_NAME_PARSER, |result: String| {
  let cleaned = result.replace("^{}", "");
  let parts: Vec<&str> = cleaned.split('/').collect();
  let ref_type = get_type_from_name(&parts);

  RefInfoPart {
    id: cleaned.to_owned(),
    ref_type,
    location: get_ref_location(&parts),
    short_name: get_short_name(&parts),
    full_name: cleaned.to_owned(),
    remote_name: get_remote_name(&parts),
    sibling_id: None,
    head: false,
  }
});

const P_TAG_REF: Parser<RefInfoPart> = map2!(and!(word!("tag: "), P_REF_NAME), result, result.1);

const P_HEAD_REF: Parser<RefInfoPart> = map!(
  and!(word!("HEAD -> "), P_REF_NAME),
  |mut result: (&str, RefInfoPart,)| {
    result.1.head = true;
    result.1
  }
);

const P_COMMIT_REF: Parser<RefInfoPart> = or!(P_TAG_REF, P_HEAD_REF, P_REF_NAME);

const P_COMMIT_REFS: Parser<Vec<RefInfoPart>> = map!(
  and!(
    character!('('),
    rep_sep!(P_COMMIT_REF, ","),
    character!(')')
  ),
  |result: (char, Vec<RefInfoPart>, char)| { result.1 }
);

pub const P_OPTIONAL_REFS: Parser<Vec<RefInfoPart>> =
  or!(P_COMMIT_REFS, map!(WS, |_: String| { Vec::new() }));

fn get_type_from_name(parts: &[&str]) -> RefType {
  if parts.len() > 1 {
    match parts[1] {
      "tags" => RefType::Tag,
      "stash" => RefType::Stash,
      _ => RefType::Branch,
    }
  } else {
    RefType::Branch
  }
}

fn get_ref_location(parts: &[&str]) -> RefLocation {
  if parts.len() >= 3 {
    if parts[1] == "heads" {
      return RefLocation::Local;
    }
    return RefLocation::Remote;
  }
  RefLocation::Local
}

fn get_short_name(parts: &[&str]) -> String {
  if parts.len() == 1 {
    parts[0].to_string()
  } else if parts[1] == "remotes" {
    parts[3..].join("/")
  } else {
    parts[2..].join("/")
  }
}

fn get_remote_name(parts: &[&str]) -> Option<String> {
  if parts.len() > 3 && parts[1] == "remotes" {
    Some(parts[2].to_string())
  } else {
    None
  }
}

pub fn make_ref_info(info: RefInfoPart, commit_id: String, time: f32) -> RefInfo {
  let RefInfoPart {
    id,
    location,
    full_name,
    short_name,
    remote_name,
    sibling_id,
    ref_type,
    head,
  } = info;

  RefInfo {
    id,
    location,
    full_name,
    short_name,
    remote_name,
    sibling_id,
    ref_type,
    head,
    commit_id,
    time,
  }
}

#[elapsed]
pub fn get_ref_info_from_commits(commits: &[CommitInfo]) -> Vec<RefInfo> {
  let mut refs: Vec<RefInfo> = Vec::new();

  for c in commits.iter() {
    for r in c.refs.iter() {
      if !r.full_name.contains("HEAD") {
        refs.push(r.clone())
      }
    }
  }

  refs
}

// pub fn finish_initialising_refs_on_commits(
//   commits: Vec<CommitInfo>,
//   repo_path: &RepoPath,
// ) -> Vec<CommitInfo> {
//   let refs = get_ref_info_from_commits(&commits);
//
//   set_sibling_and_remotes_for_commits(commits, &refs, repo_path)
// }

// Sets siblings and remotes and returns new refs.
pub fn finish_properties_on_refs(refs: Vec<RefInfo>, repo_path: &RepoPath) -> Vec<RefInfo> {
  let config = CONFIG.get_by_key(repo_path).unwrap_or_else(GitConfig::new);

  refs
    .iter()
    .map(|r| {
      let mut i = r.clone();

      if i.remote_name.is_none() {
        i.remote_name = Some(config.get_remote_for_branch(&i.short_name));
      }

      i.sibling_id = get_sibling_id_for_ref(&i, &refs);
      i
    })
    .collect()
}

// fn set_sibling_and_remotes_for_commits(
//   commits: Vec<CommitInfo>,
//   refs: &[RefInfo],
//   repo_path: &RepoPath,
// ) -> Vec<CommitInfo> {
//   let config = CONFIG.get_by_key(repo_path).unwrap_or_else(GitConfig::new);
//
//   commits
//     .into_iter()
//     .map(|mut c| {
//       c.refs = c
//         .refs
//         .into_iter()
//         .map(|mut r| {
//           if r.remote_name.is_none() {
//             r.remote_name = Some(config.get_remote_for_branch(&r.short_name));
//           }
//
//           r.sibling_id = get_sibling_id_for_ref(&r, refs);
//           r
//         })
//         .collect();
//       c
//     })
//     .collect()
// }

fn get_sibling_id_for_ref(ri: &RefInfo, refs: &[RefInfo]) -> Option<String> {
  if ri.location == RefLocation::Remote {
    if let Some(local) = refs
      .iter()
      .find(|i| i.location == RefLocation::Local && i.short_name == ri.short_name)
    {
      return Some(local.id.clone());
    }
  } else if let Some(remote) = refs.iter().find(|i| {
    i.location == RefLocation::Remote
      && i.short_name == ri.short_name
      && i.remote_name == ri.remote_name
  }) {
    return Some(remote.id.clone());
  }

  None
}

pub struct RefInfoPart {
  pub id: String,
  pub location: RefLocation,
  pub full_name: String,
  pub short_name: String,
  pub remote_name: Option<String>,
  pub sibling_id: Option<String>,
  pub ref_type: RefType,
  pub head: bool,
}

#[cfg(test)]
mod tests {
  use crate::git::git_types::RefLocation::Local;
  use crate::git::git_types::RefType;
  use crate::git::queries::refs::{
    get_ref_location, get_remote_name, get_short_name, get_type_from_name, P_COMMIT_REFS,
    P_HEAD_REF, P_OPTIONAL_REFS, P_REF_NAME, P_TAG_REF,
  };
  use crate::parser::parse_all;

  #[test]
  fn test_get_ref_location() {
    let loc = get_ref_location(&["refs", "heads", "commit-list-experiments"]);

    assert_eq!(loc, Local);
  }

  #[test]
  fn test_get_ref_short_name() {
    assert_eq!(
      get_short_name(&["refs", "heads", "feature", "dialogs"]),
      "feature/dialogs"
    );
    assert_eq!(
      get_short_name(&["refs", "remotes", "origin", "git-lib"]),
      "git-lib"
    );
    assert_eq!(get_short_name(&["HEAD"]), "HEAD");
  }

  #[test]
  fn test_get_remote_name() {
    assert_eq!(
      get_remote_name(&["refs", "remotes", "origin", "git-lib"]),
      Some("origin".to_string())
    );
    assert_eq!(
      get_remote_name(&["refs", "heads", "feature", "dialogs"]),
      None
    );
    assert_eq!(get_remote_name(&["refs", "tags", "hello"]), None);
  }

  #[test]
  fn test_p_ref_name() {
    let res = parse_all(P_REF_NAME, "refs/heads/git-lib");

    assert!(res.is_some());
  }

  #[test]
  fn test_p_tag_ref() {
    let result = parse_all(P_TAG_REF, "tag: refs/tags/v0.11.2");
    assert!(result.is_some());
  }

  #[test]
  fn test_p_head_ref() {
    let result = parse_all(P_HEAD_REF, "HEAD -> refs/heads/master");

    assert!(result.is_some());
    assert_eq!(result.unwrap().id, "refs/heads/master");
  }

  #[test]
  fn test_p_commit_refs() {
    let a = "(HEAD -> refs/heads/master, refs/remotes/origin/master, refs/remotes/origin/HEAD)";
    let result = parse_all(P_COMMIT_REFS, a);

    assert!(result.is_some());
    assert_eq!(result.as_ref().unwrap().len(), 3);
    assert_eq!(result.as_ref().unwrap()[1].id, "refs/remotes/origin/master");
  }

  #[test]
  fn test_p_optional_refs() {
    let a = "(HEAD -> refs/heads/master, refs/remotes/origin/master)";
    let result = parse_all(P_OPTIONAL_REFS, a);

    assert!(result.is_some());

    let refs = result.unwrap();

    assert_eq!(refs.len(), 2);
    assert_eq!(refs[0].id, "refs/heads/master");

    let b = "(HEAD -> refs/heads/master, refs/remotes/origin/master, refs/remotes/origin/HEAD)";
    let result = parse_all(P_OPTIONAL_REFS, b);

    assert!(result.is_some());

    let refs = result.unwrap();

    assert_eq!(refs.len(), 3);
    assert_eq!(refs[1].id, "refs/remotes/origin/master");
  }

  #[test]
  fn test_get_type_from_name() {
    assert_eq!(
      get_type_from_name(&["refs", "remotes", "origin", "git-lib"]),
      RefType::Branch
    );
    assert_eq!(get_type_from_name(&["refs", "tags", "hello"]), RefType::Tag);
    assert_eq!(get_type_from_name(&["HEAD"]), RefType::Branch);
  }
}

use crate::git::git_types::{RefInfo, RefInfoPart, RefLocation, RefType};
use crate::parser::standard_parsers::WS;
use crate::parser::Parser;
use crate::{and, character, map, rep_parser_sep, rep_sep, take_char_while, word};
use crate::{or, Input};

const REF_NAME_PARSER: Parser<String> =
  take_char_while!(|c: char| { !c.is_whitespace() && c != ',' && c != '(' && c != ')' });

const P_REF_NAME: Parser<RefInfoPart> = map!(REF_NAME_PARSER, |result: String| {
  let cleaned = result.replace("^{}", "");
  let parts: Vec<&str> = cleaned.split("/").collect();
  let ref_type = get_type_from_name(parts[1]);

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

const P_TAG_REF: Parser<RefInfoPart> = map!(and!(word!("tag: "), P_REF_NAME), |result: (
  &str,
  RefInfoPart,
)| { result.1 });

const P_HEAD_REF: Parser<RefInfoPart> = map!(and!(word!("HEAD -> "), P_REF_NAME), |result: (
  &str,
  RefInfoPart,
)| { result.1 });

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

fn get_type_from_name(part: &str) -> RefType {
  match part {
    "tags" => RefType::Tag,
    "stash" => RefType::Stash,
    _ => RefType::Branch,
  }
}

fn get_ref_location(parts: &Vec<&str>) -> RefLocation {
  if parts.len() >= 3 {
    if parts[1] == "heads" {
      return RefLocation::Local;
    }
    return RefLocation::Remote;
  }
  RefLocation::Local
}

fn get_short_name(parts: &Vec<&str>) -> String {
  if parts[1] == "remotes" {
    parts[3..].join("/")
  } else {
    parts[2..].join("/")
  }
}

fn get_remote_name(parts: &Vec<&str>) -> Option<String> {
  if parts.len() > 3 && parts[1] == "remotes" {
    Some(parts[2].to_string())
  } else {
    None
  }
}

pub fn make_ref_info(info: RefInfoPart, commit_id: String, time: f32) -> RefInfo {
  match info {
    RefInfoPart {
      id,
      location,
      full_name,
      short_name,
      remote_name,
      sibling_id,
      ref_type,
      head,
    } => RefInfo {
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
    },
  }
}

#[cfg(test)]
mod tests {
  use crate::git::git_types::RefLocation::Local;
  use crate::git::queries::refs::{
    get_ref_location, get_remote_name, get_short_name, P_COMMIT_REFS, P_HEAD_REF, P_OPTIONAL_REFS,
    P_REF_NAME, P_TAG_REF,
  };
  use crate::parser::parse_all;

  #[test]
  fn test_get_ref_location() {
    let loc = get_ref_location(&vec!["refs", "heads", "commit-list-experiments"]);

    assert_eq!(loc, Local);
  }

  #[test]
  fn test_get_ref_short_name() {
    let name = get_short_name(&vec!["refs", "heads", "feature", "dialogs"]);

    assert_eq!(name, "feature/dialogs");

    let name = get_short_name(&vec!["refs", "remotes", "origin", "git-lib"]);

    assert_eq!(name, "git-lib");
  }

  #[test]
  fn test_get_remote_name() {
    assert_eq!(
      get_remote_name(&vec!["refs", "remotes", "origin", "git-lib"]),
      Some("origin".to_string())
    );
    assert_eq!(
      get_remote_name(&vec!["refs", "heads", "feature", "dialogs"]),
      None
    );
    assert_eq!(get_remote_name(&vec!["refs", "tags", "hello"]), None);
  }

  #[test]
  fn test_p_ref_name() {
    let res = parse_all(P_REF_NAME, "refs/heads/git-lib");

    assert_eq!(res.is_some(), true);
  }

  #[test]
  fn test_p_tag_ref() {
    let result = parse_all(P_TAG_REF, "tag: refs/tags/v0.11.2");
    assert_eq!(result.is_some(), true);
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
}

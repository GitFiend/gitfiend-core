use crate::git::git_types::{RefInfoPart, RefLocation, RefType};
use crate::parser::Parser;
use crate::Input;
use crate::{map, take_char_while};

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

#[cfg(test)]
mod tests {
  use crate::git::git_types::RefLocation::Local;
  use crate::git::queries::refs::{get_ref_location, get_remote_name, get_short_name, P_REF_NAME};
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
}

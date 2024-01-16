use crate::git::git_types::{Patch, PatchType};
use crate::git::queries::patches;
use crate::many;
use crate::parser::standard_parsers::{UNSIGNED_INT, UNTIL_NUL, WS};
use crate::parser::Parser;
use crate::{and, map, until_parser};
use crate::{character, or};

#[derive(Debug, PartialEq)]
pub struct PatchData {
  old_file: String,
  new_file: String,
  patch_type: PatchType,
  id: String,
}

pub const P_PATCHES_WITH_COMMIT_ID: Parser<(String, Vec<Patch>)> = map!(
  and!(
    until_parser!(and!(character!(','), WS)),
    P_PATCHES,
    or!(UNTIL_NUL, WS)
  ),
  |result: (String, Vec<PatchData>, String)| {
    let (commit_id, patches, ..) = result;

    (
      commit_id.clone(),
      patches
        .into_iter()
        .map(|data| map_data_to_patch(data, commit_id.clone()))
        .collect(),
    )
  }
);

pub const P_MANY_PATCHES_WITH_COMMIT_IDS: Parser<Vec<(String, Vec<Patch>)>> =
  many!(P_PATCHES_WITH_COMMIT_ID);

pub fn map_data_to_patch(data: PatchData, commit_id: String) -> Patch {
  let PatchData {
    patch_type,
    old_file,
    new_file,
    id,
  } = data;

  let is_image = patches::file_is_image(&new_file);

  Patch {
    patch_type,
    old_file,
    new_file,
    id,
    commit_id,
    is_image,
  }
}

pub const P_PATCHES: Parser<Vec<PatchData>> =
  many!(or!(P_RENAME_PATCH, P_COPY_PATCH, P_OTHER_PATCH));

const P_RENAME_PATCH: Parser<PatchData> = map!(
  and!(
    and!(character!('R'), UNSIGNED_INT),
    UNTIL_NUL,
    UNTIL_NUL,
    UNTIL_NUL
  ),
  |result: ((char, String), String, String, String)| {
    PatchData {
      patch_type: PatchType::R,
      old_file: result.2.clone(),
      new_file: result.3.clone(),
      id: format!("{}-{}{}", result.3, result.0 .0, result.0 .1),
    }
  }
);

const P_COPY_PATCH: Parser<PatchData> = map!(
  and!(
    and!(character!('C'), UNSIGNED_INT),
    UNTIL_NUL,
    UNTIL_NUL,
    UNTIL_NUL
  ),
  |result: ((char, String), String, String, String)| {
    PatchData {
      patch_type: PatchType::C,
      old_file: result.2.clone(),
      new_file: result.3.clone(),
      id: format!("{}-{}{}", result.3, result.0 .0, result.0 .1),
    }
  }
);

const P_OTHER_PATCH: Parser<PatchData> = map!(and!(P_STATUS, UNTIL_NUL, UNTIL_NUL), |result: (
  PatchType,
  String,
  String
)| {
  let (t, _, n) = result;
  let type_str = t.to_string();

  PatchData {
    patch_type: t,
    old_file: n.clone(),
    new_file: n.clone(),
    id: format!("{}-{}", n, type_str),
  }
});

const P_STATUS: Parser<PatchType> = map!(
  or!(
    character!('A'),
    character!('B'),
    character!('C'),
    character!('D'),
    character!('M'),
    character!('T'),
    character!('U'),
    character!('X')
  ),
  |result: char| {
    match result {
      'A' => PatchType::A,
      'B' => PatchType::B,
      'C' => PatchType::C,
      'D' => PatchType::D,
      'M' => PatchType::M,
      'T' => PatchType::T,
      'U' => PatchType::U,
      _ => PatchType::X,
    }
  }
);

#[cfg(test)]
mod tests {
  use crate::git::git_types::PatchType;
  use crate::git::queries::patches::patch_parsers::{
    PatchData, P_COPY_PATCH, P_OTHER_PATCH, P_RENAME_PATCH,
  };
  use crate::parser::parse_all;

  const P1: &str = "src2/parser-lib/input.ts";
  const P2: &str = "src2/renderer-process/redux-store/repo-state/commits/commits-reducer.test.ts";
  // const P3: &str = "\"src2/Parser Lib/input.ts\"";

  #[test]
  fn test_p_rename_patch() {
    let log = &format!("R100\0{}\0{}\0", P1, P2);
    let res = parse_all(P_RENAME_PATCH, log);

    assert!(res.is_some());
    assert_eq!(
      res.unwrap(),
      PatchData {
        patch_type: PatchType::R,
        old_file: P1.to_string(),
        new_file: P2.to_string(),
        id: format!("{P2}-R100")
      }
    )
  }

  #[test]
  fn test_p_copy_patch() {
    let log = &format!("C100\0{}\0{}\0", P1, P2);
    let res = parse_all(P_COPY_PATCH, log);

    assert!(res.is_some());
    assert_eq!(
      res.unwrap(),
      PatchData {
        patch_type: PatchType::C,
        old_file: P1.to_string(),
        new_file: P2.to_string(),
        id: format!("{P2}-C100")
      }
    )
  }

  #[test]
  fn test_p_other_patch() {
    let log = &format!("M\0{P2}\0");
    let res = parse_all(P_OTHER_PATCH, log);

    assert!(res.is_some());
    assert_eq!(
      res.unwrap(),
      PatchData {
        patch_type: PatchType::M,
        old_file: P2.to_string(),
        new_file: P2.to_string(),
        id: format!("{P2}-M")
      }
    )
  }
}

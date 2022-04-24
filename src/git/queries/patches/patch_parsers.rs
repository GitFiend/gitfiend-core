use crate::git::git_types::PatchType;
use crate::parser::standard_parsers::{UNSIGNED_INT, UNTIL_NUL};
use crate::parser::Parser;
use crate::Input;
use crate::{and, map};
use crate::{character, or};

#[derive(Debug, PartialEq)]
struct PatchData {
  old_file: String,
  new_file: String,
  patch_type: PatchType,
  id: String,
}

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
  use crate::git::queries::patches::patch_parsers::{PatchData, P_COPY_PATCH, P_RENAME_PATCH};
  use crate::parser::parse_all;

  const P1: &str = "src2/parser-lib/input.ts";
  const P2: &str = "src2/renderer-process/redux-store/repo-state/commits/commits-reducer.test.ts";
  const P3: &str = "\"src2/Parser Lib/input.ts\"";

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
}

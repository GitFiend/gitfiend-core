use crate::git::git_types::WipPatchType;
use crate::git::queries::wip::wip_patches::WipPatchInfo;
use crate::parser::standard_parsers::{UNTIL_NUL, WS};
use crate::parser::Parser;
use crate::{and, character, map, or};

pub const P_WORK_STATUS_PART: Parser<char> = or!(
  character!(' '),
  character!('?'),
  character!('A'),
  character!('C'),
  character!('D'),
  character!('M'),
  character!('R'),
  character!('U')
);

/*
[ D]        R    renamed in work tree
 R        [ MD]   renamed in index
 */
pub const P_RENAME_STATUS: Parser<(char, char)> = or!(
  and!(character!('R'), P_WORK_STATUS_PART),
  and!(P_WORK_STATUS_PART, character!('R'))
);

impl WipPatchType {
  fn from_char(c: char) -> WipPatchType {
    match c {
      '?' => WipPatchType::Question,
      ' ' => WipPatchType::Empty,
      'A' => WipPatchType::A,
      'C' => WipPatchType::C,
      'D' => WipPatchType::D,
      'M' => WipPatchType::M,
      'R' => WipPatchType::R,
      'U' => WipPatchType::U,
      _ => WipPatchType::Empty,
    }
  }
}

pub const P_WIP_RENAME_PATCH: Parser<WipPatchInfo> = map!(
  and!(P_RENAME_STATUS, WS, UNTIL_NUL, WS, UNTIL_NUL),
  |res: ((char, char), String, String, String, String)| {
    WipPatchInfo {
      staged: WipPatchType::from_char(res.0 .0),
      un_staged: WipPatchType::from_char(res.0 .1),
      old_file: res.4,
      new_file: res.2,
    }
  }
);

#[cfg(test)]
mod tests {
  use crate::git::git_types::WipPatchType;
  use crate::git::queries::wip::wip_patch_parsers::{P_RENAME_STATUS, P_WIP_RENAME_PATCH};
  use crate::git::queries::wip::wip_patches::WipPatchInfo;
  use crate::parser::parse_all;

  #[test]
  fn test_p_rename_status() {
    let out = parse_all(P_RENAME_STATUS, "R ");
    assert!(out.is_some());
    assert_eq!(out.unwrap(), ('R', ' '));

    let out = parse_all(P_RENAME_STATUS, " R");
    assert!(out.is_some());
    assert_eq!(out.unwrap(), (' ', 'R'));

    let out = parse_all(P_RENAME_STATUS, "RM");
    assert!(out.is_some());
    assert_eq!(out.unwrap(), ('R', 'M'));

    let out = parse_all(P_RENAME_STATUS, "DR");
    assert!(out.is_some());
    assert_eq!(out.unwrap(), ('D', 'R'));
  }

  #[test]
  fn test_p_wip_rename_patch() {
    let out = parse_all(P_WIP_RENAME_PATCH, "R  filename.txt\0has some spaces.txt\0");

    assert!(out.is_some());
    assert_eq!(
      out.unwrap(),
      WipPatchInfo {
        staged: WipPatchType::R,
        un_staged: WipPatchType::Empty,
        old_file: String::from("has some spaces.txt"),
        new_file: String::from("filename.txt")
      }
    );
  }
}

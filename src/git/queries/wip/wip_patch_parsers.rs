use crate::git::git_types::WipPatchType;
use crate::git::queries::wip::wip_patches::WipPatchInfo;
use crate::parser::standard_parsers::{UNTIL_NUL, WS};
use crate::parser::Parser;
use crate::{and, character, many, map, or};
use bstr::BString;

const P_WORK_STATUS_PART: Parser<char> = or!(
  character!(' '),
  character!('?'),
  character!('A'),
  character!('C'),
  character!('D'),
  character!('M'),
  character!('R'),
  character!('U'),
  character!('T')
);

/*
[ D]        R    renamed in work tree
 R        [ MD]   renamed in index
 */
const P_RENAME_STATUS: Parser<(char, char)> = or!(
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
      'T' => WipPatchType::T,
      _ => WipPatchType::Empty,
    }
  }
}

const P_WIP_RENAME_PATCH: Parser<WipPatchInfo> = map!(
  and!(P_RENAME_STATUS, WS, UNTIL_NUL, WS, UNTIL_NUL),
  |res: ((char, char), BString, BString, BString, BString)| {
    WipPatchInfo {
      staged: WipPatchType::from_char(res.0 .0),
      un_staged: WipPatchType::from_char(res.0 .1),
      old_file: res.4.to_string(),
      new_file: res.2.to_string(),
    }
  }
);

const P_WORK_STATUS: Parser<(char, char)> = and!(P_WORK_STATUS_PART, P_WORK_STATUS_PART);

const P_WIP_OTHER_PATCH: Parser<WipPatchInfo> = map!(
  and!(P_WORK_STATUS, WS, UNTIL_NUL),
  |res: ((char, char), BString, BString)| {
    WipPatchInfo {
      staged: WipPatchType::from_char(res.0 .0),
      un_staged: WipPatchType::from_char(res.0 .1),
      old_file: res.2.to_string(),
      new_file: res.2.to_string(),
    }
  }
);

const P_COPY_STATUS: Parser<(char, char)> = or!(
  and!(character!('C'), P_WORK_STATUS_PART),
  and!(P_WORK_STATUS_PART, character!('C'))
);

const P_WIP_COPY_PATCH: Parser<WipPatchInfo> = map!(
  and!(P_COPY_STATUS, WS, UNTIL_NUL, WS, UNTIL_NUL),
  |res: ((char, char), BString, BString, BString, BString)| {
    WipPatchInfo {
      staged: WipPatchType::from_char(res.0 .0),
      un_staged: WipPatchType::from_char(res.0 .1),
      old_file: res.4.to_string(),
      new_file: res.2.to_string(),
    }
  }
);

pub const P_WIP_PATCHES: Parser<Vec<WipPatchInfo>> =
  many!(or!(P_WIP_RENAME_PATCH, P_WIP_COPY_PATCH, P_WIP_OTHER_PATCH));

#[cfg(test)]
mod tests {
  use crate::git::git_types::WipPatchType;
  use crate::git::queries::wip::wip_patch_parsers::{
    P_RENAME_STATUS, P_WIP_OTHER_PATCH, P_WIP_PATCHES, P_WIP_RENAME_PATCH, P_WORK_STATUS,
  };
  use crate::git::queries::wip::wip_patches::WipPatchInfo;
  use crate::parser::parse_all;

  #[test]
  fn test_p_rename_status() {
    let out = parse_all(P_RENAME_STATUS, b"R ");
    assert!(out.is_some());
    assert_eq!(out.unwrap(), ('R', ' '));

    let out = parse_all(P_RENAME_STATUS, b" R");
    assert!(out.is_some());
    assert_eq!(out.unwrap(), (' ', 'R'));

    let out = parse_all(P_RENAME_STATUS, b"RM");
    assert!(out.is_some());
    assert_eq!(out.unwrap(), ('R', 'M'));

    let out = parse_all(P_RENAME_STATUS, b"DR");
    assert!(out.is_some());
    assert_eq!(out.unwrap(), ('D', 'R'));
  }

  #[test]
  fn test_p_wip_rename_patch() {
    let out = parse_all(
      P_WIP_RENAME_PATCH,
      b"R  filename.txt\0has some spaces.txt\0",
    );

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

  #[test]
  fn test_p_work_status() {
    let out = parse_all(P_WORK_STATUS, b"??");
    assert_eq!(out.unwrap(), ('?', '?'));

    let out = parse_all(P_WORK_STATUS, b" A");
    assert_eq!(out.unwrap(), (' ', 'A'));

    let out = parse_all(P_WORK_STATUS, b"AM");
    assert_eq!(out.unwrap(), ('A', 'M'));
  }

  #[test]
  fn test_p_wip_other_patch() {
    let out = parse_all(P_WIP_OTHER_PATCH, b"DU folder/has a space/test2.js\0");
    assert!(out.is_some());
    assert_eq!(
      out.unwrap(),
      WipPatchInfo {
        staged: WipPatchType::D,
        un_staged: WipPatchType::U,
        old_file: String::from("folder/has a space/test2.js"),
        new_file: String::from("folder/has a space/test2.js")
      }
    )
  }

  #[test]
  fn test_p_wip_patches() {
    let out = parse_all(P_WIP_PATCHES, b"R  582160ee-5216-4dc6-bf74-1c1fce4978eb2.txt\0 582160ee-5216-4dc6-bf74-1c1fce4978eb.txt\0DU folder/has a space/test2.js\0");
    assert!(out.is_some());

    assert_eq!(
      out.unwrap(),
      [
        WipPatchInfo {
          staged: WipPatchType::R,
          un_staged: WipPatchType::Empty,
          old_file: String::from("582160ee-5216-4dc6-bf74-1c1fce4978eb.txt"),
          new_file: String::from("582160ee-5216-4dc6-bf74-1c1fce4978eb2.txt")
        },
        WipPatchInfo {
          staged: WipPatchType::D,
          un_staged: WipPatchType::U,
          old_file: String::from("folder/has a space/test2.js"),
          new_file: String::from("folder/has a space/test2.js")
        }
      ]
    );

    let out = parse_all(P_WIP_PATCHES, b"C  582160ee-5216-4dc6-bf74-1c1fce4978eb2.txt\0 582160ee-5216-4dc6-bf74-1c1fce4978eb.txt\0DU folder/has a space/test2.js\0");
    assert!(out.is_some());

    assert_eq!(
      out.unwrap(),
      [
        WipPatchInfo {
          staged: WipPatchType::C,
          un_staged: WipPatchType::Empty,
          old_file: String::from("582160ee-5216-4dc6-bf74-1c1fce4978eb.txt"),
          new_file: String::from("582160ee-5216-4dc6-bf74-1c1fce4978eb2.txt")
        },
        WipPatchInfo {
          staged: WipPatchType::D,
          un_staged: WipPatchType::U,
          old_file: String::from("folder/has a space/test2.js"),
          new_file: String::from("folder/has a space/test2.js")
        }
      ]
    );
  }

  #[test]
  fn test_p_wip_patches2() {
    let out = parse_all(
      P_WIP_PATCHES,
      b"T  582160ee-5216-4dc6-bf74-1c1fce4978eb2.txt\0DU folder/has a space/test2.js\0",
    );
    assert!(out.is_some());

    assert_eq!(
      out.unwrap(),
      [
        WipPatchInfo {
          staged: WipPatchType::T,
          un_staged: WipPatchType::Empty,
          old_file: String::from("582160ee-5216-4dc6-bf74-1c1fce4978eb2.txt"),
          new_file: String::from("582160ee-5216-4dc6-bf74-1c1fce4978eb2.txt")
        },
        WipPatchInfo {
          staged: WipPatchType::D,
          un_staged: WipPatchType::U,
          old_file: String::from("folder/has a space/test2.js"),
          new_file: String::from("folder/has a space/test2.js")
        }
      ]
    );
  }

  #[test]
  fn test_p_wip_patches3() {
    let text = b" M .DS_Store\0 D LabBook/.ztr-directory\0 M LabBook/2023-06-18_CRISPR23-code.md\0?? Icon\r\0?? LabBook/2023-06-26_TEST.md\0";

    let out = parse_all(P_WIP_PATCHES, text);
    assert!(out.is_some());

    use WipPatchType::*;

    assert_eq!(
      out.unwrap(),
      [
        WipPatchInfo {
          staged: Empty,
          un_staged: M,
          old_file: String::from(".DS_Store"),
          new_file: String::from(".DS_Store")
        },
        WipPatchInfo {
          staged: Empty,
          un_staged: D,
          old_file: String::from("LabBook/.ztr-directory"),
          new_file: String::from("LabBook/.ztr-directory")
        },
        WipPatchInfo {
          staged: Empty,
          un_staged: M,
          old_file: String::from("LabBook/2023-06-18_CRISPR23-code.md"),
          new_file: String::from("LabBook/2023-06-18_CRISPR23-code.md")
        },
        WipPatchInfo {
          staged: Question,
          un_staged: Question,
          old_file: String::from("Icon\r"),
          new_file: String::from("Icon\r")
        },
        WipPatchInfo {
          staged: Question,
          un_staged: Question,
          old_file: String::from("LabBook/2023-06-26_TEST.md"),
          new_file: String::from("LabBook/2023-06-26_TEST.md")
        },
      ]
    );
  }
}

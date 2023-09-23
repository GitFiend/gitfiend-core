use std::cmp::Ordering;
use std::fs;
use std::path::Path;

use serde::Deserialize;
use ts_rs::TS;

use crate::git::conflicts::conflicted_file::{
  BlankLine, CFLine, CFSection, CFSectionLine, CFSide, ConflictLine, ConflictedFile, OkLine,
  SlotLine,
};
use crate::git::git_types::WipPatch;
use crate::git::queries::refs::P_REF_NAME;
use crate::parser::parse_all;
use crate::server::request_util::R;

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct LoadConflictOptions {
  pub repo_path: String,
  pub patch: WipPatch,
}

pub fn load_conflicted_file(options: &LoadConflictOptions) -> R<ConflictedFile> {
  let LoadConflictOptions { repo_path, patch } = options;

  let path = Path::new(repo_path).join(&patch.new_file);
  let text = fs::read_to_string(path)?;

  let lines = text.lines().collect::<Vec<&str>>();

  Ok(conflicted_lines(lines))
}

const CONFLICT_START: &str = "<<<<<<<";
const CONFLICT_MIDDLE: &str = "=======";
const CONFLICT_END: &str = ">>>>>>>";

fn conflicted_lines(lines: Vec<&str>) -> ConflictedFile {
  let mut file = ConflictedFile::new();

  let mut in_section = false;
  let mut section: usize = 0;
  let mut side = CFSide::Left;

  for str_line in lines {
    if str_line.starts_with(CONFLICT_START) {
      in_section = true;
      side = CFSide::Left;

      if file.ref_name_top.is_empty() {
        file.ref_name_top = parse_ref_name(str_line);
      }
    } else if str_line.starts_with(CONFLICT_MIDDLE) {
      side = CFSide::Right;
    } else if str_line.starts_with(CONFLICT_END) {
      in_section = false;

      balance_section_with_blanks(&mut file, section);

      if file.ref_name_bottom.is_empty() {
        file.ref_name_bottom = parse_ref_name(str_line);
      }
      section += 1;
    } else {
      if str_line.len() > file.max_line_length {
        file.max_line_length = str_line.len();
      }

      if in_section {
        if file.sections.get(section).is_none() {
          file.sections.push(CFSection {
            left: vec![],
            right: vec![],
          });
        }

        let index_in_section = file.sections[section].get(&side).len();

        file.sections[section]
          .get_mut(&side)
          .push(CFSectionLine::Conflict(ConflictLine {
            text: str_line.to_string(),
            side: side.clone(),
            section,
            key: format!("{}-{}-{}", side, section, index_in_section),
          }));
      } else {
        file.lines.push(CFLine::Ok(OkLine {
          text: str_line.to_string(),
        }));
      }
    }
  }

  file
}

fn parse_ref_name(line: &str) -> String {
  if let Some(name) = line.split(' ').nth(1) {
    if name == "HEAD" {
      return name.to_string();
    }

    if let Some(ref_part) = parse_all(P_REF_NAME, name) {
      return format!("{}/{}", ref_part.location, ref_part.short_name);
    }

    return name.to_string();
  }

  String::new()
}

fn balance_section_with_blanks(file: &mut ConflictedFile, section: usize) {
  let CFSection { left, right } = &mut file.sections[section];

  match right.len().cmp(&left.len()) {
    Ordering::Greater => {
      let left_blanks = right.len() - left.len();

      for _ in 0..left_blanks {
        left.push(CFSectionLine::Blank(BlankLine { section }));
      }
    }
    Ordering::Less => {
      let right_blanks = left.len() - right.len();

      for _ in 0..right_blanks {
        right.push(CFSectionLine::Blank(BlankLine { section }));
      }
    }
    Ordering::Equal => {}
  };

  for i in 0..right.len() {
    file
      .lines
      .push(CFLine::Slot(SlotLine { section, index: i }));
  }
}

#[cfg(test)]
mod tests {
  use crate::git::conflicts::api::conflicted_lines;
  use crate::git::conflicts::conflicted_file::{
    CFLine, CFSection, CFSectionLine, CFSide, ConflictLine, ConflictedFile, OkLine, SlotLine,
  };

  #[test]
  fn test_conflicted_file() {
    let str_lines = vec![
      "<<<<<<< HEAD",
      "abc",
      "=======",
      "cba",
      ">>>>>>> refs/heads/B",
    ];

    let res = conflicted_lines(str_lines);

    let expected = ConflictedFile {
      lines: vec![CFLine::Slot(SlotLine {
        section: 0,
        index: 0,
      })],
      sections: vec![CFSection {
        left: vec![CFSectionLine::Conflict(ConflictLine {
          text: String::from("abc"),
          side: CFSide::Left,
          section: 0,
          key: String::from("Left-0-0"),
        })],
        right: vec![CFSectionLine::Conflict(ConflictLine {
          text: String::from("cba"),
          side: CFSide::Right,
          section: 0,
          key: String::from("Right-0-0"),
        })],
      }],
      ref_name_top: String::from("HEAD"),
      ref_name_bottom: String::from("Local/B"),
      line_ending: String::from("\n"),
      max_line_length: 3,
    };

    assert_eq!(res, expected);
  }

  #[test]
  fn test_conflicted_file2() {
    let str_lines = vec![
      "before",
      "<<<<<<< HEAD",
      "abc",
      "=======",
      "cba",
      ">>>>>>> refs/heads/B",
      "after",
    ];

    let res = conflicted_lines(str_lines);

    let expected = ConflictedFile {
      lines: vec![
        CFLine::Ok(OkLine {
          text: String::from("before"),
        }),
        CFLine::Slot(SlotLine {
          section: 0,
          index: 0,
        }),
        CFLine::Ok(OkLine {
          text: String::from("after"),
        }),
      ],
      sections: vec![CFSection {
        left: vec![CFSectionLine::Conflict(ConflictLine {
          text: String::from("abc"),
          side: CFSide::Left,
          section: 0,
          key: String::from("Left-0-0"),
        })],
        right: vec![CFSectionLine::Conflict(ConflictLine {
          text: String::from("cba"),
          side: CFSide::Right,
          section: 0,
          key: String::from("Right-0-0"),
        })],
      }],
      ref_name_top: String::from("HEAD"),
      ref_name_bottom: String::from("Local/B"),
      line_ending: String::from("\n"),
      max_line_length: 6,
    };

    assert_eq!(res, expected);
  }
}

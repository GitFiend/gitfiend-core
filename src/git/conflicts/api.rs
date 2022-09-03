use std::fs;
use std::path::Path;

use serde::Deserialize;
use ts_rs::TS;

use crate::git::conflicts::conflicted_file::{
  CFSection, CFSide, ConflictedFile, ConflictedFileLine, ConflictedLine,
};
use crate::git::git_types::WipPatch;
use crate::git::queries::refs::P_REF_NAME;
use crate::parser::parse_all;

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct LoadConflictOptions {
  pub repo_path: String,
  pub patch: WipPatch,
}

pub fn load_conflicted_file(options: &LoadConflictOptions) -> Option<ConflictedFile> {
  let LoadConflictOptions { repo_path, patch } = options;

  let path = Path::new(repo_path).join(&patch.new_file);
  let text = fs::read_to_string(path).ok()?;

  let lines = text.lines().collect::<Vec<&str>>();

  Some(conflicted_lines(lines))
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

        file.sections[section].get_mut(&side).push(ConflictedLine {
          text: str_line.to_string(),
          blank: false,
          side: side.clone(),
          section,
          conflicted: true,
          key: format!("{}-{}-{}", side, section, index_in_section),
        })
      } else {
        file.lines.push(ConflictedFileLine {
          text: Some(str_line.to_string()),
          section: None,
          index: file.lines.len(),
        })
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

  let left_blanks = right.len() - left.len();

  for index in 0..left_blanks {
    left.push(ConflictedLine {
      text: String::new(),
      blank: true,
      side: CFSide::Left,
      section,
      conflicted: false,
      key: format!("{}-{}-{}", CFSide::Left, section, index),
    });
  }

  let right_blanks = left.len() - right.len();

  for index in 0..right_blanks {
    right.push(ConflictedLine {
      text: String::new(),
      blank: true,
      side: CFSide::Right,
      section,
      conflicted: false,
      key: format!("{}-{}-{}", CFSide::Right, section, index),
    });
  }

  for i in 0..right.len() {
    file.lines.push(ConflictedFileLine {
      text: None,
      section: Some(section),
      index: i,
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::git::conflicts::api::conflicted_lines;
  use crate::git::conflicts::conflicted_file::{
    CFSection, CFSide, ConflictedFile, ConflictedFileLine, ConflictedLine,
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

    println!("{:?}", res);

    let expected = ConflictedFile {
      lines: vec![ConflictedFileLine {
        text: None,
        section: Some(0),
        index: 0,
      }],
      sections: vec![CFSection {
        left: vec![ConflictedLine {
          text: String::from("abc"),
          blank: false,
          side: CFSide::Left,
          section: 0,
          conflicted: true,
          key: String::from("Left-0-0"),
        }],
        right: vec![ConflictedLine {
          text: String::from("cba"),
          blank: false,
          side: CFSide::Right,
          section: 0,
          conflicted: true,
          key: String::from("Right-0-0"),
        }],
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

    println!("{:?}", res);

    let expected = ConflictedFile {
      lines: vec![
        ConflictedFileLine {
          text: Some(String::from("before")),
          section: None,
          index: 0,
        },
        ConflictedFileLine {
          text: None,
          section: Some(0),
          index: 0,
        },
        ConflictedFileLine {
          text: Some(String::from("after")),
          section: None,
          index: 2,
        },
      ],
      sections: vec![CFSection {
        left: vec![ConflictedLine {
          text: String::from("abc"),
          blank: false,
          side: CFSide::Left,
          section: 0,
          conflicted: true,
          key: String::from("Left-0-0"),
        }],
        right: vec![ConflictedLine {
          text: String::from("cba"),
          blank: false,
          side: CFSide::Right,
          section: 0,
          conflicted: true,
          key: String::from("Right-0-0"),
        }],
      }],
      ref_name_top: String::from("HEAD"),
      ref_name_bottom: String::from("Local/B"),
      line_ending: String::from("\n"),
      max_line_length: 6,
    };

    assert_eq!(res, expected);
  }
}

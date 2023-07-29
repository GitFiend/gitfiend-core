use crate::git::git_types::{Hunk, HunkLine, HunkLineStatus, HunkRange};
use std::cmp::{max, min};
use HunkLineStatus::{Added, Removed};

pub fn convert_lines_to_hunks(lines: Vec<HunkLine>) -> (Vec<Hunk>, u32) {
  let mut hunks = Vec::<Hunk>::new();
  let mut current_hunk = Hunk::new();
  let mut started_making_hunk = false;

  // Hunks should be joined if there's only 6 unchanged lines between them.
  let mut gap_count = 0;
  let mut patch_size: u32 = 0;

  for (i, line) in lines.iter().enumerate() {
    let HunkLine { status, .. } = line;

    if *status == Added || *status == Removed {
      patch_size += 1;
      gap_count = 0;

      if !started_making_hunk {
        started_making_hunk = true;

        let start_i = max(0, (i as i32) - 3) as usize;
        let slice = &lines[start_i..i];

        current_hunk.lines.extend_from_slice(slice);
      }
      current_hunk.lines.push(line.clone());
    } else if started_making_hunk {
      if gap_count < 6 {
        gap_count += 1;
        current_hunk.lines.push(line.clone());
      } else {
        if gap_count < 3 {
          current_hunk
            .lines
            .extend_from_slice(&lines[i..min(i + (3 - gap_count), lines.len())]);
        } else {
          for _ in 0..(gap_count - 3) {
            current_hunk.lines.pop();
          }
        }

        set_line_ranges(&mut current_hunk);
        hunks.push(current_hunk.clone());
        current_hunk = Hunk::new();
        started_making_hunk = false;
        gap_count = 0;
      }
    }
  }

  if started_making_hunk {
    set_line_ranges(&mut current_hunk);
    hunks.push(current_hunk.clone());
  }

  set_indices(&mut hunks);

  (hunks, patch_size)
}

fn set_line_ranges(hunk: &mut Hunk) {
  let Hunk { lines, .. } = hunk;

  if lines.is_empty() {
    return;
  }

  let first = &lines[0];
  let last = &lines[lines.len() - 1];

  hunk.old_line_range = HunkRange {
    start: first.old_num.unwrap_or(0),
    length: max(
      last.old_num.unwrap_or(0) - first.old_num.unwrap_or(0) + 1,
      0,
    ),
  };
  hunk.new_line_range = HunkRange {
    start: first.new_num.unwrap_or(0),
    length: max(
      last.new_num.unwrap_or(0) - first.new_num.unwrap_or(0) + 1,
      0,
    ),
  };
}

fn set_indices(hunks: &mut Vec<Hunk>) {
  for (i, hunk) in hunks.iter_mut().enumerate() {
    hunk.index = i as i32;
    for line in &mut hunk.lines {
      line.hunk_index = i as i32;
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::git::queries::wip::create_hunks::convert_lines_to_hunks;
  use crate::git::queries::wip::wip_diff::calc_hunk_line_from_text;
  use std::cmp::max;

  #[test]
  fn test_create_hunks() {
    let text = "import {ThemeName} from '../views/theme/theming'

export const maxNumberOfCommits = 1000
export const maxNumberOfCommits = 100

export const bgSize = 500

export const font = `13px -apple-system,BlinkMacSystemFont,Segoe UI,Helvetica,Arial,sans-serif,Apple Color Emoji,Segoe UI Emoji`

export const monoFont = `13px 'Menlo', 'Ubuntu Mono', 'Consolas', monospace`

export const defaultTheme: ThemeName = 'dark'

export const defaultAnimationTime: AnimationTime = {
  short: 150,
  medium: 300,
  long: 400,
}

export const animationTimeDisabled: AnimationTime = {
  short: 0,
  medium: 0,
  long: 0,
}

export interface AnimationTime {
  short: number
  medium: number
  long: number
}
";

    let lines = calc_hunk_line_from_text("", text);

    assert_eq!(lines.len(), 30);

    let hunks = convert_lines_to_hunks(lines);

    assert_eq!(hunks.0.len(), 1);

    // This is a bit dumb. All lines are added
    assert_eq!(hunks.0[0].lines.len(), 30);
  }

  #[test]
  fn test_max_behaviour() {
    assert_eq!(max(0, -3) as u32, 0);

    assert_eq!([1, 2][0..0].len(), 0);

    // let i: usize = 0;
    // let n: i32 = i - 3;
    //
    // assert_eq!(i - 3, -3 as i32);
  }
}

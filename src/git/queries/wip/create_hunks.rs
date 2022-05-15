use crate::git::git_types::{Hunk, HunkLine, HunkLineStatus, HunkRange};
use std::cmp::{max, min};

pub fn convert_lines_to_hunks(lines: Vec<HunkLine>) -> (Vec<Hunk>, u32) {
  let mut hunks = Vec::<Hunk>::new();
  let mut current_hunk = Hunk::new();
  let mut started_making_hunk = false;

  // Hunks should be joined if there's only 6 unchanged lines between them.
  let mut gap_count = 0;
  let mut patch_size: u32 = 0;

  for (i, line) in lines.iter().enumerate() {
    let HunkLine { status, .. } = line;

    if *status == HunkLineStatus::Added || *status == HunkLineStatus::Removed {
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

fn set_line_ranges(hunk: &mut Hunk) -> () {
  let Hunk { lines, .. } = hunk;

  if lines.len() == 0 {
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

fn set_indices(hunks: &mut Vec<Hunk>) -> () {
  for i in 0..hunks.len() {
    hunks[i].index = i as i32;

    for j in 0..hunks[i].lines.len() {
      hunks[i].lines[j].hunk_index = i as i32;
    }
  }
}

#[cfg(test)]
mod tests {
  use std::cmp::max;

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

use crate::git::git_types::Commit;
use crate::server::graph_instructions::instruction_types::{
  CommitPoint, LineInstruction, PointInstruction,
};
use ahash::AHashMap;

pub(crate) mod api;
pub(crate) mod instruction_types;

pub struct GraphInstructions {
  drawn_commits: AHashMap<String, CommitPoint>,
  filled: Vec<usize>,

  pub points: Vec<PointInstruction>,
  pub lines: Vec<LineInstruction>,
}

impl GraphInstructions {
  pub fn new(len: usize) -> Self {
    Self {
      points: Vec::new(),
      lines: Vec::new(),
      drawn_commits: AHashMap::new(),
      filled: vec![0; len],
    }
  }

  fn generate(&mut self, commit_ids: &Vec<String>, commits: &AHashMap<String, Commit>) {
    let filtered_commits: AHashMap<&String, &Commit> = commit_ids
      .iter()
      .flat_map(|id| Some((id, commits.get(id)?)))
      .collect();

    for (_, commit) in filtered_commits.iter() {
      self.traverse_commit(*commit, commit_ids, &filtered_commits);
    }
  }

  fn traverse_commit(
    &mut self,
    commit: &Commit,
    commit_ids: &Vec<String>,
    filtered_commits: &AHashMap<&String, &Commit>,
  ) {
    let Commit {
      id,
      index,
      is_merge,
      ..
    } = commit;

    if self.drawn_commits.contains_key(id) {
      return;
    }

    let point = CommitPoint {
      x: self.filled[*index],
      y: *index,
      commit_id: id.clone(),
    };

    let colour = strong_colour(&commit.email);

    self.points.push(PointInstruction {
      commit_id: id.clone(),
      x: point.x,
      y: point.y,
      colour: colour.clone(),
      is_merge: *is_merge,
    });

    self.filled[point.y] += 1;
    self.drawn_commits.insert(id.clone(), point.clone());

    for parent_id in commit.parent_ids.iter() {
      if let Some(parent_commit) = filtered_commits.get(parent_id) {
        self.traverse_commit(*parent_commit, commit_ids, filtered_commits);
        self.generate_line(point.clone(), parent_commit, commit_ids, colour.clone());
      }
    }
  }

  fn generate_line(
    &mut self,
    commit_point: CommitPoint,
    parent_commit: &Commit,
    commit_ids: &Vec<String>,
    colour: String,
  ) {
    let mut points = vec![commit_point.clone()];

    let Commit {
      index: parent_y,
      id: parent_id,
      ..
    } = parent_commit;

    for y in (commit_point.y + 1)..*parent_y {
      let x = self.filled[y];
      self.filled[y] += 1;

      points.push(CommitPoint {
        x,
        y,
        commit_id: commit_ids[y].clone(),
      });
    }

    let parent_x = match self.drawn_commits.get(parent_id) {
      Some(CommitPoint { x, .. }) => x,
      None => &self.filled[*parent_y],
    };

    points.push(CommitPoint {
      x: parent_x.clone(),
      y: parent_y.clone(),
      commit_id: commit_ids[*parent_y].clone(),
    });

    self.lines.push(LineInstruction {
      id: format!(
        "{}{}",
        points[0].commit_id,
        points[points.len() - 1].commit_id
      ),
      points,
      colour,
    });
  }
}

fn strong_colour(text: &str) -> String {
  format!("hsl({}, 65%, 60%)", text_to_hue(text))
}

fn text_to_hue(text: &str) -> i64 {
  let mut hash = 0;

  for c in text.chars() {
    hash = c as i64 + ((hash << 5)/* - hash*/);
  }

  (hash.abs() + 240) % 360
}

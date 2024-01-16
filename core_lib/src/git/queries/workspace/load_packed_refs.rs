use crate::git::request_util::R;
use crate::git::store::STORE;
use crate::parser::standard_parsers::{ANY_WORD, UNTIL_LINE_END};
use crate::parser::{parse_all_err, Parser};
use crate::{and, character, many, map2, or, word};
use std::fs::read_to_string;

pub fn load_packed_refs(repo_path: &str) -> R<Vec<PackedRef>> {
  let repo = STORE.get_repo_path(repo_path)?;
  let path = repo.git_path.join("packed-refs");

  let text = read_to_string(path)?;

  parse_all_err(P_LINES, &text)
}

#[derive(Debug, Eq, PartialEq)]
pub enum PackedRef {
  Local(PackedLocalRef),
  Remote(PackedRemoteRef),
  Unknown,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PackedRemoteRef {
  pub commit_id: String,
  pub remote_name: String,
  pub name: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PackedLocalRef {
  pub commit_id: String,
  pub name: String,
}

const P_LOCAL: Parser<PackedRef> = map2!(
  and!(
    ANY_WORD,
    character!(' '),
    word!("refs/heads/"),
    UNTIL_LINE_END
  ),
  res,
  PackedRef::Local(PackedLocalRef {
    commit_id: res.0,
    name: res.3
  })
);

const P_REMOTE: Parser<PackedRef> = map2!(
  and!(
    ANY_WORD,
    character!(' '),
    word!("refs/remotes/"),
    UNTIL_LINE_END
  ),
  res,
  {
    let (remote_name, name) = remove_remote(res.3);

    PackedRef::Remote(PackedRemoteRef {
      commit_id: res.0,
      remote_name,
      name,
    })
  }
);

const P_OTHER: Parser<PackedRef> = map2!(UNTIL_LINE_END, __, PackedRef::Unknown);

const P_LINE: Parser<PackedRef> = or!(P_LOCAL, P_REMOTE, P_OTHER);

const P_LINES: Parser<Vec<PackedRef>> = many!(P_LINE);

fn remove_remote(ref_part: String) -> (String, String) {
  if let Some((remote, tail)) = ref_part.split_once('/') {
    return (remote.to_string(), tail.to_string());
  }
  (String::new(), ref_part)
}

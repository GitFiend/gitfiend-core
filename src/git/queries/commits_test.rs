#[cfg(test)]
mod tests {
  use crate::git::queries::commits::{load_commits, load_commits_and_stashes};
  use crate::git::queries::commits_parsers::{P_COMMIT_ROW, P_GROUP, P_ID_LIST};
  use crate::parser::{_parse_part, parse_all};
  use crate::server::git_request::ReqCommitsOptions;
  use std::env::current_dir;

  #[test]
  fn test_p_group() {
    let result = _parse_part(P_GROUP, "omg,");

    assert!(result.is_some());
  }

  #[test]
  fn test_p_commit_row() {
    let res = parse_all(
      P_COMMIT_ROW,
      "Toby, sugto555@gmail.com, 1648863350 +1300, \
      dd5733ad96082f0f33164bd1e2d72f7540bf7d9f, 2e8966986f620f491c34e6243a546d85dd2322e0, \
      Write commit row parser. Added necessary new git types. 4a41380f-a4e8-4251-9ca2-bf55186ed32a\
      ,  (HEAD -> refs/heads/master, refs/remotes/origin/master)",
    );

    assert_eq!(res.is_some(), true);
  }

  #[test]
  fn test_load_commits() {
    let cwd = current_dir().unwrap();
    let repo_path = cwd.into_os_string().into_string().unwrap();

    let result = load_commits(&repo_path, 5);

    assert!(result.is_some());
  }

  #[test]
  fn test_load_commits_and_stashes() {
    let result = load_commits_and_stashes(&ReqCommitsOptions {
      repo_path: "/home/toby/Repos/gitfiend-seed/git-fiend".to_string(),
      num_commits: 1000,
    });

    println!("{:?}", result);
    assert!(true);
  }

  #[test]
  fn test_p_id_list() {
    let a = "febe3a062cc8c4c156a3d869310976173d64c04a";
    let b = "2269bc6b714890412d0c983825cf9e9d340291ce";
    let c = "1b7a69a328a61e9ad54dfb302cf3301448ab0cfe";
    let d = "fd48376ff1e2cb213bd6d1919cf0f07f81a553a6";

    let ids = format!("{a}\n{b}\n{c}\n{d}");

    let out = parse_all(P_ID_LIST, &ids);

    assert!(out.is_some());
    assert_eq!(out.unwrap().len(), 4);
  }
}

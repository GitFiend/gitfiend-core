#[cfg(test)]
mod tests {
  use crate::git::queries::commits::{
    load_commits, load_commits_and_stashes, P_COMMIT_ROW, P_GROUP,
  };
  use crate::parser::{parse_all, parse_part};
  use crate::server::git_request::ReqCommitsOptions;
  use std::env::current_dir;

  #[test]
  fn test_p_group() {
    let result = parse_part(P_GROUP, "omg,");

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
}

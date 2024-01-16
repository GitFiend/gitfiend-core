use crate::git::git_types::RefInfo;
use crate::git::queries::refs::P_OPTIONAL_REFS;

pub mod commits;
mod commits_parsers;
mod commits_test;

pub mod commit_calcs;
mod commit_filters;
pub mod config;
pub mod hunks;
pub mod patches;
pub mod refs;
pub mod run;
pub mod scan_workspace;
pub mod search;
pub mod stashes;
mod stashes_test;
mod syntax_colouring;
pub mod unpushed_commits;
pub mod wip;
pub mod workspace;

pub const COMMIT_0_ID: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";

use crate::git::git_types::{RefInfo, RefInfoPart};
use crate::git::queries::refs::P_OPTIONAL_REFS;

pub(crate) mod commits;
mod commits_parsers;
mod commits_test;

pub(crate) mod config;
pub(crate) mod patches;
mod refs;
pub(crate) mod stashes;
mod stashes_test;

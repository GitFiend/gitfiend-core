use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::git::store::{get_next_search_num, RwStore};

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SearchOptions {
  pub repo_path: String,
  pub search_text: String,
}

pub fn search_diffs(options: &SearchOptions, store: RwStore) -> Option<()> {
  let search_num = get_next_search_num(&store);

  /*
  TODO: Create and run async version of run_git

  use async_process library for Command.
  call try_status on child process every x milliseconds

  if done, return Some(result), if not, check get_current_search_num() == search_num.
  if we are still the current search, continue polling.
  If we aren't, return None.
   */

  None
}

use crate::git::queries::commits::COMMITS;
use crate::git::queries::search::search_request::clear_completed_searches;
use crate::server::git_request::ReqOptions;

pub fn clear_cache(_: &ReqOptions) {
  COMMITS.clear();
  clear_completed_searches();

  println!("Cleared cache.");
}

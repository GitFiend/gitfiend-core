use crate::server::async_server::start_async_server;

pub(crate) mod git;
mod parser;
mod server;
mod util;

#[cfg(feature = "internal-git")]
pub const INTERNAL_GIT: bool = true;
#[cfg(not(feature = "internal-git"))]
pub const INTERNAL_GIT: bool = false;

fn main() {
  println!("INTERNAL_GIT: {}", INTERNAL_GIT);

  start_async_server();
}

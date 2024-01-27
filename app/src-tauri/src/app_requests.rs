use core_lib::git::git_request::ReqOptions;
use core_lib::handler;

// Note: Not in use right now as we are using the typescript apis.
pub fn run_app_request(name: &str, options: &str) -> serde_json::Result<String> {
  println!("run_app_request: {}, {}", name, options);

  handler! {
    name,
    options,

    show_open_folder_window
  }
}

fn show_open_folder_window(_: &ReqOptions) -> Option<()> {
  //
  println!("TODO: Show open folder window");
  None
}

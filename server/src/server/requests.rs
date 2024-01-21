use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

use tiny_http::{Header, Request, Response, Server};

use core_lib::dprintln;
use core_lib::handle_request::run_request;
use core_lib::util::static_files::get_content_type;

#[cfg(debug_assertions)]
const PORT: u16 = 29997;
#[cfg(not(debug_assertions))]
const PORT: u16 = 0;

const ADDRESS: fn() -> String = || format!("127.0.0.1:{}", PORT);

pub fn start_async_server() {
  let server = Server::http(ADDRESS()).expect("Started server");

  print_port(
    server
      .server_addr()
      .to_ip()
      .expect("Get port for printing")
      .port(),
  );

  for request in server.incoming_requests() {
    match &request.url()[..3] {
      "/r/" => {
        handle_resource_request(request);
      }
      "/pi" => {
        let _ = request.respond(Response::from_string("gitfiend"));
      }
      "/ex" => {
        let _ = request.respond(Response::from_string("GitFiend core exiting..."));
        exit(0);
      }
      "/f/" => {
        handle_server_request(request);
      }
      _ => {
        dprintln!("Unhandled url {}", request.url());
      }
    }
  }
}

fn handle_server_request(mut request: Request) {
  let func_name = &request.url()[3..].to_string();

  let mut options = String::new();
  if let Err(e) = request.as_reader().read_to_string(&mut options) {
    dprintln!("{}", e);
    return;
  }

  if let Ok(result) = run_request(func_name, &options) {
    let _ = request.respond(Response::from_string(result));
  }
}

fn print_port(port: u16) {
  // This is required by the renderer. Expected to be formatted like:
  // PORT:12345
  // We pad the width so we can read a specific number of chars from the stream.
  println!("PORT:{:<12}", port);
}

// TODO: If there's an error then a response won't be sent. This probably leaks memory.
pub fn handle_resource_request(request: Request) -> Option<()> {
  let dir = get_server_dir()?;

  // Remove any extra query part.
  let url = request.url().split('?').next()?;
  let file_path = dir.join(&url[3..]);

  dprintln!("file_path {:?}, exists: {}", file_path, file_path.exists());

  let file = File::open(&file_path).ok()?;
  let mut response = Response::from_file(file);

  let content_type = get_content_type(&file_path.to_string_lossy())?;

  let header = Header::from_str(&content_type).ok()?;
  response.add_header(header);

  let _ = request.respond(response);

  Some(())
}

fn get_server_dir() -> Option<PathBuf> {
  #[cfg(debug_assertions)]
  return Some(env::current_dir().ok()?.parent()?.join("git-fiend"));

  // TODO: This is tested for native mac app, not electron production build.
  // TODO: May need to unpack all from asar?
  #[cfg(not(debug_assertions))]
  Some(
    env::current_exe()
      .ok()?
      .parent()?
      .parent()?
      .parent()?
      .to_path_buf(),
  )
}

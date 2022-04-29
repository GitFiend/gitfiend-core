extern crate directories;

use crate::git::git_types::Patch;
use directories::ProjectDirs;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::BufReader;
use std::path::Path;

pub fn load_patches_cache(repo_path: &String) {
  // Does this create it if it doesn't exist?
  if let Some(proj_dirs) = ProjectDirs::from("com", "tobysuggate", "GitFiend") {
    let config = proj_dirs.config_dir();
    let dir = proj_dirs.cache_dir();

    let id = Path::new(&repo_path)
      .iter()
      .map(|p| p.to_str().unwrap_or(""))
      .collect::<Vec<&str>>()
      .join("");

    println!("{}", id);

    println!("{}, {}", config.display(), dir.display());

    let patches_dir = dir.join("patches");
    let res = create_dir_all(&patches_dir);

    if res.is_ok() {
      let cache_file = patches_dir.join(id);

      let patches = read_patches_from_file(cache_file);
      //
    }
  }
}

fn read_patches_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Patch>, Box<dyn Error>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);

  let patches = serde_json::from_reader(reader)?;

  Ok(patches)
}

fn write_patches_to_file() {
  //
}

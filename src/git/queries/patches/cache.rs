extern crate directories;

use std::collections::HashMap;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

use directories::ProjectDirs;

use crate::git::git_types::Patch;

pub fn write_patches_cache(
  repo_path: &String,
  patches: &HashMap<String, Vec<Patch>>,
) -> Option<()> {
  let cache_dir = get_cache_dir()?;
  let file_name = get_file_name(repo_path);

  write_patches_to_file(cache_dir.join(file_name), patches).ok()
}

pub fn load_patches_cache(repo_path: &String) -> Option<HashMap<String, Vec<Patch>>> {
  let cache_dir = get_cache_dir()?;
  let file_name = get_file_name(repo_path);

  create_dir_all(&cache_dir).ok()?;

  let cache_file = cache_dir.join(file_name);

  read_patches_from_file(cache_file).ok()
}

fn get_cache_dir() -> Option<PathBuf> {
  if let Some(proj_dirs) = ProjectDirs::from("com", "tobysuggate", "GitFiend") {
    let cache_dir = proj_dirs.cache_dir();

    Some(cache_dir.join("patches"))
  } else {
    None
  }
}

fn get_file_name(repo_path: &String) -> String {
  let id = Path::new(&repo_path)
    .iter()
    .map(|p| p.to_str().unwrap_or(""))
    .collect::<Vec<&str>>()
    .join("");

  format!("{}.json", id)
}

fn read_patches_from_file<P: AsRef<Path>>(
  path: P,
) -> Result<HashMap<String, Vec<Patch>>, Box<dyn Error>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);

  let patches = serde_json::from_reader(reader)?;

  Ok(patches)
}

fn write_patches_to_file<P: AsRef<Path>>(
  path: P,
  patches: &HashMap<String, Vec<Patch>>,
) -> Result<(), Box<dyn Error>> {
  let str = serde_json::to_string(&patches)?;

  let mut file = File::create(&path)?;

  file.write_all(str.as_ref())?;

  println!("Wrote patches to '{:?}'", path.as_ref().to_str());

  Ok(())
}

extern crate directories;

use std::collections::HashMap;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use directories::ProjectDirs;

use crate::git::git_types::Patch;
use crate::git::store::PATCHES;

pub fn write_patches_cache(
  repo_path: &String,
  patches: &HashMap<String, Vec<Patch>>,
) -> Option<()> {
  let cache_dir = get_cache_dir()?;
  let file_name = generate_file_name(repo_path);

  let full_path = cache_dir.join(file_name);

  let now = Instant::now();

  PATCHES.set((repo_path.clone(), patches.clone()));
  println!(
    "Took {}ms to put patches in temp cache (write).",
    now.elapsed().as_millis(),
  );

  write_patches_to_file(full_path, patches).ok()
}

pub fn load_patches_cache(repo_path: &String) -> Option<HashMap<String, Vec<Patch>>> {
  if let Some((rp, patches)) = PATCHES.get() {
    if &rp == repo_path && !patches.is_empty() {
      return Some(patches);
    }
  }

  let cache_dir = get_cache_dir()?;
  let file_name = generate_file_name(repo_path);

  create_dir_all(&cache_dir).ok()?;

  let cache_file = cache_dir.join(file_name);

  let maybe_patches = read_patches_from_file(cache_file).ok();

  if let Some(patches) = maybe_patches {
    let now = Instant::now();

    PATCHES.set((repo_path.clone(), patches.clone()));
    println!(
      "Took {}ms to put patches in temp cache (load).",
      now.elapsed().as_millis(),
    );

    return Some(patches);
  }

  None
}

fn get_cache_dir() -> Option<PathBuf> {
  if let Some(proj_dirs) = ProjectDirs::from("com", "tobysuggate", "GitFiend") {
    let cache_dir = proj_dirs.cache_dir();

    Some(cache_dir.join("patches"))
  } else {
    None
  }
}

/// This generates a file name from the repo path e.g.
/// c:\user\something\thing -> cusersomethingthing.json
fn generate_file_name(repo_path: &String) -> String {
  let id = Path::new(&repo_path)
    .iter()
    .map(|p| p.to_str().unwrap_or(""))
    .collect::<Vec<&str>>()
    .join("")
    .replace('\\', "")
    .replace(':', "")
    .replace('/', "");

  format!("{}.json", id)
}

fn read_patches_from_file<P: AsRef<Path>>(
  path: P,
) -> Result<HashMap<String, Vec<Patch>>, Box<dyn Error>> {
  let now = Instant::now();
  let file = File::open(&path)?;

  let mut reader = BufReader::new(file);
  let mut text = String::new();

  reader.read_to_string(&mut text)?;

  let patches = serde_json::from_str(&text)?;

  println!(
    "Took {}ms to read and parse patches. Length {}.",
    now.elapsed().as_millis(),
    text.len()
  );

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

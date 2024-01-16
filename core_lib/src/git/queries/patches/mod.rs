pub mod cache;
mod cache_test;
pub mod patch_parsers;
pub mod patches;
pub mod patches_for_commit;

const IMAGE_EXTENSIONS: [&str; 10] = [
  ".apng", ".bmp", ".gif", ".ico", ".cur", ".jpg", ".jpeg", ".png", ".svg", ".webp",
];

pub fn file_is_image(file_name: &str) -> bool {
  let name = file_name.to_lowercase();

  IMAGE_EXTENSIONS.iter().any(|ext| name.ends_with(ext))
}

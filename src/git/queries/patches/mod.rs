pub(crate) mod cache;
mod cache_test;
pub(crate) mod patch_parsers;
pub(crate) mod patches;
pub(crate) mod patches_for_commit;

const IMAGE_EXTENSIONS: [&str; 10] = [
  ".apng", ".bmp", ".gif", ".ico", ".cur", ".jpg", ".jpeg", ".png", ".svg", ".webp",
];

pub fn file_is_image(file_name: &str) -> bool {
  let name = file_name.to_lowercase();

  IMAGE_EXTENSIONS.iter().any(|ext| name.ends_with(ext))
}

use std::fs::File;
use std::io::Read;

const SAMPLE_SIZE: usize = 8192; // 8KB sample size
const BINARY_THRESHOLD: f32 = 0.30; // 30% binary threshold

pub fn file_is_text(file_name: &str) -> bool {
  let file = match File::open(file_name) {
    Ok(file) => file,
    Err(_) => return false,
  };

  let mut buffer = vec![0; SAMPLE_SIZE];
  let bytes_read = match file.take(SAMPLE_SIZE as u64).read(&mut buffer) {
    Ok(n) => n,
    Err(_) => return false,
  };

  if bytes_read == 0 {
    return true;
  }

  buffer.truncate(bytes_read);

  let binary_count = buffer
    .iter()
    .filter(|&&byte| byte == 0 || byte > 127)
    .count();
  let binary_ratio = binary_count as f32 / bytes_read as f32;

  binary_ratio < BINARY_THRESHOLD
}

pub mod html_code;
pub mod html_code_split;
mod hunk_line_parsers;
pub mod hunk_parsers;
pub mod images;
pub mod load_hunks;

use base64::{engine::general_purpose, Engine as _};

pub fn encode_text<T: AsRef<[u8]>>(bytes: T) -> String {
  general_purpose::STANDARD.encode(bytes)
}

pub fn decode_text<T: AsRef<[u8]>>(encoded: T) -> Vec<u8> {
  if let Ok(text) = general_purpose::STANDARD.decode(encoded) {
    return text;
  }

  Vec::new()
}

fn test() {
  encode_text("");
}

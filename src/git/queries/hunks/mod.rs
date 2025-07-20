mod hunk_line_parsers;
pub mod hunk_parsers;
pub mod images;
pub mod load_hunks;

#[cfg(feature = "syntect")]
pub mod html_code;
#[cfg(feature = "syntect")]
pub mod html_code_split;

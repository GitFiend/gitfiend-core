pub mod data_store;
pub mod debug_print;
pub mod global;
pub mod short_cache;
pub mod static_files;

#[macro_export]
macro_rules! f {
  ($($arg:tt)*) => {{
     let res = std::fmt::format(format_args!($($arg)*));
       res
  }}
}

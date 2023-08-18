pub(crate) mod data_store;
pub(crate) mod debug_print;
pub(crate) mod global;
pub(crate) mod short_cache;

#[macro_export]
macro_rules! f {
  ($($arg:tt)*) => {{
     let res = std::fmt::format(format_args!($($arg)*));
       res
  }}
}

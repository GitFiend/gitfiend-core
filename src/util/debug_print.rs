#[cfg(debug_assertions)]
#[macro_export]
macro_rules! dprintln {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! dprintln {
  ($( $args:expr ),*) => {};
}

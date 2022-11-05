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

#[macro_export]
macro_rules! time {
  ($name:expr, $code:block) => {
    let now = std::time::Instant::now();

    $code

    dprintln!("{} took {}ms", $name, now.elapsed().as_millis());
  }
}

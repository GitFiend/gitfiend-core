#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! dprintln {
  ($( $args:expr ),*) => {};
}
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! dprintln {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! time_block {
  ($name:expr, $code:block) => {
    $code
  };
}
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! time_block {
  ($name:expr, $code:block) => {
    let now = std::time::Instant::now();

    $code

    let ms = now.elapsed().as_millis();

    if (ms > 1) {
      dprintln!("{}ms for {}", now.elapsed().as_millis(), $name);
    }
  }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! time_result {
  ($name:expr, $code:expr) => {{
    $code
  }};
}
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! time_result {
  ($name:expr, $code:expr) => {{
    let now = std::time::Instant::now();

    let result = $code;

    let ms = now.elapsed().as_millis();

    if (ms > 1) {
      dprintln!("{}ms for {}", now.elapsed().as_millis(), $name);
    }

    result
  }};
}

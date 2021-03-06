#[macro_export]
macro_rules! unwrap_or_return {
  ($e:expr) => {
    match $e {
      Some(x) => x,
      _ => return,
    }
  }
}
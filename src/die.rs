#![allow(unused_variables)] // Rust complains with `msg` and `flags` not being used in the Die trait, but they are actually used in the trait's implementation

#[cfg(not(debug_assertions))]
use crate::color::ColorExt;
use crate::core::args::Flags;

pub trait Die<T> {
  fn die(self, msg: &str, flags: &Flags) -> T;
}

impl<T, E> Die<T> for Result<T, E>
where
  E: std::fmt::Debug,
{
  fn die(self, msg: &str, flags: &Flags) -> T {
    #[cfg(not(debug_assertions))]
    if let Err(_) = self {
      println!("{}: {}", "ERROR".red(flags).bright(flags).reset(flags), msg);
      std::process::exit(1)
    } else {
      self.unwrap()
    }

    #[cfg(debug_assertions)]
    self.expect(msg)
  }
}

impl<T> Die<T> for Option<T> {
  fn die(self, msg: &str, flags: &Flags) -> T {
    #[cfg(not(debug_assertions))]
    if let None = self {
      println!("{}: {}", "ERROR".red(flags).bright(flags).reset(flags), msg);
      std::process::exit(1)
    } else {
      self.unwrap()
    }

    #[cfg(debug_assertions)]
    self.expect(msg)
  }
}

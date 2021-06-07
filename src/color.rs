use std::fmt::Display;

use crate::core::*;

#[cfg(feature = "color")]
const RESET: &str = "\x1b[0m";
#[cfg(feature = "color")]
const BRIGHT: &str = "\x1b[1m";
#[cfg(feature = "color")]
const UNDERLINE: &str = "\x1b[4m";
#[cfg(feature = "color")]
const GREEN: &str = "\x1b[32m";
#[cfg(feature = "color")]
const YELLOW: &str = "\x1b[33m";
#[cfg(feature = "color")]
const CYAN: &str = "\x1b[36m";
#[cfg(feature = "color")]
const WHITE: &str = "\x1b[37m";
#[cfg(feature = "color")]
const GREY: &str = "\x1b[90m";
#[cfg(feature = "color")]
const RED: &str = "\x1b[31m";

pub trait ColorExt {
  fn reset(self, _flags: &args::Flags) -> String
  where
    Self: Sized + Display,
  {
    format!("{}", self)
  }

  fn bright(self, _flags: &args::Flags) -> String
  where
    Self: Sized + Display,
  {
    format!("{}", self)
  }

  fn underline(self, _flags: &args::Flags) -> String
  where
    Self: Sized + Display,
  {
    format!("{}", self)
  }

  fn green(self, _flags: &args::Flags) -> String
  where
    Self: Sized + Display,
  {
    format!("{}", self)
  }

  fn yellow(self, _flags: &args::Flags) -> String
  where
    Self: Sized + Display,
  {
    format!("{}", self)
  }

  fn cyan(self, _flags: &args::Flags) -> String
  where
    Self: Sized + Display,
  {
    format!("{}", self)
  }

  fn white(self, _flags: &args::Flags) -> String
  where
    Self: Sized + Display,
  {
    format!("{}", self)
  }

  fn grey(self, _flags: &args::Flags) -> String
  where
    Self: Sized + Display,
  {
    format!("{}", self)
  }

  fn red(self, _flags: &args::Flags) -> String
  where
    Self: Sized + Display,
  {
    format!("{}", self)
  }

  fn custom(self, _color: &str, _flags: &args::Flags) -> String
  where
    Self: Sized + Display,
  {
    format!("{}", self)
  }
}

#[cfg(feature = "color")]
impl<T> ColorExt for T
where
  T: std::fmt::Display,
{
  fn reset(self, flags: &args::Flags) -> String {
    if flags.no_color {
      format!("{}", self)
    } else {
      format!("{}{}", self, String::from(RESET))
    }
  }

  fn bright(self, flags: &args::Flags) -> String {
    if flags.no_color {
      format!("{}", self)
    } else {
      format!("{}{}", String::from(BRIGHT), self)
    }
  }

  fn underline(self, flags: &args::Flags) -> String {
    if flags.no_color {
      format!("{}", self)
    } else {
      format!("{}{}", String::from(UNDERLINE), self)
    }
  }

  fn green(self, flags: &args::Flags) -> String {
    if flags.no_color {
      format!("{}", self)
    } else {
      format!("{}{}", String::from(GREEN), self)
    }
  }

  fn yellow(self, flags: &args::Flags) -> String {
    if flags.no_color {
      format!("{}", self)
    } else {
      format!("{}{}", String::from(YELLOW), self)
    }
  }

  fn cyan(self, flags: &args::Flags) -> String {
    if flags.no_color {
      format!("{}", self)
    } else {
      format!("{}{}", String::from(CYAN), self)
    }
  }

  fn white(self, flags: &args::Flags) -> String {
    if flags.no_color {
      format!("{}", self)
    } else {
      format!("{}{}", String::from(WHITE), self)
    }
  }

  fn grey(self, flags: &args::Flags) -> String {
    if flags.no_color {
      format!("{}", self)
    } else {
      format!("{}{}", String::from(GREY), self)
    }
  }

  fn red(self, flags: &args::Flags) -> String {
    if flags.no_color {
      format!("{}", self)
    } else {
      format!("{}{}", String::from(RED), self)
    }
  }

  fn custom(self, color: &str, flags: &args::Flags) -> String {
    if flags.no_color {
      format!("{}", self)
    } else {
      format!("{}{}", String::from(color), self)
    }
  }
}

#[cfg(not(feature = "color"))]
impl<T> ColorExt for T where T: std::fmt::Display {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn color() {
    let flags = args::Flags {
      all: false,
      size: false,
      tree: false,
      icons: false,
      no_color: false,
      no_git: false,
    };

    assert_eq!(format!("{}abc{}", CYAN, RESET), format!("{}", "abc".cyan(&flags).reset(&flags)));
  }

  #[test]
  fn no_color() {
    let flags = args::Flags {
      all: false,
      size: false,
      tree: false,
      icons: false,
      no_color: true,
      no_git: false,
    };

    assert_eq!("abc", format!("{}", "abc".cyan(&flags).reset(&flags)));
  }
}

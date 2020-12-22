use crate::utils::Flags;

pub const RESET: &str = "\x1b[0m";
pub const BRIGHT: &str = "\x1b[1m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";
pub const GREY: &str = "\x1b[90m";

pub fn get_color(color: &str, flags: &Flags) -> String {
  if flags.no_color {
    String::new()
  } else {
    String::from(color)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn color() {
    let flags = Flags {
      all: false,
      size: false,
      tree: false,
      no_color: false,
    };

    assert_eq!(format!("{}abc{}", CYAN, RESET), format!("{}abc{}", get_color(CYAN, &flags), get_color(RESET, &flags)));
  }

  #[test]
  fn no_color() {
    let flags = Flags {
      all: false,
      size: false,
      tree: false,
      no_color: true,
    };

    assert_eq!("abc", format!("{}abc{}", get_color(CYAN, &flags), get_color(RESET, &flags)));
  }
}

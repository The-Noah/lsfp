use crate::utils::Flags;
pub const RESET: &str = "\x1b[0m";
pub const BRIGHT: &str = "\x1b[1m";
pub const UNDERLINE: &str = "\x1b[4m";
pub const GREEN: &str = "\x1b[32m";
pub const ORANGE: &str = "\x1b[33m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";
pub const GREY: &str = "\x1b[90m";
pub trait ColorExt {
  fn reset(&self, flags: &Flags) -> Self;
  fn bright(&self, flags: &Flags) -> Self;
  fn underline(&self, flags: &Flags) -> Self;
  fn green(&self, flags: &Flags) -> Self;
  fn orange(&self, flags: &Flags) -> Self;
  fn cyan(&self, flags: &Flags) -> Self;
  fn white(&self, flags: &Flags) -> Self;
  fn grey(&self, flags: &Flags) -> Self;
  fn custom(&self, color: &str, flags: &Flags) -> Self;
}
impl ColorExt for String {
  fn reset(&self, flags: &Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(RESET), self)
    }
  }
  fn bright(&self, flags: &Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(BRIGHT), self)
    }
  }
  fn underline(&self, flags: &Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(UNDERLINE), self)
    }
  }
  fn green(&self, flags: &Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(GREEN), self)
    }
  }
  fn orange(&self, flags: &Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(ORANGE), self)
    }
  }
  fn cyan(&self, flags: &Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(CYAN), self)
    }
  }
  fn white(&self, flags: &Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(WHITE), self)
    }
  }
  fn grey(&self, flags: &Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(GREY), self)
    }
  }
  fn custom(&self, color: &str, flags: &Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(color), self)
    }
  }
}

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
      no_git: false,
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
      no_git: false,
    };

    assert_eq!("abc", format!("{}abc{}", get_color(CYAN, &flags), get_color(RESET, &flags)));
  }
}

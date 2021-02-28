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
  fn reset(&self, flags: &args::Flags) -> Self;
  fn bright(&self, flags: &args::Flags) -> Self;
  fn underline(&self, flags: &args::Flags) -> Self;
  fn green(&self, flags: &args::Flags) -> Self;
  fn yellow(&self, flags: &args::Flags) -> Self;
  fn cyan(&self, flags: &args::Flags) -> Self;
  fn white(&self, flags: &args::Flags) -> Self;
  fn grey(&self, flags: &args::Flags) -> Self;
  fn red(&self, flags: &args::Flags) -> Self;
  fn custom(&self, color: &str, flags: &args::Flags) -> Self;
}

#[cfg(feature = "color")]
impl ColorExt for String {
  fn reset(&self, flags: &args::Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", self, String::from(RESET))
    }
  }

  fn bright(&self, flags: &args::Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(BRIGHT), self)
    }
  }

  fn underline(&self, flags: &args::Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(UNDERLINE), self)
    }
  }

  fn green(&self, flags: &args::Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(GREEN), self)
    }
  }

  fn yellow(&self, flags: &args::Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(YELLOW), self)
    }
  }

  fn cyan(&self, flags: &args::Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(CYAN), self)
    }
  }

  fn white(&self, flags: &args::Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(WHITE), self)
    }
  }

  fn grey(&self, flags: &args::Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(GREY), self)
    }
  }

  fn red(&self, flags: &args::Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(RED), self)
    }
  }

  fn custom(&self, color: &str, flags: &args::Flags) -> Self {
    if flags.no_color {
      self.to_string()
    } else {
      format!("{}{}", String::from(color), self)
    }
  }
}

#[cfg(not(feature = "color"))]
impl ColorExt for String {
  fn reset(&self, _flags: &args::Flags) -> Self {
    self.to_string()
  }

  fn bright(&self, _flags: &args::Flags) -> Self {
    self.to_string()
  }

  fn underline(&self, _flags: &args::Flags) -> Self {
    self.to_string()
  }

  fn green(&self, _flags: &args::Flags) -> Self {
    self.to_string()
  }

  fn yellow(&self, _flags: &args::Flags) -> Self {
    self.to_string()
  }

  fn cyan(&self, _flags: &args::Flags) -> Self {
    self.to_string()
  }

  fn white(&self, _flags: &args::Flags) -> Self {
    self.to_string()
  }

  fn grey(&self, _flags: &args::Flags) -> Self {
    self.to_string()
  }

  fn red(&self, _flags: &args::Flags) -> Self {
    self.to_string()
  }

  fn custom(&self, _color: &str, _flags: &args::Flags) -> Self {
    self.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn color() {
    let flags = args::Flags {
      all: false,
      size: false,
      tree: false,
      no_color: false,
      no_git: false,
    };

    assert_eq!(format!("{}abc{}", CYAN, RESET), format!("{}", "abc".to_owned().cyan(&flags).reset(&flags)));
  }

  #[test]
  fn no_color() {
    let flags = args::Flags {
      all: false,
      size: false,
      tree: false,
      no_color: true,
      no_git: false,
    };

    assert_eq!("abc", format!("{}", "abc".to_owned().cyan(&flags).reset(&flags)));
  }
}

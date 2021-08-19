#[cfg(feature = "themes")]
use std::path::Path;

#[cfg(feature = "themes")]
use crate::core::args::Flags;
#[cfg(feature = "home")]
use crate::modules::home;
#[cfg(feature = "themes")]
use crate::Die;

#[cfg(feature = "themes")]
pub mod lang;
#[cfg(feature = "themes")]
mod parser;
mod types;

#[cfg(feature = "themes")]
pub use parser::Parser;
pub use types::{Color, Theme, VecConvert};
#[cfg(feature = "themes")]
pub use types::{Line, ParserError};

#[cfg(feature = "themes")]
pub fn get_theme(name: &Option<String>, flags: &Flags) -> String {
  #[cfg(feature = "themes")]
  match name {
    Some(path) => {
      let theme_dir = home::home_dir().die("Unable to get home directory", flags).join(".lsfp-themes");
      if Path::new(&path).exists() && (path.contains('.') || path.contains('/') || path.contains('\\')) {
        std::fs::read_to_string(&path).die("Unable to read passed file path", flags)
      } else {
        let mut text = String::new();
        if !theme_dir.exists() {
          std::fs::create_dir(&theme_dir).die("Unable to create theme directory", flags);
        };
        for item in std::fs::read_dir(&theme_dir).die("Unable to read theme directory", flags) {
          if item.die("Unable to unwrap theme directory read result", flags).file_name() == std::ffi::OsString::from(&path) {
            text = std::fs::read_to_string(&theme_dir.join(&path)).die(&format!("Unable to read named file {}", path), flags)
          }
        }
        text
      }
    }
    None => String::new(),
  }
  #[cfg(not(feature = "themes"))]
  String::new()
}

use crate::themes::types::FileStyle;
use crate::themes::{Color, Line, ParserError, VecConvert};

#[derive(Debug, PartialEq, Eq)]
pub struct Language<'a> {
  extensions: Vec<String>,
  extensions_str: Vec<&'a str>,
  color: Color,
  icon: String,
}

impl<'a> Language<'a> {
  pub const fn new(extensions: Vec<String>, color: Color, icon: String) -> Self {
    Self {
      extensions,
      extensions_str: Vec::new(),
      color,
      icon,
    }
  }

  pub fn parse(text: &[Line]) -> Result<Language<'a>, ParserError> {
    let mut extensions = Vec::new();
    #[cfg(feature = "icons")]
    let mut color: Vec<u8> = Vec::new();
    #[cfg(not(feature = "icons"))]
    let color: Vec<u8> = Vec::new();
    #[cfg(feature = "icons")]
    let mut icon: String = String::new();
    #[cfg(not(feature = "icons"))]
    let icon: String = String::new();

    for (index, original, (key, value)) in text
      .iter()
      .map(|(i, line)| (i, line, line.split_once('#').unwrap_or((line, "")).0)) // Remove inline comments
      .map(|(i, original, pair)| (i, original, pair.split_once('=').unwrap_or_else(|| pair.split_once(':').unwrap_or(("", "")))))
      .map(|(i, original, (key, value))| (*i, *original, (key.trim(), value.trim())))
    {
      if key.is_empty() && value.is_empty() {
        return Err(ParserError::new(
          index,
          original.to_owned(),
          format!(
            "Line must include a pair of key and value separated by a equal (=) in text line \"{key}={value}\" ({key}={value})"
          ),
        ));
      } else if key.is_empty() {
        return Err(ParserError::new(index, original.to_owned(), format!("Missing key in text line \"{key}={value}\"")));
      } else if value.is_empty() {
        return Err(ParserError::new(index, original.to_owned(), format!("Missing value in text line \"{key}={value}\"")));
      } else if key == "e" || key == "extensions" {
        let mut ext = String::new();
        for (i, mut ch) in value.char_indices() {
          if ch == ',' && i == 0 {
            continue;
          }
          if i == value.len() - 1 && ch != ',' {
            ext.push(ch);
            ch = ',';
          }
          if ch != ',' {
            ext.push(ch)
          } else {
            extensions.push(ext);
            ext = String::new();
          }
        }
      } else if key == "c" || key == "color" {
        #[cfg(feature = "color")]
        {
          let mut num = String::new();
          for (i, mut ch) in value.char_indices() {
            if color.len() >= 3 {
              return Err(ParserError::new(
                index,
                original.to_owned(),
                "Only 3 numbers (red, green, blue) should be provided for color".to_owned(),
              ));
            }
            if num.len() > 3 {
              return Err(ParserError::new(index, original.to_owned(), format!("Color must range from 0 to 255, received {num}")));
            }
            if ch == ',' && i == 0 {
              continue;
            }
            if i == value.len() - 1 && ch != ',' {
              num.push(ch);
              ch = ',';
            }
            if ch != ',' {
              num.push(ch)
            } else {
              color.push(match num.parse() {
                Ok(num) => num,
                Err(err) => return Err(ParserError::new(index, original.to_owned(), err.to_string())),
              });
              num = String::new();
            }
          }
        }
      } else if key == "i" || key == "icon" {
        #[cfg(feature = "icons")]
        {
          if value.len() > 4 {
            return Err(ParserError::new(
              index,
              original.to_owned(),
              format!(
                "Icon must be 4 characters, 2 hexadecimal values, but found \"{}\", which is {} characters long",
                value,
                value.len()
              ),
            ));
          }
          if !value.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ParserError::new(
              index,
              original.to_owned(),
              format!("One or more characters in \"{value}\" icon are not valid hexadecimal characters"),
            ));
          }
          icon = value.to_owned()
        }
      }
    }

    if extensions.is_empty() {
      Err(ParserError::new(
        text[text.len() - 1].0,
        text[text.len() - 1].1.to_owned(),
        "extensions key must be present".to_owned(),
      ))
    } else {
      Ok(Language::new(extensions, color.as_color(), icon))
    }
  }

  pub fn as_style(&'a mut self) -> FileStyle {
    self.extensions_str = self.extensions.iter().map(String::as_str).collect();
    // self.extensions_str = self.extensions.as_slice();
    (&self.extensions_str[..], self.color, &self.icon)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn as_vec(text: &str) -> Vec<(usize, &str)> {
    text.lines().enumerate().collect()
  }

  mod languages {
    use super::*;

    #[test]
    fn js() {
      assert_eq!(
        Language::new(vec!["js".to_owned(), "mjs".to_owned()], (255, 220, 0), "f898".to_owned()),
        Language::parse(&as_vec("extensions=js,mjs\ncolor=255,220,0\nicon=f898")).unwrap()
      )
    }

    #[test]
    fn ts() {
      assert_eq!(
        Language::new(vec!["ts".to_owned()], (0, 31, 63), "e628".to_owned()),
        Language::parse(&as_vec("extensions=ts,\ncolor=0,31,63\nicon=e628")).unwrap()
      )
    }

    #[test]
    fn rs() {
      assert_eq!(
        Language::new(vec!["rs".to_owned()], (255, 65, 54), "e7A8".to_owned()),
        Language::parse(&as_vec("extensions=,rs,\ncolor=255,65,54,\nicon=e7A8")).unwrap()
      )
    }
  }

  mod panic {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_line() {
      Language::parse(&as_vec("-lang\nwrong line with no equal at all")).unwrap();
    }

    #[test]
    #[should_panic]
    fn too_long_number() {
      Language::parse(&as_vec("extensions=,rs,\ncolor=2555,65,54,\nicon=e7A8")).unwrap();
    }

    #[test]
    #[should_panic]
    fn too_big_number() {
      Language::parse(&as_vec("extensions=,rs,\ncolor=458,0,54,\nicon=e7A8")).unwrap();
    }

    #[test]
    #[should_panic]
    fn too_many_number() {
      Language::parse(&as_vec("extensions=,rs,\ncolor=-58,0,54,234,\nicon=e7A8")).unwrap();
    }

    #[test]
    #[should_panic]
    fn negative_numbers() {
      Language::parse(&as_vec("extensions=,rs,\ncolor=458,0,54,\nicon=e7A8")).unwrap();
    }
  }
}

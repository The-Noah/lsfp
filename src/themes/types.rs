#[cfg(feature = "themes")]
use std::{error, fmt};

pub type Color = (u8, u8, u8);

pub type FileStyle<'a> = (&'a [&'a str], Color, &'a str);

pub type Theme<'a> = &'a [FileStyle<'a>];

pub trait VecConvert {
  fn as_color(&self) -> Color;
}

impl VecConvert for Vec<u8> {
  fn as_color(&self) -> Color {
    (*self.first().unwrap_or(&0), *self.get(1).unwrap_or(&0), *self.get(2).unwrap_or(&0))
  }
}

#[cfg(feature = "themes")]
pub type Line<'a> = (usize, &'a str);

#[cfg(feature = "themes")]
#[derive(Debug)]
pub struct ParserError {
  line: usize,
  text: String,
  msg: String,
}

#[cfg(feature = "themes")]
impl ParserError {
  pub fn new(line: usize, text: String, msg: String) -> ParserError {
    ParserError { line, text, msg }
  }
}

#[cfg(feature = "themes")]
impl fmt::Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&format!("parser error on line {} with contents \"{}\": {}", self.line, self.text, self.msg.to_lowercase()))
  }
}

#[cfg(feature = "themes")]
impl error::Error for ParserError {}

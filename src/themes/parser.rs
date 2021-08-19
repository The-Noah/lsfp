use crate::themes::lang::Language;
use crate::themes::{Line, ParserError};

#[derive(Debug)]
pub struct Parser {
  text: String, // iter: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Parser {
  pub fn new(text: String) -> Parser {
    Parser {
      text, // iter: text.chars().peekable(),
    }
  }

  /*
  fn next_char(&mut self) -> Option<&char> {
    self.iter.peek()
  }

  fn consume_char(&mut self) -> Option<char> {
    self.iter.next()
  }

  fn consume_while<F>(&mut self, test_fun: F) -> String
  where
    F: Fn(&char) -> bool,
  {
    let mut result = String::new();
    while !self.next_char().is_none() && test_fun(self.next_char().unwrap_or(&' ')) {
      result.push(self.consume_char().unwrap());
    }
    result
  }

  fn consume_whitespace(&mut self) {
    self.consume_while(|c| c.is_whitespace());
  }

  fn consume_until_newline(&mut self) -> String {
    let res = self.consume_while(|c| c != &'\n' && c != &'\r');
    self.consume_char();
    res
  }

  fn consume_ascii(&mut self) -> String {
    self.consume_while(|c| c.is_ascii())
  }
  */

  pub fn parse(self) -> Result<Vec<Language<'a>>, ParserError> {
    let mut split = self.text.as_str().lines().enumerate();

    let mut section_started = false;

    // let mut reading_lang = false;
    let mut langs = Vec::<Language>::new();
    let mut lang = Vec::<Line<'a>>::new();

    loop {
      let option = split.next();
      let (index, line) = match option {
        Some(opt) => (opt.0 + 1, Some(opt.1)),
        None => (0, None),
      };
      if line.is_none() || line.unwrap_or_default().trim_start().starts_with('-') {
        section_started = true;
        if !lang.is_empty() {
          langs.push(Language::parse(lang.as_slice())?);
        }
        lang = Vec::new();
        // reading_lang = false;
        if line.is_none() {
          break;
        } else {
          continue;
        }
      }
      let line = line.unwrap().trim();
      if !line.is_empty() && !line.starts_with('#') {
        if !section_started {
          return Err(ParserError::new(
            index,
            line.to_owned(),
            "Expected file to start with section or comment, instead pair was found".to_owned(),
          ));
        }
        lang.push((index, line))
      }
    }

    Ok(langs)
  }
}

use crate::constants;
use std::ffi::OsStr;
use std::fs;
use std::path;

pub fn is_hidden(path: &path::Path) -> bool {
  // name.chars().nth(0).unwrap() == '.'
  let file = fs::File::open(path);
  if file.is_ok() {
    false
  } else {
    true
  }
}

pub fn get_license(path: &path::Path) -> String {
  let mut contents = fs::read_to_string(path).expect("Something went wrong reading the file");
  contents = contents.replace("\r", "").trim().to_string();

  let mut final_contents = String::new();
  for line in contents.split("\n") {
    final_contents += &(line.trim().to_string() + "\n");
  }

  let mut license_type = "Custom";
  for license in constants::LICENSES {
    if final_contents.starts_with(license.1) {
      license_type = license.0;
      break;
    }
  }

  license_type.to_string()
}

pub fn file_extension_color(path: &path::PathBuf) -> String {
  let mut color = String::new();

  'extension_loop: for extension_color in constants::FILE_EXTENSION_COLORS.iter() {
    for extension in extension_color.0 {
      if extension_matches(path, extension) {
        color = format!("\x1b[38;2;{};{};{}m", extension_color.1 .0, extension_color.1 .1, extension_color.1 .2);
        break 'extension_loop;
      }
    }
  }

  color
}

fn extension_matches(path: &std::path::PathBuf, extension: &str) -> bool {
  if match path.extension() {
    Some(val) => OsStr::new(val).to_str().unwrap_or(""),
    None => "",
  } == extension
  {
    true
  } else {
    false
  }
}

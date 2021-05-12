use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::constants;

#[cfg(target_os = "windows")]
fn is_hidden_extra(path: &Path) -> bool {
  use std::os::windows::fs::MetadataExt;

  let file_metadata = fs::metadata(path).expect("Failed getting metadata");
  let attribs = file_metadata.file_attributes();

  // https://docs.microsoft.com/en-us/dotnet/api/system.io.fileattributes?view=net-5.0#fields
  attribs & 2 == 2
}

#[cfg(not(target_os = "windows"))]
fn is_hidden_extra(_path: &Path) -> bool {
  false
}

pub fn is_hidden(path: &Path) -> bool {
  let item_name = OsStr::new(path.file_name().expect("Unable to get file name")).to_str().expect("Unable to parse file name");

  item_name.starts_with('.') || is_hidden_extra(path)
}

pub fn get_license(path: &Path) -> String {
  let mut contents = fs::read_to_string(path).expect("Failed reading license file");
  contents = contents.replace("\r", "").trim().to_string();

  let mut final_contents = String::new();
  for line in contents.split('\n') {
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

pub fn file_extension_color(path: &Path) -> String {
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

fn extension_matches(path: &Path, extension: &str) -> bool {
  path.extension().unwrap_or(OsStr::new("")).to_str().expect("Unable to parse file extension") == extension
}

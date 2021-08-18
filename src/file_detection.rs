use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::constants;
use crate::core::args::Flags;
use crate::die::Die;
#[cfg(feature = "icons")]
use crate::modules::icon;
use crate::themes::Theme;

#[cfg(target_os = "windows")]
fn is_hidden_extra(path: &Path, flags: &Flags) -> bool {
  use std::os::windows::fs::MetadataExt;

  let file_metadata = fs::metadata(path).die("Failed getting metadata", flags);
  let attribs = file_metadata.file_attributes();

  // https://docs.microsoft.com/en-us/dotnet/api/system.io.fileattributes?view=net-5.0#fields
  attribs & 2 == 2
}

#[cfg(not(target_os = "windows"))]
fn is_hidden_extra(_path: &Path, _flags: &Flags) -> bool {
  false
}

pub fn is_hidden(path: &Path, flags: &Flags) -> bool {
  let item_name = OsStr::new(path.file_name().die("Unable to get file name", flags))
    .to_str()
    .die("Unable to parse file name", flags);

  item_name.starts_with('.') || is_hidden_extra(path, flags)
}

pub fn get_license(path: &Path, flags: &Flags) -> String {
  let mut contents = fs::read_to_string(path).die("Failed reading license file", flags);
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

pub fn file_extension_styles(path: &Path, theme: &Theme, flags: &Flags) -> (String, String) {
  for extension_color in theme.iter() {
    for extension in extension_color.0 {
      if extension_matches(path, extension, flags) {
        return (
          format!("\x1b[38;2;{};{};{}m", extension_color.1 .0, extension_color.1 .1, extension_color.1 .2,),
          #[cfg(feature = "icons")]
          if flags.icons { icon::from(extension_color.2) } else { String::new() },
          #[cfg(not(feature = "icons"))]
          String::new(),
        );
      }
    }
  }

  #[cfg(feature = "icons")]
  if flags.icons {
    return (String::new(), icon::from(constants::ICON_GENERIC));
  }
  (String::new(), String::new())
}

fn extension_matches(path: &Path, extension: &str, flags: &Flags) -> bool {
  path.extension().unwrap_or_else(|| OsStr::new("")).to_str().die("Unable to parse file extension", flags) == extension
}

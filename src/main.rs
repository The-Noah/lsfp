use std::ffi::OsStr;
use std::fs;
use std::path;

mod color;
mod constants;
mod core;
mod die;
mod file_detection;
mod modules;
mod prelude;
mod themes;

use crate::core::*;
#[cfg(feature = "icons")]
use modules::icon;
use prelude::*;

fn print_item(root: &path::Path, path: path::PathBuf, theme: &themes::Theme, flags: &args::Flags, single_item: bool) {
  if !flags.all && file_detection::is_hidden(&path, flags) && !single_item {
    return;
  }
  let mut color = if path.is_dir() { "".cyan(flags) } else { "".to_owned() };

  let mut prefix = String::new();
  let mut name_prefix = String::new();
  let mut suffix = String::new();

  let item_name = OsStr::new(path.file_name().die("Unable to get file name", flags))
    .to_str()
    .die("Unable to parse file name", flags);

  let file_path = OsStr::new(path.strip_prefix(root).die("Unable to get file path", flags))
    .to_str()
    .die("Unable to parse file path", flags);

  #[cfg(feature = "icons")]
  let mut icon = String::new();
  #[cfg(not(feature = "icons"))]
  let icon = String::new();

  #[cfg(feature = "icons")]
  if path.is_dir() && flags.icons {
    if !flags.all && flags.tree && constants::COLLAPSED_DIRECTORIES.contains(&item_name) {
      icon = icon::from(constants::ICON_FOLDER_CLOSED); // Closed folder icon (font awesome outline)
    } else {
      icon = icon::from(constants::ICON_FOLDER_OPEN); // Open folder icon (font awesome outline)
    }
  }

  let mut indentation: u32 = 0;
  if flags.tree {
    let mut parent_path = path.parent().die("Failed to get parent path", flags).to_path_buf();

    while parent_path != root {
      parent_path = parent_path.parent().die("Failed to get parent path", flags).to_path_buf();
      indentation += 1;
    }
  }

  if flags.size {
    let size = match path.metadata() {
      Ok(metadata) => metadata.len(),
      Err(_) => 0,
    };

    prefix += format!("{} ", modules::file_size::human_readable_size(size))
      .bright(flags)
      .grey(flags)
      .reset(flags)
      .as_str();
  }

  if path.is_file() {
    if item_name.to_lowercase().starts_with("license") {
      // Add license icon if the flag is passed
      color = "".white(flags);
      #[cfg(feature = "icons")]
      if flags.icons {
        icon = icon::from(constants::ICON_LICENSE);
      }
      suffix += format!(" [{}]", file_detection::get_license(path.as_path(), flags)).grey(flags).as_str();
    } else {
      let styles = file_detection::file_extension_styles(&path, theme, flags);
      color = styles.0;
      #[cfg(feature = "icons")]
      if flags.icons {
        icon = styles.1;
      }
    }

    // file path for git
    let mut final_path = root.to_path_buf();
    if !root.is_file() {
      final_path.push(file_path);
    }

    // file changed (git)
    #[cfg(feature = "git")]
    if !flags.no_git && modules::git::changed(&final_path, flags) {
      suffix += format!(
        " {}{}{}",
        "[".grey(flags).reset(flags),
        "M".bright(flags).yellow(flags).reset(flags),
        "]".grey(flags).reset(flags)
      )
      .as_str();
    }
  } else if !flags.all && flags.tree && constants::COLLAPSED_DIRECTORIES.contains(&item_name) {
    name_prefix = String::new().underline(flags);
  }

  suffix += String::new().reset(flags).as_str();

  #[cfg(feature = "icons")]
  let leading_space = if flags.icons { " " } else { "" };
  #[cfg(not(feature = "icons"))]
  let leading_space = "";

  println!(
    "{}{}{}{}{}{}{}{}{}{}{}",
    if indentation == 0 && leading_space == " " {
      leading_space.to_owned()
    } else if leading_space == " " {
      (0..indentation).map(|_| leading_space).collect::<String>()
    } else {
      String::new()
    }, // Align tree branches properly when leading spaces appear
    prefix,
    (3..indentation * 3).map(|_| " ").collect::<String>(),
    if indentation > 0 { "└──" } else { "" },
    if indentation > 0 { leading_space } else { "" },
    if !color.is_empty() { "".bright(flags) } else { String::new() },
    name_prefix.custom(color.as_str(), flags),
    icon,
    if path.extension().unwrap_or(&std::ffi::OsString::new()) != "pl" {
      leading_space
    } else {
      ""
    }, // Perl's icon already includes an space after it, so this ensure that it is not shown when printing a perl file
    item_name.reset(flags),
    suffix,
  );
}

fn do_scan(root: &path::Path, path_to_scan: &path::Path, theme: &themes::Theme, flags: &args::Flags) {
  if !flags.all && file_detection::is_hidden(path_to_scan, flags) {
    return;
  }

  let item_name = OsStr::new(path_to_scan.file_name().die("Unable to get file name", flags))
    .to_str()
    .die("Unable to parse file name", flags);

  if path_to_scan.is_file() {
    let path = path::PathBuf::from(path_to_scan);
    print_item(root, path, theme, flags, false);
  } else {
    if !flags.all && flags.tree && constants::COLLAPSED_DIRECTORIES.contains(&item_name) {
      return;
    }

    for entry in fs::read_dir(path_to_scan).die("Directory cannot be accessed", flags) {
      let path = entry.die("Failed retrieving path", flags).path();

      print_item(root, path.clone(), theme, flags, false);

      if path.is_dir() {
        do_scan(root, path.as_path(), theme, flags);
      }
    }
  }
}

fn main() {
  let (flags, mut args) = args::get();

  #[cfg(target_os = "windows")]
  #[cfg(feature = "color")]
  if !flags.no_color {
    use windows::Win32::System::Console::{GetConsoleMode, GetStdHandle, SetConsoleMode, CONSOLE_MODE, ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_OUTPUT_HANDLE};

    unsafe {
      let handle = GetStdHandle(STD_OUTPUT_HANDLE).die("Failed to get stdout", &flags);
      let mut mode: CONSOLE_MODE = CONSOLE_MODE(0);

      GetConsoleMode(handle, &mut mode).die("Failed to get console mode", &flags);

      if !mode.contains(ENABLE_VIRTUAL_TERMINAL_PROCESSING) {
        SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING).die("Failed to set console mode", &flags);
      }
    }
  }

  // Get theme
  #[cfg(feature = "themes")]
  let mut _theme = themes::get_theme(&flags.theme, &flags);
  #[cfg(feature = "themes")]
  let mut parsed = match themes::Parser::new(_theme.clone()).parse() {
    Ok(some) => some,
    Err(err) => {
      _theme = String::new();
      println!("{} {}", "THEME PARSING ERROR:".yellow(&flags), err.reset(&flags));
      Vec::new()
    }
  }; // Parsed is declared here because theme borrows it
  #[cfg(feature = "themes")]
  let _theme = if _theme.is_empty() {
    Vec::new()
  } else {
    parsed.iter_mut().map(|l| l.as_style()).collect::<Vec<_>>()
  };
  #[cfg(feature = "themes")]
  let theme = if _theme.is_empty() {
    constants::DEFAULT_THEME
  } else {
    _theme.as_slice() as crate::themes::Theme
  };
  #[cfg(not(feature = "themes"))]
  let theme = constants::DEFAULT_THEME;

  if args.is_empty() {
    args.push(String::from("."));
  }
  for arg in &args {
    let path_to_scan = path::Path::new(arg.as_str()).canonicalize().die("Invalid or inexistent path", &flags);
    let path_to_scan = path_to_scan.as_path();

    if !path_to_scan.exists() {
      println!("File or directory does not exist");
      std::process::exit(1);
    }

    if path_to_scan.is_file() {
      print_item(path_to_scan, path::PathBuf::from(path_to_scan), &theme, &flags, true);
    // Only situation where single_item will be true
    } else if flags.tree {
      do_scan(path_to_scan, path_to_scan, &theme, &flags);
    } else {
      if args.len() > 1 {
        let newline: &str;
        if args.iter().position(|a| a == arg).die("Unexpected error when listing directory", &flags) == 0 {
          newline = "";
        } else {
          newline = "\n";
        }
        println!("{}", format!("{newline}Folder {arg}:").underline(&flags).bright(&flags).reset(&flags));
      }
      for entry in fs::read_dir(path_to_scan).die("Directory cannot be accessed", &flags) {
        let path = entry.die("Failed retrieving path", &flags).path();
        print_item(path_to_scan, path, &theme, &flags, false);
      }
    }
  }
}

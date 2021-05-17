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

use crate::core::*;
use prelude::*;

#[cfg(target_os = "windows")]
#[cfg(feature = "color")]
#[link(name = "color")]
extern "C" {
  fn enable_color();
}

fn print_item(root: &path::Path, path: path::PathBuf, flags: &args::Flags, single_item: bool) {
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
      color = "".white(&flags);
      suffix += format!(" [{}]", file_detection::get_license(path.as_path(), flags)).grey(flags).as_str();
    } else {
      color = file_detection::file_extension_color(&path, flags);
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

  suffix += String::new().reset(&flags).as_str();
  println!(
    "{}{}{}{}{}{}{}",
    prefix,
    (3..indentation * 3).map(|_| " ").collect::<String>(),
    if indentation > 0 { "└──" } else { "" },
    if !color.is_empty() { "".bright(flags) } else { String::new() },
    name_prefix.custom(color.as_str(), flags),
    item_name.reset(flags),
    suffix,
  );
}

fn do_scan(root: &path::Path, path_to_scan: &path::Path, flags: &args::Flags) {
  if !flags.all && file_detection::is_hidden(&path_to_scan.to_path_buf(), flags) {
    return;
  }

  let item_name = OsStr::new(path_to_scan.file_name().die("Unable to get file name", flags))
    .to_str()
    .die("Unable to parse file name", flags);

  if path_to_scan.is_file() {
    let path = path::PathBuf::from(path_to_scan);
    print_item(root, path, flags, false);
  } else {
    if !flags.all && flags.tree && constants::COLLAPSED_DIRECTORIES.contains(&item_name) {
      return;
    }

    for entry in fs::read_dir(path_to_scan).die("Directory cannot be accessed", flags) {
      let path = entry.die("Failed retrieving path", flags).path();

      print_item(root, path.clone(), flags, false);

      if path.is_dir() {
        do_scan(root, path.as_path(), flags);
      }
    }
  }
}

fn main() {
  let (flags, mut args) = args::get();

  #[cfg(target_os = "windows")]
  #[cfg(feature = "color")]
  if !flags.no_color {
    unsafe {
      enable_color();
    }
  }

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
      print_item(path_to_scan, path::PathBuf::from(path_to_scan), &flags, true);
    // Only situation where single_item will be true
    } else if flags.tree {
      do_scan(path_to_scan, path_to_scan, &flags);
    } else {
      if args.len() > 1 {
        let newline: &str;
        if args.iter().position(|a| a == arg).die("Unexpected error when listing directory", &flags) == 0 {
          newline = "";
        } else {
          newline = "\n";
        }
        println!("{}", format!("{}Folder {}:", newline, arg).underline(&flags).bright(&flags));
      }
      for entry in fs::read_dir(path_to_scan).die("Directory cannot be accessed", &flags) {
        let path = entry.die("Failed retrieving path", &flags).path();
        print_item(path_to_scan, path, &flags, false);
      }
    }
  }
}

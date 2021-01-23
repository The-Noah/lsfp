use std::ffi::OsStr;
use std::fs;
use std::path;

mod color;
mod constants;
mod core;
mod file_detection;
mod modules;

use crate::core::*;
use color::*;

fn print_item(root: &path::Path, path: path::PathBuf, flags: &args::Flags) {
  if !flags.all && file_detection::is_hidden(&path) {
    return;
  }
  let mut color = if path.is_dir() { color::CYAN } else { "" };

  let mut prefix = String::new();
  let mut name_prefix = String::new();
  let mut suffix = String::new();

  let item_name = match path.file_name() {
    Some(val) => OsStr::new(val).to_str().unwrap_or("??"),
    None => "??",
  };

  let file_path = match path.strip_prefix(root) {
    Ok(val) => OsStr::new(val).to_str().unwrap_or("??"),
    Err(err) => panic!(err),
  };

  let mut indentation: u32 = 0;
  if flags.tree {
    let mut parent_path = path.parent().unwrap().to_path_buf();
    while parent_path != root {
      parent_path = parent_path.parent().unwrap().to_path_buf();
      indentation += 1;
    }
  }

  if flags.size {
    let size;
    match path.metadata() {
      Ok(metadata) => size = metadata.len(),
      Err(_) => size = 0,
    }

    prefix += format!("{} ", modules::file_size::human_readable_size(size))
      .bright(flags)
      .grey(flags)
      .reset(flags)
      .as_str();
  }

  let the_color_so_it_lives: String; // HACK to make the borrow checker happy
  if path.is_file() {
    if item_name.to_lowercase().starts_with("license") {
      the_color_so_it_lives = "".to_owned().white(&flags);
      color = the_color_so_it_lives.as_str();
      suffix += format!(" [{}]", file_detection::get_license(path.as_path())).grey(flags).as_str();
    } else {
      the_color_so_it_lives = file_detection::file_extension_color(&path);
      color = the_color_so_it_lives.as_str();
    }

    // file path for git
    let mut final_path = root.clone().to_path_buf();
    if !root.is_file() {
      final_path.push(file_path);
    }

    // file changed (git)
    if !flags.no_git && modules::git::changed(&final_path) {
      suffix += format!(
        " {}{}{}",
        "[".to_owned().grey(flags).reset(flags),
        "M".to_owned().bright(flags).orange(flags).reset(flags),
        "]".to_owned().grey(flags).reset(flags)
      )
      .as_str();
    }
  } else if !flags.all && constants::COLLAPSED_DIRECTORIES.contains(&item_name) {
    name_prefix = String::new().underline(&flags);
  }

  suffix += String::new().reset(&flags).as_str();
  println!(
    "{}{}{}{}{}{}{}",
    prefix,
    (3..indentation * 3).map(|_| " ").collect::<String>(),
    if indentation > 0 { "└──" } else { "" },
    if !color.is_empty() { "".to_owned().bright(&flags) } else { String::new() },
    name_prefix.custom(color, &flags),
    item_name,
    suffix
  );
}

fn do_scan(root: &path::Path, path_to_scan: &path::Path, flags: &args::Flags) {
  if !flags.all && file_detection::is_hidden(&path_to_scan.to_path_buf()) {
    return;
  }
  let item_name = match path_to_scan.file_name() {
    Some(val) => OsStr::new(val).to_str().unwrap_or("??"),
    None => "??",
  };

  if path_to_scan.is_file() {
    let path = path::PathBuf::from(path_to_scan);
    print_item(root, path, flags);
  } else {
    if !flags.all && constants::COLLAPSED_DIRECTORIES.contains(&item_name) {
      return;
    }

    for entry in fs::read_dir(path_to_scan).unwrap() {
      let path = entry.unwrap().path();

      print_item(root, path.clone(), flags);

      if path.is_dir() {
        do_scan(root, path.as_path(), flags);
      }
    }
  }
}

fn main() {
  let (flags, mut args) = args::get();

  let raw_path = args.pop().unwrap_or(String::from("."));
  let path_to_scan = path::Path::new(raw_path.as_str());

  if !path_to_scan.exists() {
    println!("File or directory does not exist");
    std::process::exit(0);
  }

  if path_to_scan.is_file() {
    print_item(path_to_scan, path::PathBuf::from(path_to_scan), &flags);
  } else {
    if flags.tree {
      do_scan(path_to_scan, path_to_scan, &flags);
    } else {
      let read_dir = fs::read_dir(path_to_scan);
      if read_dir.is_ok() {
        for entry in read_dir.unwrap() {
          let path = entry.unwrap().path();
          print_item(path_to_scan, path, &flags);
        }
      } else if read_dir.is_err() {
        println!(
          "{}",
          format!(
            "{} {}",
            "error:".to_owned().bright(&flags).red(&flags).reset(&flags),
            "this file or directory is hidden or cannot be accessed"
          )
        );
        std::process::exit(0);
      }
    }
  }
}

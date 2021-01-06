use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path;

mod color;
mod config;
mod constants;
mod file_detection;
mod git;
mod utils;

use color::*;

fn print_item(root: &path::Path, path: path::PathBuf, flags: &utils::Flags) {
  if !flags.all && file_detection::is_hidden(&path) {
    return;
  }
  let mut color = if path.is_dir() { color::CYAN } else { color::RESET };

  let mut prefix = String::new();
  let mut name_prefix = String::new();
  let mut suffix = String::new().grey(&flags);

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

    prefix += format!("{} ", utils::human_readable_size(size)).as_str();
  }

  let the_color_so_it_lives: String; // FIXME: plz 😭
  if path.is_file() {
    if item_name.to_lowercase().starts_with("license") {
      the_color_so_it_lives = "".to_owned().white(&flags);
      color = the_color_so_it_lives.as_str();
      suffix += format!(" [{}]", file_detection::get_license(path.as_path())).as_str();
    } else {
      the_color_so_it_lives = file_detection::file_extension_color(&path);
      color = the_color_so_it_lives.as_str();
    }
    // file status in git
    let mut final_path = root.clone().to_path_buf();
    if !root.is_file() {
      final_path.push(file_path);
    }

    // file changed (git)
    if !flags.no_git {
      let check = git::check(&final_path).unwrap_or((true, String::new()));
      if !check.0 && check.1 != "" {
        suffix += format!(
          " [{}{}+{}{}]",
          "".to_owned().bright(&flags),
          "".to_owned().green(&flags),
          "".to_owned().reset(&flags),
          "".to_owned().grey(&flags)
        )
        .as_str();
      }
    }
  } else if !flags.all && constants::COLLAPSED_DIRECTORIES.contains(&item_name) {
    name_prefix = String::new().underline(&flags);
  }

  suffix += String::from("").reset(&flags).as_str();
  println!(
    "{}{}{}{}{}{}{}{}",
    prefix,
    (3..indentation * 3).map(|_| " ").collect::<String>(),
    if indentation > 0 { "└──" } else { "" },
    if !color.is_empty() { "".to_owned().bright(&flags) } else { String::new() },
    name_prefix.custom(color, &flags),
    item_name,
    String::from("").reset(&flags),
    suffix
  );
}

fn do_scan(root: &path::Path, path_to_scan: &path::Path, flags: &utils::Flags) {
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
  let mut args: Vec<String> = env::args().collect();
  args.remove(0); // remove first arguement which is self

  let mut flags = utils::Flags {
    all: false,
    size: false,
    tree: false,
    no_color: env::var("NO_COLOR").is_ok(),
    no_git: false,
  };

  let mut args_to_remove = vec![];

  let mut print_help = false;
  let mut print_version = false;

  for (i, arg_str) in args.iter().enumerate() {
    let arg = arg_str.as_str();

    match arg {
      "-h" | "--help" => print_help = true,
      "-v" | "--version" => print_version = true,
      "-a" | "--all" => flags.all = true,
      "-s" | "--size" => flags.size = true,
      "-t" | "--tree" | "-r" | "--recursive" => flags.tree = true,
      "--no-color" => flags.no_color = true,
      "--no-git" => flags.no_git = true,
      _ => {
        if !config::parse_arg(arg) {
          continue;
        }
      }
    }

    args_to_remove.push(i);
  }

  // config options
  if !flags.no_git {
    flags.no_git = !config::get_bool("git", true);
  }
  if !flags.no_color {
    flags.no_color = !config::get_bool("color", true);
  }

  if print_help {
    utils::print_help(&flags);
    std::process::exit(0);
  } else if print_version {
    utils::print_name_version(&flags);
    std::process::exit(0);
  }

  let mut removed_count = 0;
  for arg_to_remove in args_to_remove {
    args.remove(arg_to_remove - removed_count);
    removed_count += 1;
  }

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
      for entry in fs::read_dir(path_to_scan).unwrap() {
        let path = entry.unwrap().path();
        print_item(path_to_scan, path, &flags);
      }
    }
  }
}

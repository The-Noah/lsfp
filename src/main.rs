use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path;

mod color;
mod constants;
mod file_detection;
mod utils;

fn print_item(root: &path::Path, path: path::PathBuf, flags: &utils::Flags) {
  let mut color = if path.is_dir() { color::CYAN } else { color::RESET };

  let mut prefix = String::new();
  let mut suffix = color::get_color(color::GREY, &flags);

  let file_name = match path.file_name() {
    Some(val) => OsStr::new(val).to_str().unwrap_or("??"),
    None => "??",
  };

  let mut indentation: u32 = 0;
  let mut parent_path = path.parent().unwrap().to_path_buf();
  while parent_path != root {
    parent_path = parent_path.parent().unwrap().to_path_buf();
    indentation += 1;
  }

  if !flags.all && file_name.chars().nth(0).unwrap() == '.' {
    return;
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
    if file_name.to_lowercase().starts_with("license") {
      the_color_so_it_lives = color::get_color(color::WHITE, &flags);
      color = the_color_so_it_lives.as_str();
      suffix += format!("[{}]", file_detection::get_license(path.as_path())).as_str();
    } else {
      the_color_so_it_lives = file_detection::file_extension_color(&path);
      color = the_color_so_it_lives.as_str();
    }
  }

  suffix += color::get_color(color::RESET, &flags).as_str();

  println!(
    "{}{}{}{}{}{}{} {}",
    prefix,
    (3..indentation * 3).map(|_| " ").collect::<String>(),
    if indentation > 0 { "└──" } else { "" },
    color::get_color(color::BRIGHT, &flags),
    color::get_color(color, &flags),
    file_name,
    color::get_color(color::RESET, &flags),
    suffix
  );
}

fn do_scan(root: &path::Path, path_to_scan: &path::Path, flags: &utils::Flags) {
  if path_to_scan.is_file() {
    let path = path::PathBuf::from(path_to_scan);
    print_item(root, path, flags);
  } else {
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
  };

  let mut args_to_remove = vec![];

  for (i, arg_str) in args.iter().enumerate() {
    let arg = arg_str.as_str();

    match arg {
      "-h" | "--help" => utils::print_help(),
      "-v" | "--version" => {
        utils::print_name_version();
        std::process::exit(0);
      }
      "-a" | "--all" => flags.all = true,
      "-s" | "--size" => flags.size = true,
      "-t" | "--tree" | "-r" | "--recursive" => flags.tree = true,
      "--no-color" => flags.no_color = true,
      _ => continue,
    }

    args_to_remove.push(i);
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

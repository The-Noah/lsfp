use std::env;
use std::ffi::OsStr;
use std::fs;

mod utils;

const COLOR_RESET: &str = "\x1b[0m";
const COLOR_BRIGHT: &str = "\x1b[1m";
const COLOR_CYAN: &str = "\x1b[36m";
const COLOR_GREEN: &str = "\x1b[32m";

struct Flags {
  all: bool,
  size: bool,
  no_color: bool,
}

fn print_item(path: std::path::PathBuf, flags: &Flags) {
  let color = if path.is_dir() { COLOR_CYAN } else { COLOR_GREEN };

  let mut extra = String::new();

  let file_name = match path.file_name() {
    Some(file_name) => OsStr::new(file_name).to_str().unwrap_or("??"),
    None => "??",
  };

  if !flags.all && file_name.chars().nth(0).unwrap() == '.' {
    return;
  }

  if flags.size {
    let size;
    match path.metadata() {
      Ok(metadata) => size = metadata.len(),
      Err(_) => size = 0,
    }

    extra += format!("{} ", utils::human_readable_size(size)).as_str();
  }

  println!(
    "{}{}{}{}{}",
    extra,
    if flags.no_color { "" } else { COLOR_BRIGHT },
    if flags.no_color { "" } else { color },
    file_name,
    if flags.no_color { "" } else { COLOR_RESET }
  );
}

fn main() {
  let mut args: Vec<String> = env::args().collect();
  args.remove(0); // remove first arguement which is self

  let mut flags = Flags {
    all: false,
    size: false,
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

  let path_to_scan = match args.pop() {
    Some(val) => val,
    None => String::from("."),
  };

  let mut paths = vec![];

  for entry in fs::read_dir(path_to_scan).unwrap() {
    let path = entry.unwrap().path();

    paths.push(path);
  }

  for path in paths {
    print_item(path, &flags);
  }
}

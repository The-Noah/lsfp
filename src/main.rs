use std::env;
use std::ffi::OsStr;
use std::fs;

const COLOR_RESET: &str = "\x1b[0m";
const COLOR_CYAN: &str = "\x1b[36m";
const COLOR_GREEN: &str = "\x1b[32m";

struct Flags {
  all: bool,
}

fn print_item(path: std::path::PathBuf, flags: &Flags) {
  let color = if path.is_dir() { COLOR_CYAN } else { COLOR_GREEN };

  let file_name = match path.file_name() {
    Some(file_name) => OsStr::new(file_name).to_str().unwrap_or("??"),
    None => "??",
  };

  if !flags.all && file_name.chars().nth(0).unwrap() == '.' {
    return;
  }

  println!("{}{}{}", color, file_name, COLOR_RESET);
}

fn print_help() {
  println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
  println!("{}", env!("CARGO_PKG_AUTHORS"));
  println!("{}", env!("CARGO_PKG_DESCRIPTION"));
  println!();
  println!("usage:");
  println!("    ls [options] [path]");
  println!();
  println!("flags:");
  println!("    -h, --help    Prints help information");
  println!();
  println!("options:");
  println!("    -a, --all    Shows all (hidden) files and directories");
}

fn main() {
  let mut args: Vec<String> = env::args().collect();
  args.remove(0); // remove first arguement which is self

  let mut flags = Flags { all: false };

  let mut args_to_remove = vec![];

  for (i, arg_str) in args.iter().enumerate() {
    let arg = arg_str.as_str();

    match arg {
      "-h" | "--help" => {
        print_help();
        std::process::exit(0);
      }
      "-a" | "--all" => flags.all = true,
      _ => continue,
    }

    args_to_remove.push(i);
  }

  for arg_to_remove in args_to_remove {
    args.remove(arg_to_remove);
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

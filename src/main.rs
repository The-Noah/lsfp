use std::env;
use std::ffi::OsStr;
use std::fs;

const COLOR_RESET: &str = "\x1b[0m";
const COLOR_CYAN: &str = "\x1b[36m";
const COLOR_GREEN: &str = "\x1b[32m";

const FILE_SIZE_WIDTH: usize = 5;

struct Flags {
  all: bool,
  size: bool,
}

fn human_readable_size(size: u64) -> String {
  if size < 1024 {
    // bytes
    return format!("{1:>0$}B", FILE_SIZE_WIDTH, size);
  } else if size < 1049000 {
    // kibibytes
    format!("{1:>0$.1}K", FILE_SIZE_WIDTH, size as f64 / 1024f64)
  } else if size < 1074000000 {
    // mebibytes
    format!("{1:>0$.1}M", FILE_SIZE_WIDTH, size as f64 / 1049000f64)
  } else {
    // gibibytes
    format!("{1:>0$.1}G", FILE_SIZE_WIDTH, size as f64 / 1074000000f64)
  }
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

    extra += format!("{} ", human_readable_size(size)).as_str();
  }

  println!("{}{}{}{}", extra, color, file_name, COLOR_RESET);
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
  println!("    -a, --all     Shows all (hidden) files and directories");
  println!("    -s, --size    Shows file sizes");
}

fn main() {
  let mut args: Vec<String> = env::args().collect();
  args.remove(0); // remove first arguement which is self

  let mut flags = Flags { all: false, size: false };

  let mut args_to_remove = vec![];

  for (i, arg_str) in args.iter().enumerate() {
    let arg = arg_str.as_str();

    match arg {
      "-h" | "--help" => {
        print_help();
        std::process::exit(0);
      }
      "-a" | "--all" => flags.all = true,
      "-s" | "--size" => flags.size = true,
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

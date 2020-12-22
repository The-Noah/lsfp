use std::env;
use std::ffi::OsStr;
use std::fs;

mod utils;

const COLOR_RESET: &str = "\x1b[0m";
const COLOR_BRIGHT: &str = "\x1b[1m";
const COLOR_CYAN: &str = "\x1b[36m";
const COLOR_WHITE: &str = "\x1b[37m";
const COLOR_GREY: &str = "\x1b[90m";

const FILE_EXTENSION_COLORS: &[(&[&str], (u8, u8, u8))] = &[
  (&["js"], (241, 224, 90)),                // JavaScript
  (&["ts"], (43, 116, 137)),                // TypeScript
  (&["cpp", "cxx", "hpp"], (243, 75, 125)), // C++
  (&["c", "h"], (85, 85, 85)),              // C
  (&["yaml, yml"], (203, 23, 30)),          // YAML
  (&["json"], (64, 212, 126)),              // JSON
  (&["rs"], (222, 165, 132)),               // Rust
  (&["php"], (79, 93, 149)),                // PHP
  (&["cs"], (23, 134, 0)),                  // C#
  (&["rb"], (112, 21, 22)),                 // Ruby
  (&["pl"], (2, 152, 195)),                 // Pearl
  (&["swift"], (255, 172, 69)),             // Switft
  (&["md", "markdown"], (8, 63, 161)),      // Markdown
  (&["py"], (53, 114, 165)),                // Python
  (&["html", "htm"], (227, 76, 38)),        // HTML
  (&["css"], (86, 61, 124)),                // CSS
  (&["scss"], (198, 83, 140)),              // SCSS
  (&["sass"], (165, 59, 112)),              // SASS
  (&["less"], (29, 54, 93)),                // Less
  (&["bat"], (193, 241, 46)),               // Batch
  (&["ps1", "psm1", "psd1"], (1, 36, 86)),  // Powershell
  (&["sh"], (137, 224, 81)),                // Shell
  (&["lua"], (0, 0, 128)),                  // LUA
];

const LICENSES: &[(&str, &str)] = &[
  ("MIT", "MIT License"),
  ("GPLv3", "GNU GENERAL PUBLIC LICENSE\nVersion 3"),
  ("GPLv2", "GNU GENERAL PUBLIC LICENSE\nVersion 2"),
  ("LGPLv3", "GNU LESSER GENERAL PUBLIC LICENSE\nVersion 3"),
  ("AGPLv3", "GNU AFFERO GENERAL PUBLIC LICENSE\nVersion 3"),
  ("Mozilla Public License 2.0", "Mozilla Public License Version 2.0"),
  ("Apache License 2.0", "Apache License\nVersion 2.0"),
  ("Boost Software License 1.0", "Boost Software License - Version 1.0"),
  ("The Unlicense", "This is free and unencumbered software released into the public domain."),
];

struct Flags {
  all: bool,
  size: bool,
  no_color: bool,
}

fn print_item(path: std::path::PathBuf, flags: &Flags) {
  let mut color = if path.is_dir() { COLOR_CYAN } else { COLOR_RESET };

  let mut prefix = String::new();
  let mut suffix = String::from(COLOR_GREY);

  let file_name = match path.file_name() {
    Some(val) => OsStr::new(val).to_str().unwrap_or("??"),
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

    prefix += format!("{} ", utils::human_readable_size(size)).as_str();
  }

  let the_color_so_it_lives: String; // FIXME: plz 😭
  if path.is_file() {
    if file_name.to_lowercase().starts_with("license") {
      let mut contents = fs::read_to_string(path.as_path()).expect("Something went wrong reading the file");
      contents = contents.replace("\r", "").trim().to_string();

      let mut final_contents = String::new();
      for line in contents.split("\n") {
        final_contents += &(line.trim().to_string() + "\n");
      }

      let mut license_type = "Custom";
      for license in LICENSES {
        if final_contents.starts_with(license.1) {
          color = COLOR_WHITE;
          license_type = license.0;
          break;
        }
      }

      suffix += format!("[{}]", license_type).as_str();
    } else {
      'extension_loop: for extension_color in FILE_EXTENSION_COLORS.iter() {
        for extension in extension_color.0 {
          if utils::extension_matches(&path, extension) {
            the_color_so_it_lives = format!("\x1b[38;2;{};{};{}m", extension_color.1 .0, extension_color.1 .1, extension_color.1 .2);
            color = the_color_so_it_lives.as_str();
            break 'extension_loop;
          }
        }
      }
    }
  }

  suffix += COLOR_RESET;

  println!(
    "{}{}{}{}{} {}",
    prefix,
    if flags.no_color { "" } else { COLOR_BRIGHT },
    if flags.no_color { "" } else { color },
    file_name,
    if flags.no_color { "" } else { COLOR_RESET },
    suffix
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

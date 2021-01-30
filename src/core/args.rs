#[cfg(feature = "config")]
use crate::core::config;

use crate::core::help;

pub struct Flags {
  pub all: bool,
  pub size: bool,
  pub tree: bool,
  #[cfg(feature = "color")]
  pub no_color: bool,
  #[cfg(feature = "git")]
  pub no_git: bool,
}

pub fn get() -> (Flags, Vec<String>) {
  let mut args: Vec<String> = std::env::args().collect();
  args.remove(0); // remove first arguement which is self

  let mut flags = Flags {
    all: false,
    size: false,
    tree: false,
    #[cfg(feature = "color")]
    no_color: std::env::var("NO_COLOR").is_ok(),
    #[cfg(feature = "git")]
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
      #[cfg(feature = "color")]
      "--no-color" => flags.no_color = true,
      #[cfg(feature = "git")]
      "--no-git" => flags.no_git = true,
      #[cfg(feature = "config")]
      _ => {
        if !config::parse_arg(arg) {
          continue;
        }
      }
      #[cfg(not(feature = "config"))]
      _ => {}
    }

    args_to_remove.push(i);
  }

  if print_help {
    help::print_help(&flags);
    std::process::exit(0);
  } else if print_version {
    help::print_name_version(&flags);
    std::process::exit(0);
  }

  // config options
  #[cfg(feature = "git")]
  if !flags.no_git {
    flags.no_git = !config::get_bool("git", true);
  }
  #[cfg(feature = "color")]
  if !flags.no_color {
    flags.no_color = !config::get_bool("color", true);
  }

  for (removed_count, arg_to_remove) in args_to_remove.iter().enumerate() {
    args.remove(arg_to_remove - removed_count);
  }

  (flags, args)
}

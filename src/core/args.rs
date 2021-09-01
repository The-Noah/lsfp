#[cfg(feature = "index_flags")]
use std::ops::{Index, IndexMut};

#[cfg(feature = "config")]
use crate::core::config;

use crate::core::help;

pub struct Flags {
  pub all: bool,
  pub size: bool,
  pub tree: bool,
  #[cfg(feature = "icons")]
  pub icons: bool,
  #[cfg(feature = "color")]
  pub no_color: bool,
  #[cfg(feature = "git")]
  pub no_git: bool,
  #[cfg(feature = "themes")]
  pub theme: Option<String>,
}

#[cfg(feature = "index_flags")]
// Only used for options that take value, which always is Option<String>
impl Index<&'_ str> for Flags {
  type Output = Option<String>;
  fn index(&self, s: &str) -> &Self::Output {
    match s {
      #[cfg(feature = "themes")]
      "theme" => &self.theme,
      _ => panic!("Cannot index field {} of flags", s),
    }
  }
}

#[cfg(feature = "index_flags")]
impl IndexMut<&'_ str> for Flags {
  fn index_mut(&mut self, s: &str) -> &mut Self::Output {
    match s {
      #[cfg(feature = "themes")]
      "theme" => &mut self.theme,
      _ => panic!("Cannot index field {} of flags", s),
    }
  }
}

pub fn get() -> (Flags, Vec<String>) {
  let mut args: Vec<String> = std::env::args()
    .skip(1) // remove first argument which is self
    .flat_map(|arg: String| {
      if arg.starts_with('-') && arg.get(1..2) != Some("-") && arg.get(1..).map(|text| text.len()) > Some(1) {
        arg.chars().skip(1).map(|c: char| format!("-{}", c)).collect::<Vec<String>>()
      } else {
        vec![arg]
      }
    })
    .collect();

  let mut flags = Flags {
    all: false,
    size: false,
    tree: false,
    #[cfg(feature = "icons")]
    icons: false,
    #[cfg(feature = "color")]
    no_color: std::env::var("NO_COLOR").is_ok(),
    #[cfg(feature = "git")]
    no_git: false,
    #[cfg(feature = "themes")]
    theme: None,
  };

  let mut args_to_remove = vec![];

  let mut print_help = false;
  let mut print_version = false;

  #[cfg(feature = "index_flags")]
  let mut takes_value: &str = "";

  for (i, arg_str) in args.iter().enumerate() {
    let arg = arg_str.as_str();

    #[cfg(feature = "index_flags")]
    if !takes_value.is_empty() {
      flags[takes_value] = Some(arg.to_owned());
      takes_value = "";
      args_to_remove.push(i);
      continue;
    }

    match arg {
      "-h" | "--help" => print_help = true,
      "-v" | "--version" => print_version = true,
      "-a" | "--all" => flags.all = true,
      "-s" | "--size" => flags.size = true,
      "-t" | "--tree" | "-r" | "--recursive" => flags.tree = true,
      #[cfg(feature = "icons")]
      "-i" | "--icons" => flags.icons = true,
      #[cfg(feature = "color")]
      "--no-color" => flags.no_color = true,
      #[cfg(feature = "git")]
      "--no-git" => flags.no_git = true,
      #[cfg(feature = "themes")]
      "--theme" => takes_value = "theme",
      #[cfg(feature = "config")]
      _ => {
        if !config::parse_arg(arg, &flags) {
          continue;
        }
      }
      #[cfg(not(feature = "config"))]
      _ => {
        continue;
      }
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
  #[cfg(feature = "icons")]
  if !flags.icons {
    flags.icons = config::get_bool("icons", false);
  }

  for (removed_count, arg_to_remove) in args_to_remove.iter().enumerate() {
    args.remove(arg_to_remove - removed_count);
  }

  (flags, args)
}

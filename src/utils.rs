use crate::color::*;

const FILE_SIZE_WIDTH: usize = 5;
const INDENT: &str = "    ";

pub struct Flags {
  pub all: bool,
  pub size: bool,
  pub tree: bool,
  pub no_color: bool,
  pub no_git: bool,
}

pub fn print_name_version(flags: &Flags) {
  println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION").to_owned().green(flags).reset(flags));
}

pub fn print_help(flags: &Flags) {
  print_name_version(flags);
  println!("{}", format!("Created by {}", env!("CARGO_PKG_AUTHORS")).grey(flags).reset(flags));
  println!("{}", env!("CARGO_PKG_DESCRIPTION").to_owned().grey(flags).reset(flags));
  println!();
  println!("{}", "Usage:".to_owned().orange(flags).reset(flags));
  println!("{}{} [options] [arguments]", INDENT, env!("CARGO_PKG_NAME"));
  println!();
  println!("{}", "Options:".to_owned().orange(flags).reset(flags));
  println!("{}{}     {}Print help information", INDENT, "-h, --help".to_owned().green(flags).reset(flags), INDENT);
  println!("{}{}  {}Print version", INDENT, "-v, --version".to_owned().green(flags).reset(flags), INDENT);
  println!(
    "{}{}      {}Show all (hidden) files and directories",
    INDENT,
    "-a, --all".to_owned().green(flags).reset(flags),
    INDENT
  );
  println!("{}{}     {}Show file sizes", INDENT, "-s, --size".to_owned().green(flags).reset(flags), INDENT);
  println!(
    "{}{}     {}Show output as a tree (recursive)",
    INDENT,
    "-t, --tree".to_owned().green(flags).reset(flags),
    INDENT
  );
  println!(
    "{}{}{}Alias for {}",
    INDENT,
    "-r, --recursive".to_owned().green(flags).reset(flags),
    INDENT,
    "--tree".to_owned().orange(flags).reset(flags)
  );
  println!(
    "{}    {} {}Do not output any color (automatically set with NO_COLOR env)",
    INDENT,
    "--no-color".to_owned().green(flags).reset(flags),
    INDENT
  );
  println!("{}    {}   {}Do not use git integration", INDENT, "--no-git".to_owned().green(flags).reset(flags), INDENT);
  println!();
  println!("{}", "Arguments:".to_owned().orange(flags).reset(flags));
  println!(
    "{}{}{}Path to run in {}",
    INDENT,
    "path".to_owned().green(flags).reset(flags),
    INDENT,
    "[default: .]".to_owned().orange(flags).reset(flags)
  );
}

pub fn human_readable_size(size: u64) -> String {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn b() {
    assert_eq!("   50B", human_readable_size(50));
  }

  #[test]
  fn kb() {
    assert_eq!("  1.0K", human_readable_size(1024));
  }

  #[test]
  fn mb() {
    assert_eq!("150.2M", human_readable_size(157600000));
  }

  #[test]
  fn gb() {
    assert_eq!("1000.0G", human_readable_size(1074000000000));
  }
}

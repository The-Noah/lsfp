const FILE_SIZE_WIDTH: usize = 5;

pub struct Flags {
  pub all: bool,
  pub size: bool,
  pub tree: bool,
  pub no_color: bool,
  pub no_git: bool,
}

pub fn print_name_version() {
  println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

pub fn print_help() {
  print_name_version();
  println!("{}", env!("CARGO_PKG_AUTHORS"));
  println!("{}", env!("CARGO_PKG_DESCRIPTION"));
  println!();
  println!("USAGE:");
  println!("    {} [OPTIONS] [path]", env!("CARGO_PKG_NAME"));
  println!();
  println!("FLAGS:");
  println!("    -h, --help       Print help information");
  println!("    -v, --version    Print version");
  println!();
  println!("OPTIONS:");
  println!("    -a, --all          Show all (hidden) files and directories");
  println!("    -s, --size         Show file sizes");
  println!("    -t, --tree         Show output as a tree (recursive)");
  println!("    -r, --recursive    Alias for --tree");
  println!("    --no-color         Do not output any color (automatically set with NO_COLOR env)");
  println!("    --no-git           Do not use git integration");
  println!();
  println!("ARGS:");
  println!("    path    Path to run in (defaults to .)");

  std::process::exit(0);
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

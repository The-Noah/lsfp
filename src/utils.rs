use crate::color;

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
  println!(
    "{} {}v{}{}",
    env!("CARGO_PKG_NAME"),
    color::get_color(color::GREEN, flags),
    env!("CARGO_PKG_VERSION"),
    color::get_color(color::RESET, flags)
  );
}

pub fn print_help(flags: &Flags) {
  print_name_version(flags);
  println!(
    "{}Created by {}{}",
    color::get_color(color::GREY, flags),
    env!("CARGO_PKG_AUTHORS"),
    color::get_color(color::RESET, flags)
  );
  println!(
    "{}{}{}",
    color::get_color(color::GREY, flags),
    env!("CARGO_PKG_DESCRIPTION"),
    color::get_color(color::RESET, flags)
  );
  println!();
  println!("{}Usage:{}", color::get_color(color::ORANGE, flags), color::get_color(color::RESET, flags));
  println!("{}{} [options] [arguments]", INDENT, env!("CARGO_PKG_NAME"));
  println!();
  println!("{}Options:{}", color::get_color(color::ORANGE, flags), color::get_color(color::RESET, flags));
  println!(
    "{}{}-h, --help     {}{}Print help information",
    INDENT,
    color::get_color(color::GREEN, flags),
    color::get_color(color::RESET, flags),
    INDENT
  );
  println!(
    "{}{}-v, --version  {}{}Print version",
    INDENT,
    color::get_color(color::GREEN, flags),
    color::get_color(color::RESET, flags),
    INDENT
  );
  println!(
    "{}{}-a, --all      {}{}Show all (hidden) files and directories",
    INDENT,
    color::get_color(color::GREEN, flags),
    color::get_color(color::RESET, flags),
    INDENT
  );
  println!(
    "{}{}-s, --size     {}{}Show file sizes",
    INDENT,
    color::get_color(color::GREEN, flags),
    color::get_color(color::RESET, flags),
    INDENT
  );
  println!(
    "{}{}-t, --tree     {}{}Show output as a tree (recursive)",
    INDENT,
    color::get_color(color::GREEN, flags),
    color::get_color(color::RESET, flags),
    INDENT
  );
  println!(
    "{}{}-r, --recursive{}{}Alias for {}--tree{}",
    INDENT,
    color::get_color(color::GREEN, flags),
    color::get_color(color::RESET, flags),
    INDENT,
    color::get_color(color::ORANGE, flags),
    color::get_color(color::RESET, flags),
  );
  println!(
    "{}{}    --no-color {}{}Do not output any color (automatically set with NO_COLOR env)",
    INDENT,
    color::get_color(color::GREEN, flags),
    color::get_color(color::RESET, flags),
    INDENT
  );
  println!(
    "{}{}    --no-git   {}{}Do not use git integration",
    INDENT,
    color::get_color(color::GREEN, flags),
    color::get_color(color::RESET, flags),
    INDENT
  );
  println!();
  println!("{}Arguments:{}", color::get_color(color::ORANGE, flags), color::get_color(color::RESET, flags));
  println!(
    "{}{}path{}{}Path to run in {}[default: .]{}",
    INDENT,
    color::get_color(color::GREEN, flags),
    color::get_color(color::RESET, flags),
    INDENT,
    color::get_color(color::ORANGE, flags),
    color::get_color(color::RESET, flags)
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

use crate::color::*;
use crate::core::*;

const INDENT: &str = "    ";

pub fn print_name_version(flags: &args::Flags) {
  println!(
    "{} v{}",
    env!("CARGO_PKG_NAME").to_owned().bright(flags).reset(flags),
    env!("CARGO_PKG_VERSION").to_owned().green(flags).reset(flags)
  );
}

pub fn print_help(flags: &args::Flags) {
  print_name_version(flags);
  println!("{}", env!("CARGO_PKG_DESCRIPTION"));
  println!("{}", format!("Created by {}", env!("CARGO_PKG_AUTHORS")).grey(flags).reset(flags));
  println!();
  println!("{}", "Usage:".to_owned().yellow(flags).reset(flags));
  println!("{}{} [options] [arguments]", INDENT, env!("CARGO_PKG_NAME"));
  println!();
  println!("{}", "Options:".to_owned().yellow(flags).reset(flags));
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
    "--tree".to_owned().yellow(flags).reset(flags)
  );
  #[cfg(feature = "color")]
  println!(
    "{}    {} {}Do not output any color (automatically set with NO_COLOR env)",
    INDENT,
    "--no-color".to_owned().green(flags).reset(flags),
    INDENT
  );
  #[cfg(feature = "git")]
  println!("{}    {}   {}Do not use git integration", INDENT, "--no-git".to_owned().green(flags).reset(flags), INDENT);
  println!();
  println!("{}", "Arguments:".to_owned().yellow(flags).reset(flags));
  println!(
    "{}{}{}Path to run in {}",
    INDENT,
    "path".to_owned().green(flags).reset(flags),
    INDENT,
    "[default: .]".to_owned().yellow(flags).reset(flags)
  );
}

const FILE_SIZE_WIDTH: usize = 5;

pub fn print_name_version() {
  println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

pub fn print_help() {
  print_name_version();
  println!("{}", env!("CARGO_PKG_AUTHORS"));
  println!("{}", env!("CARGO_PKG_DESCRIPTION"));
  println!();
  println!("usage:");
  println!("    ls [options] [path]");
  println!();
  println!("flags:");
  println!("    -h, --help    Print help information");
  println!();
  println!("options:");
  println!("    -a, --all     Show all (hidden) files and directories");
  println!("    -s, --size    Show file sizes");

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

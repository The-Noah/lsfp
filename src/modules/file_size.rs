const FILE_SIZE_WIDTH: usize = 5;

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

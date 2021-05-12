use std::path;
use std::process::Command;

use crate::core::args::Flags;
use crate::die::Die;

pub fn changed(file_path: &path::Path, flags: &Flags) -> bool {
  let result = Command::new("git").arg("status").arg("--porcelain").arg(file_path).output();

  if result.is_err() {
    return false;
  }

  let final_result = result.die("Failed to run git", flags);

  !final_result.stdout.is_empty()
}

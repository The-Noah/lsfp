use std::path;
use std::process::Command;

pub fn changed(file_path: &path::Path) -> bool {
  let result = Command::new("git").arg("status").arg("--porcelain").arg(file_path).output();

  if result.is_err() {
    return false;
  }

  let final_result = result.expect("failed to run git");

  !final_result.stdout.is_empty()
}

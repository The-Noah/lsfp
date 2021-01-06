use std::path;
use std::process::Command;

pub fn check(file_path: &path::Path) -> bool {
  let result = Command::new("git").arg("diff").arg("--exit-code").arg(file_path).output();

  if result.is_err() {
    return false;
  }

  let final_result = result.expect("failed to run git");

  final_result.status.code() == Some(1)
}

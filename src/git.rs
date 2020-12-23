use std::process::Command;

pub fn command(args: &str) -> Result<(bool, String), ()> {
  let result = Command::new("git").arg(args).output();

  if result.is_err() {
    return Err(());
  }

  let final_result = result.expect("failed to run git");

  let ok = final_result.status.code() == Some(0);
  let stdout = String::from_utf8(final_result.stdout).unwrap();

  Ok((ok, stdout))
}

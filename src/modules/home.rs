use std::env;
use std::path::PathBuf;

/// Get home path. Returns [`None`] if no home path was found.
///
/// # Unix
///
/// - Returns the value of the 'HOME' environment variable if it is set
///   (including to an empty string).
/// - Otherwise, it tries to determine the home directory by invoking the `getpwuid_r` function
///   using the UID of the current user. An empty home directory field returned from the
///   `getpwuid_r` function is considered to be a valid value.
/// - Returns `None` if the current user has no entry in the /etc/passwd file.
///
/// # Windows
///
/// - Returns value of the 'USERPROFILE' environment variable if it is not empty.
///
pub fn home_dir() -> Option<PathBuf> {
  home_dir_inner()
}

#[cfg(windows)]
fn home_dir_inner() -> Option<PathBuf> {
  env::var_os("USERPROFILE").filter(|s| !s.is_empty()).map(PathBuf::from)
}

#[cfg(any(unix, target_os = "redox"))]
fn home_dir_inner() -> Option<PathBuf> {
  #[allow(deprecated)]
  env::home_dir()
}

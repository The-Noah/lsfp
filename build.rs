fn main() {
  #[cfg(target_os = "windows")]
  #[cfg(feature = "color")]
  cc::Build::new().file("color.c").compile("color");
}

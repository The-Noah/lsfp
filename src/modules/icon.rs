use crate::constants;

pub fn from(s: &str) -> String {
  std::char::from_u32(u32::from_str_radix(s, 16).unwrap_or_else(|_| u32::from_str_radix(constants::ICON_GENERIC, 16).unwrap()))
    .unwrap()
    .to_string()
}

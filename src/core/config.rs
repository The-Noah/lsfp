use std::fs;

use crate::core::args::Flags;
use crate::helper::Die;

const CONFIG_NAME: &str = ".lsfprc";
const ARG_PREFIX: &str = "--config-";

pub fn parse_arg(arg: &str, flags: &Flags) -> bool {
  if arg.starts_with(ARG_PREFIX) {
    let body = arg.strip_prefix(ARG_PREFIX).unwrap(); // strip argument prefix

    if body.contains('=') {
      let parts: Vec<&str> = body.split('=').collect();
      let name = parts[0];
      let value = parts[1];

      println!("set {}={}\n", name, value);
      let content_split = match fs::read_to_string(CONFIG_NAME) {
        Ok(x) => x,
        Err(_) => {
          fs::write(CONFIG_NAME, format!("{}={}", name, value)).die("Unable to save config file", flags);
          return true;
        }
      };
      let mut contents = content_split.split('\n').collect::<Vec<&str>>();

      let mut found = false;
      // HACK to make the borrow checker happy
      let edited_line;
      let edited_line2;

      for (i, line) in contents.iter().enumerate() {
        let final_line = &(line.replace("\r", "").trim().to_string());
        if final_line.starts_with(format!("{}=", name).as_str()) {
          found = true;

          edited_line = format!("{}={}", name, value);
          contents[i] = edited_line.as_str();

          break;
        }
      }

      // append to file
      if !found {
        edited_line2 = format!("{}={}", name, value);
        contents.push(edited_line2.as_str());
      }

      fs::write(CONFIG_NAME, contents.join("\n")).die("Unable to save config file", flags);
    } else {
      let contents = match fs::read_to_string(CONFIG_NAME) {
        Ok(x) => x,
        Err(_) => {
          println!("no config file found");
          return true;
        }
      }
      .replace("\r", "");

      let mut found = false;

      for line in contents.split('\n') {
        let final_line = &(line.trim().to_string());
        if final_line.starts_with(format!("{}=", body).as_str()) {
          found = true;

          let parts: Vec<&str> = final_line.split('=').collect();
          println!("{}: {}", body, parts[1]);

          break;
        }
      }

      if !found {
        println!("config {} not found", body);
      }
    }

    true
  } else {
    false
  }
}

pub fn get_bool(name: &str, default: bool) -> bool {
  let contents = match fs::read_to_string(CONFIG_NAME) {
    Ok(x) => x,
    Err(_) => return true,
  }
  .replace("\r", "");

  for line in contents.split('\n') {
    let final_line = &(line.trim().to_string());
    if final_line.starts_with(format!("{}=", name).as_str()) {
      let parts: Vec<&str> = final_line.split('=').collect();

      return parts[1] == "true";
    }
  }

  default
}

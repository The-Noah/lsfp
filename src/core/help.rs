use crate::color::*;
use crate::core::*;

const INDENT: &str = "    ";
const HELP_SECTIONS: &[(&str, &[Argument])] = &[
  (
    "Options",
    &[
      Argument {
        name: None,
        aliases: Some(&["h", "help"]),
        description: "Print help information",
      },
      Argument {
        name: None,
        aliases: Some(&["v", "version"]),
        description: "Print version",
      },
      Argument {
        name: None,
        aliases: Some(&["a", "all"]),
        description: "Show all (hidden) files and directories",
      },
      Argument {
        name: None,
        aliases: Some(&["s", "size"]),
        description: "Show file sizes",
      },
      Argument {
        name: None,
        aliases: Some(&["t", "r", "tree", "recursive"]),
        description: "Show output as a tree (recursive)",
      },
      #[cfg(feature = "icons")]
      Argument {
        name: None,
        aliases: Some(&["i", "icons"]),
        description: "Show icons before filename, requires Nerd Font",
      },
      #[cfg(feature = "color")]
      Argument {
        name: None,
        aliases: Some(&["no-color"]),
        description: "Do not output any color (automatically set with NO_COLOR env)",
      },
      #[cfg(feature = "git")]
      Argument {
        name: None,
        aliases: Some(&["no-git"]),
        description: "Do not use git integration",
      },
      #[cfg(feature = "themes")]
      Argument {
        name: None,
        aliases: Some(&["theme <name|path>"]),
        description: "Name or path of theme to use.",
      },
    ],
  ),
  #[cfg(feature = "config")]
  (
    "Config",
    &[
      #[cfg(feature = "color")]
      Argument {
        name: None,
        aliases: Some(&["config-color=<true|false>"]),
        description: "Control colored output",
      },
      #[cfg(feature = "git")]
      Argument {
        name: None,
        aliases: Some(&["config-git=<true|false>"]),
        description: "Control git integration",
      },
      #[cfg(feature = "icons")]
      Argument {
        name: None,
        aliases: Some(&["config-icons=<true|false>"]),
        description: "Control icons",
      },
    ],
  ),
  (
    "Arguments",
    &[Argument {
      name: Some("path"),
      aliases: None,
      description: "Path to run in [default: .]",
    }],
  ),
];

struct Argument<'a> {
  name: Option<&'a str>,
  aliases: Option<&'a [&'a str]>,
  description: &'a str,
}

pub fn print_name_version(flags: &args::Flags) {
  println!(
    "{} {}{}",
    env!("CARGO_PKG_NAME").bright(flags).reset(flags),
    "v".green(flags).reset(flags),
    env!("CARGO_PKG_VERSION").green(flags).reset(flags)
  );
}

pub fn print_help(flags: &args::Flags) {
  print_name_version(flags);
  println!("{}", env!("CARGO_PKG_DESCRIPTION"));
  println!("{}", format!("Created by {}", env!("CARGO_PKG_AUTHORS")).grey(flags).reset(flags));
  println!();
  println!("{}:", "Usage".bright(flags).yellow(flags).reset(flags));
  println!("{}{} {}[options] [arguments]{}", INDENT, env!("CARGO_PKG_NAME"), "".yellow(flags), "".reset(flags));

  for help_section in HELP_SECTIONS {
    println!();
    println!("{}:", help_section.0.bright(flags).yellow(flags).reset(flags));

    let mut formatted_aliases = vec![];
    let mut max_alias_text_length = 0;

    for argument in help_section.1 {
      let mut aliases = vec![];
      if argument.aliases.is_some() {
        for alias in argument.aliases.unwrap().iter() {
          aliases.push(format!("-{}{}", if alias.len() > 1 { "-" } else { "" }, alias));
        }
      }

      let formatted_alias = aliases.join(", ");
      let length = formatted_alias.len();
      formatted_aliases.push(formatted_alias);

      if length > max_alias_text_length {
        max_alias_text_length = length;
      }
    }

    for (i, argument) in help_section.1.iter().enumerate() {
      println!(
        "{}{}{}{}{}{}",
        INDENT,
        argument.name.unwrap_or("").green(flags).reset(flags),
        &formatted_aliases[i].to_string().green(flags).reset(flags),
        (0..max_alias_text_length - formatted_aliases[i].len()).map(|_| " ").collect::<String>(),
        INDENT,
        argument
          .description
          .replace("[", "[".yellow(flags).as_str())
          .replace("]", "]".reset(flags).as_str())
          .reset(flags)
      );
    }
  }
}

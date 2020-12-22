pub const FILE_EXTENSION_COLORS: &[(&[&str], (u8, u8, u8))] = &[
  (&["js"], (241, 224, 90)),                // JavaScript
  (&["ts"], (43, 116, 137)),                // TypeScript
  (&["cpp", "cxx", "hpp"], (243, 75, 125)), // C++
  (&["c", "h"], (85, 85, 85)),              // C
  (&["yaml", "yml"], (203, 23, 30)),        // YAML
  (&["json"], (64, 212, 126)),              // JSON
  (&["rs"], (222, 165, 132)),               // Rust
  (&["php"], (79, 93, 149)),                // PHP
  (&["cs"], (23, 134, 0)),                  // C#
  (&["rb"], (112, 21, 22)),                 // Ruby
  (&["pl"], (2, 152, 195)),                 // Pearl
  (&["swift"], (255, 172, 69)),             // Switft
  (&["md", "markdown"], (8, 63, 161)),      // Markdown
  (&["py"], (53, 114, 165)),                // Python
  (&["html", "htm"], (227, 76, 38)),        // HTML
  (&["css"], (86, 61, 124)),                // CSS
  (&["scss"], (198, 83, 140)),              // SCSS
  (&["sass"], (165, 59, 112)),              // SASS
  (&["less"], (29, 54, 93)),                // Less
  (&["bat"], (193, 241, 46)),               // Batch
  (&["ps1", "psm1", "psd1"], (1, 36, 86)),  // Powershell
  (&["sh"], (137, 224, 81)),                // Shell
  (&["lua"], (0, 0, 128)),                  // LUA
];

pub const LICENSES: &[(&str, &str)] = &[
  ("MIT", "MIT License"),
  ("GPLv3", "GNU GENERAL PUBLIC LICENSE\nVersion 3"),
  ("GPLv2", "GNU GENERAL PUBLIC LICENSE\nVersion 2"),
  ("LGPLv3", "GNU LESSER GENERAL PUBLIC LICENSE\nVersion 3"),
  ("AGPLv3", "GNU AFFERO GENERAL PUBLIC LICENSE\nVersion 3"),
  ("Mozilla Public License 2.0", "Mozilla Public License Version 2.0"),
  ("Apache License 2.0", "Apache License\nVersion 2.0"),
  ("Boost Software License 1.0", "Boost Software License - Version 1.0"),
  ("The Unlicense", "This is free and unencumbered software released into the public domain."),
];

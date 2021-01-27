// https://github.com/ozh/github-colors/blob/master/colors.json
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
  (&["swift"], (255, 172, 69)),             // Swift
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
  (&["java"], (176, 114, 25)),              // Java
  (&["m"], (67, 142, 255)),                 // Objective-C
];

// https://choosealicense.com/
// https://opensource.org/licenses/
pub const LICENSES: &[(&str, &str)] = &[
  ("MIT", "MIT License"),
  ("MIT-0", "MIT No Attribution License"),
  ("GPL-3.0", "GNU GENERAL PUBLIC LICENSE\nVersion 3"),
  ("GPL-2.0", "GNU GENERAL PUBLIC LICENSE\nVersion 2"),
  ("LGPL-2.1", "GNU LESSER GENERAL PUBLIC LICENSE\n Version 2.1")
  ("LGPL-3.0", "GNU LESSER GENERAL PUBLIC LICENSE\nVersion 3"),
  ("AGPL-3.0", "GNU AFFERO GENERAL PUBLIC LICENSE\nVersion 3"),
  ("MPL-2.0", "Mozilla Public License Version 2.0"),
  ("Apache-2.0", "Apache License\nVersion 2.0"),
  ("AFL-3.0", "Academic Free License 3.0"),
  ("APSL-2.0", "Apple Public Source License"),
  ("APL-1.0", "Adaptive Public License 1.0"),
  ("BSL-1.0", "Boost Software License - Version 1.0"),
  ("Unlicense", "This is free and unencumbered software released into the public domain."),
];

pub const COLLAPSED_DIRECTORIES: &[&str] = &[
  ".git",
  "target",
  "node_modules",
  "build",
  "Build",
  "dist",
  "obj",
  "bin",
  "tmp",
  "temp",
  "__pycache__",
  "cache",
  "debug",
];

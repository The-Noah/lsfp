type FileExtensionColor<'a> = (&'a [&'a str], (u8, u8, u8), &'a str);

// https://github.com/ozh/github-colors/blob/master/colors.json
pub const FILE_EXTENSION_COLORS: &[FileExtensionColor] = &[
  (&["js"], (241, 224, 90), "\u{e60c}"),                // JavaScript
  (&["ts"], (43, 116, 137), "\u{e628}"),                // TypeScript
  (&["cpp", "cxx", "hpp"], (243, 75, 125), "\u{fb71}"), // C++
  (&["c", "h"], (85, 85, 85), "\u{e61e}"),              // C (alternative: fb70 [same design as c++'s, but is too big and inconsistent])
  (&["yaml", "yml"], (203, 23, 30), "\u{e60b}"),        // YAML
  (&["json"], (64, 212, 126), "\u{e60b}"),              // JSON
  (&["rs"], (222, 165, 132), "\u{e7a8}"),               // Rust
  (&["php"], (79, 93, 149), "\u{e73d}"),                // PHP
  (&["cs"], (23, 134, 0), "\u{f81a}"),                  // C#
  (&["rb"], (112, 21, 22), "\u{e791}"),                 // Ruby
  (&["pl"], (2, 152, 195), "\u{f977}"),                 // Pearl (generic script icon, alternatives: `code` icon)
  (&["swift"], (255, 172, 69), "\u{e755}"),             // Swift
  (&["md", "markdown"], (8, 63, 161), "\u{f853}"),      // Markdown
  (&["py"], (53, 114, 165), "\u{e73c}"),                // Python
  (&["html", "htm"], (227, 76, 38), "\u{e736}"),        // HTML
  (&["css"], (86, 61, 124), "\u{e74a}"),                // CSS
  (&["scss"], (198, 83, 140), "\u{e74b}"),              // SCSS
  (&["sass"], (165, 59, 112), "\u{e74b}"),              // SASS
  (&["less"], (29, 54, 93), "\u{e758}"),                // Less
  (&["bat"], (193, 241, 46), "\u{f120}"),               // Batch (generic terminal, alternatives: e795, e7a2)
  (&["ps1", "psm1", "psd1"], (1, 36, 86), "\u{f489}"),  // Powershell
  (&["sh"], (137, 224, 81), "\u{f120}"),                // Bash
  (&["lua"], (0, 0, 128), "\u{e620}"),                  // LUA
  (&["java"], (176, 114, 25), "\u{e738}"),              // Java
  (&["m"], (67, 142, 255), "\u{e711}"),                 // Objective-C (generic apple logo)
];

#[cfg(feature = "icons")]
pub const ICON_FOLDER_OPEN: &str = "\u{f115}";
#[cfg(feature = "icons")]
pub const ICON_FOLDER_CLOSED: &str = "\u{f114}";
#[cfg(feature = "icons")]
pub const ICON_LICENSE: &str = "\u{f623}";
#[cfg(feature = "icons")]
pub const ICON_GENERIC: &str = "\u{f723}";

// https://choosealicense.com/
// https://opensource.org/licenses/
// https://spdx.org/licenses/
// LICENSES LIST: ("SPDX identifier", "Start of the license file/text")
//  Licenses are ordered alphabetically based on their SPDX identifier
pub const LICENSES: &[(&str, &str)] = &[
  ("Abstyles", "This is APREAMBL.TEX, version 1.10e, written by Hans-Hermann Bode"),
  ("Adobe-2006", "Adobe Systems Incorporated(r) Source Code License Agreement"),
  ("Adobe-Glyph", "Copyright (c) 1997,1998,2002,2007 Adobe Systems Incorporated"),
  ("ADSL", "This software code is made available \"AS IS\" without warranties of any kind. You may copy, display, modify and redistribute the software code either by itself or as incorporated into your code;"),
  ("AFL-1.1", "Academic Free License\nVersion 1.1"),
  ("AFL-1.2", "Academic Free License\nVersion 1.2"),
  ("AFL-2.0", "The Academic Free License\nv. 2.0"), // ! Depending on the source, there is a space between the newline and the 'v. 2.0'
  ("AFL-2.1", "The Academic Free License\nv.2.1"),
  ("AFL-3.0", "Academic Free License (\"AFL\") v. 3.0"),
  ("Afmparse", "(C) 1988, 1989 by Adobe Systems Incorporated. All rights reserved."),
  ("AGPL-3.0", "GNU AFFERO GENERAL PUBLIC LICENSE\nVersion 3"),
  ("Apache-1.0", "Copyright (c) 1995-1999 The Apache Group.  All rights reserved."),
  ("Apache-1.1", "The Apache Software License, Version 1.1"),
  ("Apache-2.0", "Apache License\nVersion 2.0"),
  ("APL-1.0", "ADAPTIVE PUBLIC LICENSE\nVersion 1.0"),
  ("APSL-1.0", "APPLE PUBLIC SOURCE LICENSE\nVersion 1.0"),
  ("APSL-1.1", "APPLE PUBLIC SOURCE LICENSE\nVersion 1.1"),
  ("APSL-1.2", "APPLE PUBLIC SOURCE LICENSE\nVersion 1.2"), // Could not find license file, start induced from the other
  ("APSL-2.0", "APPLE PUBLIC SOURCE LICENSE\nVersion 2.0"),
  ("Artistic-1.0", "The Artistic License\n\nPreamble"),
  ("Artistic-2.0", "Copyright (c) 2000-2006, The Perl Foundation."),
  ("BitTorrent-1.0", "BitTorrent Open Source License\nVersion 1.0"),
  ("BitTorrent-1.1", "BitTorrent Open Source License\nVersion 1.1"),
  ("BSL-1.0", "Boost Software License - Version 1.0"),
  ("ClArtistic", "The Clarified Artistic License\n\nPreamble"),
  ("EPL-1.0", "Eclipse Public License - v 1.0"),
  ("EPL-2.0", "Eclipse Public License - v 2.0"),
  ("ErlPL-1.1", "ERLANG PUBLIC LICENSE\nVersion 1.1"),
  ("EUDatagrid", "Copyright (c) 2001 EU DataGrid. All rights reserved."),
  ("EUPL-1.0", "European Union Public Licence V.1.0"),
  ("EUPL-1.1", "European Union Public Licence v. 1.1"),
  ("EUPL-1.2", "EUROPEAN UNION PUBLIC LICENCE v. 1.2"),
  ("GPL-2.0", "GNU GENERAL PUBLIC LICENSE\nVersion 2"),
  ("GPL-3.0", "GNU GENERAL PUBLIC LICENSE\nVersion 3"),
  ("LGPL-2.1", "GNU LESSER GENERAL PUBLIC LICENSE\nVersion 2.1"),
  ("LGPL-3.0", "GNU LESSER GENERAL PUBLIC LICENSE\nVersion 3"),
  ("MIT", "MIT License"),
  ("MIT-0", "MIT No Attribution"),
  ("MPL-2.0", "Mozilla Public License Version 2.0"),
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

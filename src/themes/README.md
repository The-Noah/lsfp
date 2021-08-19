# Themes for `lsfp`

This module contains support for custom styling in `lsfp`. This custom styling is based on _themes_, which are files that tell `lsfp` what style apply to each extension. You can easily modify and distribute theme files, which can be used by any `lsfp` installation. This file guides you through the path of building a theme file and using it with `lsfp`. In case you are looking for the syntax rules of theme files, read the [formal specification](https://github.com/The-Noah/lsfp/blob/master/src/themes/SPEC.md).

## The theme file syntax

A theme file consists of multiple sections, representing languages and/or extensions that should be styled with the same color or icon. This sections are delimited by a line starting with a dash (`-`), taking the next lines until a new dash. This dash **MUST** be the first character on the line (only leading spaces can be present), but as the line is only used as an indicator of a new section start, everything after that dash is ignored, meaning that you can write as many dashes as you may want or write the language name or some comments on there.

Inside each section, there are different key-value pairs, being the _extensions_ key the most important one, as it is the one that the program will use to recognize files and styles associated. As of today, there are only three keys available, which also depend on the features that you compiled `lsfp` with: **extensions** (primary key which works across al features), **color** (an RGB color for the extensions, only available under _color_ feature) and **icon** (an hexadecimal value made up of 4 characters), only available under _icons_ feature). Keys and values are written as `<key>=<value>` or `<key>:<value>`. All empty lines are ignored, and a key without a value or line that does not match the `<key><:|=><value>` pattern will throw an error, while an unrecognized key will not be of any problem (this ensures full compatibility between binaries compiled with different features).

Here is an example theme file:

```
--- JS (mjs for modules with node).
extensions=js,mjs # TODO Add more
color=255,220,0
icon=f898

--- TS Maybe there are more extensions?
extensions=ts
color=0,31,63

icon=e628 # Don't quite like it

-- IMPORTANT Rust, don't erase this by any means!
color=255,65,54
extensions=rs
icon=e7a8
```

There is a [formal specification](https://github.com/The-Noah/lsfp/blob/master/src/themes/SPEC.md) which explains in more depth the full syntax for the theme file. In case of any ambiguity, contradiction or doubt, the specification is the absolute source of true. When creating a theme file, it is recommended to first read this specification.

You can also find a test theme file in the root of the repository where the theme parser was first developed, called [test.theme](https://github.com/HipyCas/theme-parser/blob/master/test.theme) (extension is only for easy identification). This theme file is read by the main script for testing purposes, and you may want to try modify it and test it to see how the parser internally works, and test out your scripts.

## Features

Themes in `lsfp` are enabled under the feature _themes_, and therefore if the feature is not enabled for the build, no theme will be parsed/loaded in any case.

For the `colors` and `icon` key the features _color_ and _icons_ (respectively) must be enabled for the build. In case of those not being present, the parser will still give a valid language style, yet the color or icon will be empty and will not be printed by `lsfp` anyway.

In GitHub releases, all builds include all features except `lsfp-lite.exe` (Windows), which includes none. This means that all come with theme support builtin and support for all available settings (except the one mentioned before).

_Themes depend internally in other two features,_ index_flags *and* home, *which should never be manually toggled by the end user.*

## Named themes

When `lsfp` receives a value for the `--theme` flag that is not a path, it will try to load a _named theme_. A _named theme_ is simply a theme file located under `~/.lsfp-themes`, whose name is equivalent to the file name. That way, if you run `lsfp --theme noir` the theme file `~/.lsfp-themes/noir` will be loaded, and if you run `lsfp --theme solarized.dark` the theme file `~/.lsfp-themes/solarized.dark` will be loaded.

File themes do not have an specific extensions neither syntax support in any IDE. That is one of the reasons why `lsfp` loads named themes just by their whole file name, as extension is not important, the whole file name is used.

## Distribution

Theme files are easily distributable. Due to the fact that settings that require a feature to be enabled are ignored by the parser if the feature is not enabled, someone with the _color_ feature and someone without it could be using the same theme file.

`lsfp` provides official themes which you can find in the _themes_ folder (at the crate root, as of now **there are no official themes create**) and distributed as a zip file on GitHub releases next to the built binaries.

## License

Both the code of the theme parser and the specification are distributed under the same license as `lsfp`, as it is part of the program. This license is the MIT license, which allows you to modify and use this parser and specification for both personal and commercial projects, either open source or closed source. See the full LICENSE file at [The-Noah/lsfp](https://github.com/The-Noah/lsfp/blob/master/LICENSE).

## TODO

### Meaningful (impact user experience)

- Allow to set default icon and colors, same as for directories (collapsed and expanded), use `!default`, `!dir_default`, `!dir_open` and `!dir_collapsed`

- Use `;` to separate lines instead of line break

- ~~Allow extra non empty lines as comments, maybe make them start with a `#` or similar~~

- ~~Allow for `:` instead of `=` separator~~

- ~~Make `extensions` key required~~

### DX

- ~~Better error messages, maybe by indicating the line of the error (look at https://llogiq.github.io/2017/06/01/perf-pitfalls.html), use `for (i, line) in text.lines().enumerate() { /* ... */ }`~~

- ~~Return `struct ParseError { line: usize, text: String, error_msg: String }` for Result.Err~~

- Make `ParserError.line` of `Line` type and drop `ParserError.text` attribute

- Make `Line` accept `AsRef<str>` instead of directly `&str` so you can use `String` in `ParserError`

- Return result in `Vec<u8>.as_color()` instead of `(0,0,0)`

- Improve error message ("invalid digit found in string") in lang parser

### Specification

- ~~**IMPORTANT** Formal specification file~~

- Get a better heading than _Pairs_ for SPEC.md <- starting to like _Pairs_

- ~~Update [README.md](https://github.com/HipyCas/theme-parser/blob/master/README.md) to match the specification~~

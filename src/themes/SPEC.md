# `lsfp` Theme file specification

This is the formal specification for `lsfp`'s theme file grammar. `lsfp` will only be able to parse files that strictly follow this specification, falling back to the default theme when the file is not of the correct format. For information and usage of theme files inside `lsfp`, see the [theme module README](https://github.com/The-Noah/lsfp/blob/master/src/themes/README.md).

## Sections

A theme file is organized into multiple sections, identified by the `extensions` associated with it. Section start is delimited by a dash (`-`) at the beginning of the line, being everything after the first dash discarded (`lsfp` will not analyze any text after it, you can write anything after the first dahs), and ends with the next line starting by a dash (the next section start delimiter).

Each section contains `key=value` pairs, which are explained in the [Pairs](#Pairs) section. A complete section could be the following (see [Example](#Example) for multiple sections):

```
--- JS (mjs for modules with node). TODO Add more
    extensions=js,mjs
    color=255,220,0
    icon=f898
```

All pairs **MUST** be inside a section, meaning that no pair can go at the beginning of the file before the first section, only comments may go before the first section.

## Pairs

The theme styles are indicated in form of `key=value` or `key:value` (both `=` and `:` have the same meaning, equal is used in the examples in this document) pairs inside each section, being `extensions` the primary key (the one that is used for identification by `lsfp`), and the other dependant of `lsfp`'s features. There are three available pairs as of now:

- `extensions`: An array-like of extensions, which should be styled with the styles specified in the section. If this is not specified, the styles in the section will not be applied to any file. The syntax for this key is as follows: `color={ext1},{ext2},{..exts}`, as an example `extensions=js,mjs`.

- `color`: A tuple-like color, indicating the color with which the extensions should be printed. It must be a tuple of 3 numbers, in the range of 0 to 255 included (rust's `u8` type). This configuration is only available if `lsfp` is compiled with the _color_ feature (see [Features in README.md](https://github.com/The-Noah/lsfp/blob/master/src/themes/README.md#Features) for more about features). The syntax for this key is as follows: `color={r},{g},{b}`, as an example `color=255,220,0`.

- `icon`: A four hexadecimal character string representing the icon Unicode code. Icons are supposed to be used with NerdFonts (icons used by default in `lsfp` depend on this font), and you can find a full list of icons and their hexadecimal string on the [NerdFont cheat-sheet](https://www.nerdfonts.com/cheat-sheet); but you may use any form of icon support if it is based in 4 character hexadecimal Unicode value. This configuration is only available if `lsfp` is compiled with the _icons_ feature (see [Features in README.md](https://github.com/The-Noah/lsfp/blob/master/src/themes/README.md#Features) for more about features). The syntax for this key is as follows: `icon={4 chars}`, as an example `icon=f898`.

Short versions of all keys are also available, which are the first letter of the key. That way, `icon=f898` is the same as `i=f898` and `extensions=js,mjs` is the same as `e=js,mjs`.

## Comments

You can write comments by using a hashtag (`#`), both at the beginning of a line or at the end of a pair. Any text in the comment is completely ignored by the parser. For example, the following are both valid comments:

```
icon=f2a4 # Beautiful solid icon
# TODO Add color, get some ideas online
```

Due to the fact that anything in a section delimiter after the first dash is ignored, any text after that first dash can be considered a de facto comment.

## Flexibilities

The parser presents some flexibilities in how you may write values, which are outlined below.

- Leading and trailing commas are ignored in both `color` and `extensions` settings, meaning that both `color=,9,0,12` and `extensions=c,h,o,` are both valid pairs.

- Spaces at beginning and end of lines, keys and values are removed, therefore `<tab|spaces># comment`, `<spaces>icon = f56a <spaces>` and `<spaces>- section<spaces>` are all valid.

## Example

Below is present a full example (a real theme file would be much bigger):

```
--- JS (mjs for modules with node). TODO Add more
extensions=js,mjs
color=255,220,0
icon=f898

--- TS Maybe there are more extensions? Revise icon
extensions=ts
color=0,31,63

icon=e628

-- IMPORTANT Rust, don't erase this by any means!
color=255,65,54
extensions=rs
icon=e7a8
```

# lsfp

[![GitHub license](https://img.shields.io/github/license/The-Noah/lsfp.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](http://makeapullrequest.com)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/The-Noah/lsfp?sort=semver)](https://github.com/The-Noah/lsfp/releases)
[![CI](https://github.com/The-Noah/lsfp/workflows/CI/badge.svg)](https://github.com/The-Noah/lsfp/actions?query=workflow%3ACI)
[![Built with love by devs for devs](https://img.shields.io/badge/Built%20with%20%F0%9F%92%96%20by%20developers-for%20developers-blue)](https://github.com/The-Noah/lsfp/graphs/contributors)

Cross-platform alternative to **ls**, specifically designed **f**or **p**rogrammers

> My blog post about lsfp: [blog.thenoah.dev/how-lsfp-was-created](https://blog.thenoah.dev/how-lsfp-was-created)

## Features

- Basic features (same as `ls`)
- License type detection
- Colored files by extension
- Collapsed build directories
- Git integration
- Extension-based customizable icons
- Custom colors and icons with [themes](https://github.com/The-Noah/lsfp/blob/master/src/themes/README.md)
- No external dependencies

### What is it?

Replacement for the `ls` command with a bunch of helpful tools for developers

### What is it not?

- A complete replacement for `ls` (maybe one day)
- Complete (still WIP)

### What will it be?

Hopefully so much more.

## Installation

### GitHub releases

Head directly to the [releases page](https://github.com/The-Noah/lsfp/releases) in this repository to install a fixed version of `lsfp`. You can find builds for MacOS, Linux and Windows, including a build with minimum features for Windows. Keep in mind, you will have to manually download again every update if you use GitHub releases.

### Cargo

You can install directly using Cargo with:

```sh
cargo install lsfp
```

### Git

To keep up-to-date with any changes, first clone this repo and `cd` into it:

```sh
git clone https://github.com/The-Noah/lsfp.git
cd lsfp
```

To build and install it on your system, run:

```sh
cargo install --path .
```

Now you can use `lsfp` from anywhere!

## Usage

You can see an up-to-date usage by running the help command!

```sh
lsfp --help
```

## Contributing

Contributions are welcome and much appreciated! For more details you can view the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## Contributors

[<img src="https://github.com/The-Noah.png?size=64" style="border-radius:100%" title="Author">](https://github.com/The-Noah)
[<img src="https://github.com/MattPlays.png?size=64" style="border-radius:100%" title="Core Contributor">](https://github.com/MattPlays)
[<img src="https://github.com/HipyCas.png?size=64" style="border-radius:100%" title="Contributor">](https://github.com/HipyCas)

## License

Licensed under the [MIT license](LICENSE).

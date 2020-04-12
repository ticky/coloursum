# ColourSum

[![crates.io](https://img.shields.io/crates/v/coloursum.svg)](https://crates.io/crates/coloursum) [![Rust](https://github.com/ticky/coloursum/actions/workflows/rust.yml/badge.svg)](https://github.com/ticky/coloursum/actions/workflows/rust.yml) [![codecov](https://codecov.io/gh/ticky/coloursum/branch/develop/graph/badge.svg)](https://codecov.io/gh/ticky/coloursum)

🎨 Colourise your checksum output

## What is this?

This is a utility into which you can pipe the output from various checksum generators, to get coloured output.

It understands both the BSD "tag" form, as well as the GNU Coreutils/Perl `shasum(1)` form of checksums, and has been tested with the output from macOS' `md5` and `shasum`, as well as GNU `md5sum` and `sha256sum`.

## Installation

You'll need [Rust installed and ready to go](https://www.rust-lang.org/tools/install).

```bash
cargo install coloursum
```

## Usage

```bash
md5sum [file] | coloursum
```

Coloursum also prints full usage information if you run `coloursum --help`.

### Shell Integration

You can also integrate coloursum into your shell, to output colourful checksums by default!

By default, it will search for known checksum commands' presence, and generate shell functions for those which are found.

If this behaviour is not acceptable, or your checksum command is not in the list, you can optionally specify a checksum command as the last argument to `coloursum shell-setup` to generate a shell function just for it.

#### bash, zsh, and other similar shells

Add this line to your ~/.bash_profile, ~/.zshrc or equivalent file:

```sh
eval "$(coloursum --mode=1password shell-setup)"
```

#### fish shell

Add this line to ~/.config/fish/config.fish:

```fish
status --is-interactive; and coloursum --mode=ecoji shell-setup | source
```

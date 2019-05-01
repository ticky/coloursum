# ColourSum

[![crates.io](https://img.shields.io/crates/v/coloursum.svg)](https://crates.io/crates/coloursum) [![Build Status](https://travis-ci.org/ticky/coloursum.svg?branch=develop)](https://travis-ci.org/ticky/coloursum)

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

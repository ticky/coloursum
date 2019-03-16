# ColourSum

Colourise your checksum output

## What is this?

This is a utility into which you can pipe the output from various checksum generators, to get coloured output.

It understands both the BSD "tag" form, as well as the GNU Coreutils/Perl `shasum(1)` form of checksums, and has been tested with the output from macOS' `md5`, `shasum`, as well as GNU `md5sum`, `sha256sum`.

## Usage

```bash
md5sum [file] | coloursum
```

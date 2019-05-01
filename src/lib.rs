//! 🎨 Colourise your checksum output
//!
//! Coloursum provides several line formatters capable of being produced from
//! a line of various checksum generators' output, in order to colourise or
//! otherwise transform their checksum in order to improve readability.
//!
//! ## Formatting one line with the `Line` trait
//!
//! Types implementing `Line` understand both the BSD "tag" form, as well as
//! the GNU Coreutils/Perl `shasum(1)` form of checksums, and have been tested
//! with the output from macOS' `md5`, `shasum`, as well as GNU `md5sum`
//! and `sha256sum`.
//!
//! They emit their formatted contents when `Display`ed to a user, with
//! macros like `format!` or `writeln!`:
//!
//! ```rust
//! use coloursum::EcojiLine;
//!
//! // BSD "tag" form
//! let bsd_line = EcojiLine::from("MD5 (./src/ecoji_line.rs) = 841d462b66e1f4bb839a1b72ab3f3668".to_string());
//!
//! assert_eq!(
//!     format!("{}", bsd_line),
//!     "MD5 (./src/ecoji_line.rs) = 📢💥👛🤓🤴🛌😫🥊🌵🚦😚🚲👱☕☕☕"
//! );
//!
//! // GNU Coreutils/Perl form
//! let gnu_line = EcojiLine::from("841d462b66e1f4bb839a1b72ab3f3668  ./src/ecoji_line.rs".to_string());
//!
//! assert_eq!(
//!     format!("{}", gnu_line),
//!     "📢💥👛🤓🤴🛌😫🥊🌵🚦😚🚲👱☕☕☕  ./src/ecoji_line.rs"
//! );
//! ```
//!
//! ## Formatting a buffer of lines with the `coloursum` function
//!
//! The `Line` trait implements a `coloursum` function.
//! This consumes a `BufRead` input buffer, and writes formatted lines to a
//! `Write` output buffer.
//!
//! ```rust
//! use std::io::BufReader;
//!
//! use coloursum::{Line, EcojiLine};
//! use indoc::indoc;
//!
//! // Note that each line will have its format detected separately, so
//! // BSD and GNU form lines can be in the same buffer
//! let input = indoc!("
//!     MD5 (./src/ecoji_line.rs) = 841d462b66e1f4bb839a1b72ab3f3668
//!     841d462b66e1f4bb839a1b72ab3f3668  ./src/ecoji_line.rs
//! ").as_bytes();
//!
//! let input_buffer = BufReader::new(input);
//! let mut output_buffer: Vec<u8> = Vec::new();
//!
//! EcojiLine::coloursum(input_buffer, &mut output_buffer);
//!
//! assert_eq!(
//!     std::str::from_utf8(&output_buffer).unwrap(),
//!     indoc!("
//!         MD5 (./src/ecoji_line.rs) = 📢💥👛🤓🤴🛌😫🥊🌵🚦😚🚲👱☕☕☕
//!         📢💥👛🤓🤴🛌😫🥊🌵🚦😚🚲👱☕☕☕  ./src/ecoji_line.rs
//!     ")
//! );
//! ```

mod base_line;
pub use base_line::{FormattableLine, Line};

mod ansi_coloured_line;
pub use ansi_coloured_line::ANSIColouredLine;

mod ecoji_line;
pub use ecoji_line::EcojiLine;

mod onepassword_line;
pub use onepassword_line::OnePasswordLine;

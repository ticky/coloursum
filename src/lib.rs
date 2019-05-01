//! 🎨 Colourise your checksum output
//!
//! Coloursum provides several line formatters capable of being produced from
//! a line of various checksum generators' output, in order to colourise or
//! otherwise transform their checksum in order to improve readability.
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
//! The provided `coloursum` function is able to be used with any type which
//! implements `Line`, consumes a `BufRead` input buffer, and writes formatted
//! lines to a `Write` output buffer.

use std::io;
use std::io::{BufRead, Write};

mod base_line;
pub use base_line::{FormattableLine, Line};

mod ansi_coloured_line;
pub use ansi_coloured_line::ANSIColouredLine;

mod ecoji_line;
pub use ecoji_line::EcojiLine;

mod onepassword_line;
pub use onepassword_line::OnePasswordLine;

/// Takes each line in `from`, and writes it to `to`.
///
/// If a given line is recognisable as the output of a
/// hashing utility, its hash value will be coloured.
pub fn coloursum<F: Line, I: BufRead, O: Write>(from: I, mut to: O) -> io::Result<()> {
    for wrapped_line in from.lines() {
        writeln!(to, "{}", F::from(wrapped_line?))?
    }

    Ok(())
}

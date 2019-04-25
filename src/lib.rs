use std::io;
use std::io::{BufRead, Write};

mod base_line;
pub use base_line::{FormattableLine, Line};

mod ansi_coloured_line;
pub use ansi_coloured_line::ANSIColouredLine;

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

use std::convert::From;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::io::{BufRead, Write};

#[derive(Debug)]
/// Representation of the formattable contents of a line of console output.
pub struct FormattableLine {
    contents: String,
    formattable_start: Option<usize>,
    formattable_end: Option<usize>,
}

impl From<String> for FormattableLine {
    /// Converts a `String` to a `FormattableLine`.
    fn from(contents: String) -> Self {
        let mut formattable_start: Option<usize> = None;
        let mut formattable_end: Option<usize> = None;

        if let Some(suffix_start) = find_bsd_tag_line(&contents) {
            formattable_start = Some(suffix_start);
        } else if let Some(prefix_end) = find_sum_prefixed_line(&contents) {
            formattable_end = Some(prefix_end);
        }

        Self {
            contents,
            formattable_start,
            formattable_end,
        }
    }
}

/// Used to present a formattable line, which can be derived from a `String`.
pub trait Line: Display + From<String> {
    /// Formats the given checksum string.
    fn format_hash(hash: String) -> String;

    /// Retrieves the underlying `FormattableLine` object.
    fn get_line(&self) -> &FormattableLine;

    /// Takes each line in `from`, and writes it to `to`.
    ///
    /// If a given line is recognisable as the output of a
    /// hashing utility, its hash value will be coloured.
    fn coloursum<I: BufRead, O: Write>(from: I, mut to: O) -> io::Result<()> {
        for wrapped_line in from.lines() {
            writeln!(to, "{}", Self::from(wrapped_line?))?
        }

        Ok(())
    }

    /// Writes the processed line to the supplied `Formatter`.
    ///
    /// May be overridden in order to replace the checksum-replacing behaviour if necessary.
    fn to_formatted(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let line = self.get_line();

        // Fall back to writing with no extra formatting
        // if we didn't detect a hash at any position
        if line.formattable_start.is_none() && line.formattable_end.is_none() {
            return write!(formatter, "{}", line.contents);
        }

        let slice_start = line.formattable_start.unwrap_or(0);
        let slice_end = line.formattable_end.unwrap_or(line.contents.len());

        write!(
            formatter,
            "{}{}{}",
            &line.contents[..slice_start],
            Self::format_hash(line.contents[slice_start..slice_end].to_string()),
            &line.contents[slice_end..],
        )
    }
}

/// Detects the *starting* offset of the
/// hash in a BSD `md5(1)` style line
fn find_bsd_tag_line(line: &str) -> Option<usize> {
    let needle = " = ";
    line.rfind(needle).map(|offset| offset + needle.len())
}

/// Detects the *ending* offset of the hash in a
/// GNU `md5sum(1)` / perl `shasum(1)` style line
fn find_sum_prefixed_line(line: &str) -> Option<usize> {
    line.find("  ")
}

#[cfg(test)]
mod tests {
    #[test]
    fn from_string_works() {
        use super::FormattableLine;

        let string = "MD5 (./src/main.rs) = b7527e0e28c09f6f62dd2d4197d5d225".to_string();
        let line = FormattableLine::from(string.clone());

        assert_eq!(line.contents, string);
        assert_eq!(line.formattable_start, Some(22));
        assert_eq!(line.formattable_end, None);
    }

    #[test]
    fn find_bsd_tag_line_works() {
        use super::find_bsd_tag_line;

        assert_eq!(
            find_bsd_tag_line("MD5 (./src/main.rs) = b7527e0e28c09f6f62dd2d4197d5d225"),
            Some(22)
        );
        assert_eq!(
            find_bsd_tag_line("b7527e0e28c09f6f62dd2d4197d5d225  ./src/main.rs"),
            None
        );
        assert_eq!(
            find_bsd_tag_line(
                "3e08ba70bfc57da75612af458c7ea94108f9a9ddf9d1bfd96de9c0e34e684bda  ./src/main.rs"
            ),
            None
        );
    }

    #[test]
    fn find_sum_prefixed_line_works() {
        use super::find_sum_prefixed_line;

        assert_eq!(
            find_sum_prefixed_line("b7527e0e28c09f6f62dd2d4197d5d225  ./src/main.rs"),
            Some(32)
        );
        assert_eq!(
            find_sum_prefixed_line(
                "3e08ba70bfc57da75612af458c7ea94108f9a9ddf9d1bfd96de9c0e34e684bda  ./src/main.rs"
            ),
            Some(64)
        );
        assert_eq!(
            find_sum_prefixed_line("MD5 (./src/main.rs) = b7527e0e28c09f6f62dd2d4197d5d225"),
            None
        );
    }
}

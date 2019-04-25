use std::convert::From;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::io::{BufRead, Write};
use std::iter::FromIterator;

use ansi_term::Colour::Fixed;
use itertools::Itertools;

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

    /// Retrieves the underlying [`FormattableLine`] object.
    fn get_line(&self) -> &FormattableLine;

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
        let slice_end = line.formattable_end.unwrap_or_else(|| line.contents.len());

        write!(
            formatter,
            "{}{}{}",
            line.contents[..slice_start].to_string(),
            Self::format_hash(line.contents[slice_start..slice_end].to_string()),
            line.contents[slice_end..].to_string(),
        )
    }
}

#[derive(Debug)]
/// Line with naïve ANSI Colour code formatting.
pub struct ANSIColouredLine(FormattableLine);
impl From<String> for ANSIColouredLine {
    fn from(contents: String) -> Self {
        Self(FormattableLine::from(contents))
    }
}

impl Display for ANSIColouredLine {
    fn fmt(&self, mut formatter: &mut fmt::Formatter) -> fmt::Result {
        self.to_formatted(&mut formatter)
    }
}

impl Line for ANSIColouredLine {
    fn get_line(&self) -> &FormattableLine {
        &self.0
    }

    /// Formats a base16-format hash or digest.
    ///
    /// Each 8-bit hexadecimal digit will be coloured
    /// with the corresponding xterm colour.
    fn format_hash(hash: String) -> String {
        use std::num::ParseIntError;

        // map over every two characters
        let result: Result<String, ParseIntError> = hash
            .chars()
            .chunks(2)
            .into_iter()
            .map(|byte| {
                let ord_string = String::from_iter(byte);
                // attempt to parse those two characters as a u8,
                // and if that works, colour them in
                u8::from_str_radix(&ord_string, 16)
                    .map(|ordinal| Fixed(ordinal).paint(ord_string).to_string())
            })
            .collect();

        // if there was an error at any point, return the original value
        result.unwrap_or(hash)
    }
}

#[derive(Debug)]
/// Line with formatting which colours numeric digits in blue,
/// and leaves the rest alone. Inspired by 1Password.
pub struct OnePasswordLine(FormattableLine);
impl From<String> for OnePasswordLine {
    fn from(contents: String) -> Self {
        Self(FormattableLine::from(contents))
    }
}

impl Display for OnePasswordLine {
    fn fmt(&self, mut formatter: &mut fmt::Formatter) -> fmt::Result {
        self.to_formatted(&mut formatter)
    }
}

impl Line for OnePasswordLine {
    fn get_line(&self) -> &FormattableLine {
        &self.0
    }

    /// Formats a base16-format hash or digest.
    ///
    /// Any numeric characters are formatted in blue.
    fn format_hash(hash: String) -> String {
        hash.chars()
            .map(|character| {
                if character.is_ascii_digit() {
                    Fixed(4).paint(character.to_string()).to_string()
                } else {
                    character.to_string()
                }
            })
            .collect()
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

#[cfg(test)]
mod tests {
    #[test]
    fn ansi_coloured_line_format_hash_works() {
        use super::ANSIColouredLine;
        use crate::Line;

        assert_eq!(
            ANSIColouredLine::format_hash(
                "b7527e0e28c09f6f62dd2d4197d5d225".to_string()
            ),
            "\u{1b}[38;5;183mb7\u{1b}[0m\u{1b}[38;5;82m52\u{1b}[0m\u{1b}[38;5;126m7e\u{1b}[0m\u{1b}[38;5;14m0e\u{1b}[0m\u{1b}[38;5;40m28\u{1b}[0m\u{1b}[38;5;192mc0\u{1b}[0m\u{1b}[38;5;159m9f\u{1b}[0m\u{1b}[38;5;111m6f\u{1b}[0m\u{1b}[38;5;98m62\u{1b}[0m\u{1b}[38;5;221mdd\u{1b}[0m\u{1b}[38;5;45m2d\u{1b}[0m\u{1b}[38;5;65m41\u{1b}[0m\u{1b}[38;5;151m97\u{1b}[0m\u{1b}[38;5;213md5\u{1b}[0m\u{1b}[38;5;210md2\u{1b}[0m\u{1b}[38;5;37m25\u{1b}[0m"
        );
    }

    #[test]
    fn ansi_coloured_line_format_hash_doesnt_crash_on_non_base16_characters() {
        use super::ANSIColouredLine;
        use crate::Line;

        ANSIColouredLine::format_hash("ASDF".to_string());
        ANSIColouredLine::format_hash("😄".to_string());
    }

    #[test]
    fn onepassword_line_format_hash_works() {
        use super::OnePasswordLine;
        use crate::Line;

        assert_eq!(
            OnePasswordLine::format_hash(
                "b7527e0e28c09f6f62dd2d4197d5d225".to_string()
            ),
            "b\u{1b}[38;5;4m7\u{1b}[0m\u{1b}[38;5;4m5\u{1b}[0m\u{1b}[38;5;4m2\u{1b}[0m\u{1b}[38;5;4m7\u{1b}[0me\u{1b}[38;5;4m0\u{1b}[0me\u{1b}[38;5;4m2\u{1b}[0m\u{1b}[38;5;4m8\u{1b}[0mc\u{1b}[38;5;4m0\u{1b}[0m\u{1b}[38;5;4m9\u{1b}[0mf\u{1b}[38;5;4m6\u{1b}[0mf\u{1b}[38;5;4m6\u{1b}[0m\u{1b}[38;5;4m2\u{1b}[0mdd\u{1b}[38;5;4m2\u{1b}[0md\u{1b}[38;5;4m4\u{1b}[0m\u{1b}[38;5;4m1\u{1b}[0m\u{1b}[38;5;4m9\u{1b}[0m\u{1b}[38;5;4m7\u{1b}[0md\u{1b}[38;5;4m5\u{1b}[0md\u{1b}[38;5;4m2\u{1b}[0m\u{1b}[38;5;4m2\u{1b}[0m\u{1b}[38;5;4m5\u{1b}[0m"
        );
    }

    #[test]
    fn onepassword_line_format_hash_doesnt_crash_on_non_base16_characters() {
        use super::OnePasswordLine;
        use crate::Line;

        OnePasswordLine::format_hash("ASDF".to_string());
        OnePasswordLine::format_hash("😄".to_string());
    }

    #[test]
    fn find_bsd_tag_line_works() {
        use super::find_bsd_tag_line;

        assert_eq!(
            find_bsd_tag_line(
                &"MD5 (./src/main.rs) = b7527e0e28c09f6f62dd2d4197d5d225".to_string()
            ),
            Some(22)
        );
        assert_eq!(
            find_bsd_tag_line(&"b7527e0e28c09f6f62dd2d4197d5d225  ./src/main.rs".to_string()),
            None
        );
        assert_eq!(
            find_bsd_tag_line(
                &"3e08ba70bfc57da75612af458c7ea94108f9a9ddf9d1bfd96de9c0e34e684bda  ./src/main.rs"
                    .to_string()
            ),
            None
        );
    }

    #[test]
    fn find_sum_prefixed_line_works() {
        use super::find_sum_prefixed_line;

        assert_eq!(
            find_sum_prefixed_line(&"b7527e0e28c09f6f62dd2d4197d5d225  ./src/main.rs".to_string()),
            Some(32)
        );
        assert_eq!(
            find_sum_prefixed_line(
                &"3e08ba70bfc57da75612af458c7ea94108f9a9ddf9d1bfd96de9c0e34e684bda  ./src/main.rs"
                    .to_string()
            ),
            Some(64)
        );
        assert_eq!(
            find_sum_prefixed_line(
                &"MD5 (./src/main.rs) = b7527e0e28c09f6f62dd2d4197d5d225".to_string()
            ),
            None
        );
    }
}

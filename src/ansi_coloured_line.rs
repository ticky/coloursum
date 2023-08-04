use std::convert::From;
use std::fmt;
use std::fmt::Display;
use std::iter::FromIterator;

use ansi_term::Colour::Fixed;
use itertools::Itertools;

use crate::base_line::{FormattableLine, Line};

#[derive(Debug)]
/// Line with naïve ANSI Colour code formatting.
pub struct ANSIColouredLine(FormattableLine);
impl From<String> for ANSIColouredLine {
    fn from(contents: String) -> Self {
        Self(FormattableLine::from(contents))
    }
}

impl Display for ANSIColouredLine {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.to_formatted(formatter)
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

#[cfg(test)]
mod tests {
    #[test]
    fn display_works() {
        use super::ANSIColouredLine;

        assert_eq!(
            format!("{}", ANSIColouredLine::from("MD5 (./src/main.rs) = b7527e0e28c09f6f62dd2d4197d5d225".to_string())),
            "MD5 (./src/main.rs) = \u{1b}[38;5;183mb7\u{1b}[0m\u{1b}[38;5;82m52\u{1b}[0m\u{1b}[38;5;126m7e\u{1b}[0m\u{1b}[38;5;14m0e\u{1b}[0m\u{1b}[38;5;40m28\u{1b}[0m\u{1b}[38;5;192mc0\u{1b}[0m\u{1b}[38;5;159m9f\u{1b}[0m\u{1b}[38;5;111m6f\u{1b}[0m\u{1b}[38;5;98m62\u{1b}[0m\u{1b}[38;5;221mdd\u{1b}[0m\u{1b}[38;5;45m2d\u{1b}[0m\u{1b}[38;5;65m41\u{1b}[0m\u{1b}[38;5;151m97\u{1b}[0m\u{1b}[38;5;213md5\u{1b}[0m\u{1b}[38;5;210md2\u{1b}[0m\u{1b}[38;5;37m25\u{1b}[0m"
        )
    }

    #[test]
    fn format_hash_works() {
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
    fn format_hash_doesnt_crash_on_non_base16_characters() {
        use super::ANSIColouredLine;
        use crate::Line;

        ANSIColouredLine::format_hash("ASDF".to_string());
        ANSIColouredLine::format_hash("😄".to_string());
    }
}

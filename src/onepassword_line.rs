use std::convert::From;
use std::fmt;
use std::fmt::Display;

use ansi_term::Colour::Fixed;

use crate::base_line::{FormattableLine, Line};

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

#[cfg(test)]
mod tests {
    #[test]
    fn format_hash_works() {
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
    fn format_hash_doesnt_crash_on_non_base16_characters() {
        use super::OnePasswordLine;
        use crate::Line;

        OnePasswordLine::format_hash("ASDF".to_string());
        OnePasswordLine::format_hash("😄".to_string());
    }
}

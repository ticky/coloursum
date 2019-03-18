use std::convert::From;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::io::{BufRead, Write};
use std::iter::FromIterator;

use ansi_term::Colour::Fixed;
use itertools::Itertools;

#[derive(Debug)]
enum LineKind {
    BSD { suffix_start: usize },
    SumPrefixed { prefix_end: usize },
}

#[derive(Debug)]
struct Line {
    contents: String,
    kind: Option<LineKind>,
}

impl From<String> for Line {
    fn from(contents: String) -> Self {
        let mut kind: Option<LineKind> = None;

        if let Some(suffix_start) = find_bsd_tag_line(&contents) {
            kind = Some(LineKind::BSD { suffix_start });
        } else if let Some(prefix_end) = find_sum_prefixed_line(&contents) {
            kind = Some(LineKind::SumPrefixed { prefix_end });
        }

        Self { contents, kind }
    }
}

impl Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // Fall back to writing with no extra formatting
        // if we didn't detect any particular kind of input
        let line_kind = match &self.kind {
            None => return write!(formatter, "{}", self.contents),
            Some(kind) => kind,
        };

        match line_kind {
            LineKind::BSD { suffix_start } => {
                let (prefix, suffix) = self.contents.split_at(*suffix_start);
                write!(formatter, "{}{}", prefix, format_hash(suffix.to_string()))
            }
            LineKind::SumPrefixed { prefix_end } => {
                let (prefix, suffix) = self.contents.split_at(*prefix_end);
                write!(formatter, "{}{}", format_hash(prefix.to_string()), suffix)
            }
        }
    }
}

fn format_hash(hash: String) -> String {
    use std::num::ParseIntError;

    let result: Result<String, ParseIntError> = hash
        .chars()
        .chunks(2)
        .into_iter()
        .map(|byte| {
            let ord_string = String::from_iter(byte);
            u8::from_str_radix(&ord_string, 16)
                .map(|ordinal| Fixed(ordinal).paint(ord_string).to_string())
        })
        .collect();

    result.unwrap_or(hash)
}

fn find_bsd_tag_line(line: &str) -> Option<usize> {
    let needle = " = ";
    line.rfind(needle).map(|offset| offset + needle.len())
}

fn find_sum_prefixed_line(line: &str) -> Option<usize> {
    line.find("  ")
}

fn coloursum<F: BufRead, T: Write>(from: F, mut to: T) -> io::Result<()> {
    for wrapped_line in from.lines() {
        writeln!(to, "{}", Line::from(wrapped_line?))?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let locked_stdin = stdin.lock();

    let stdout = io::stdout();
    let locked_stdout = stdout.lock();

    coloursum(locked_stdin, locked_stdout)
}

#[cfg(test)]
mod tests {
    #[test]
    fn format_hash_works() {
        use super::format_hash;

        assert_eq!(
            format_hash(
                "b7527e0e28c09f6f62dd2d4197d5d225".to_string()
            ),
            "\u{1b}[38;5;183mb7\u{1b}[0m\u{1b}[38;5;82m52\u{1b}[0m\u{1b}[38;5;126m7e\u{1b}[0m\u{1b}[38;5;14m0e\u{1b}[0m\u{1b}[38;5;40m28\u{1b}[0m\u{1b}[38;5;192mc0\u{1b}[0m\u{1b}[38;5;159m9f\u{1b}[0m\u{1b}[38;5;111m6f\u{1b}[0m\u{1b}[38;5;98m62\u{1b}[0m\u{1b}[38;5;221mdd\u{1b}[0m\u{1b}[38;5;45m2d\u{1b}[0m\u{1b}[38;5;65m41\u{1b}[0m\u{1b}[38;5;151m97\u{1b}[0m\u{1b}[38;5;213md5\u{1b}[0m\u{1b}[38;5;210md2\u{1b}[0m\u{1b}[38;5;37m25\u{1b}[0m"
        );
    }

    #[test]
    fn format_hash_doesnt_crash_on_non_base16_characters() {
        use super::format_hash;

        format_hash("ASDF".to_string());
        format_hash("😄".to_string());
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

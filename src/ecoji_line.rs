use std::convert::From;
use std::fmt;
use std::fmt::Display;
use std::iter::FromIterator;

use ecoji;
use itertools::Itertools;

use crate::base_line::{FormattableLine, Line};

#[derive(Debug)]
/// Line with Ecoji base-1024 emoji encoding.
pub struct EcojiLine(FormattableLine);
impl From<String> for EcojiLine {
    fn from(contents: String) -> Self {
        Self(FormattableLine::from(contents))
    }
}

impl Display for EcojiLine {
    fn fmt(&self, mut formatter: &mut fmt::Formatter) -> fmt::Result {
        self.to_formatted(&mut formatter)
    }
}

impl Line for EcojiLine {
    fn get_line(&self) -> &FormattableLine {
        &self.0
    }

    /// Formats a base16-format hash or digest.
    ///
    /// Data will be encoded using the Ecoji base-1024 emoji encoding.
    fn format_hash(hash: String) -> String {
        use std::num::ParseIntError;

        // map over every two characters
        let result: Result<Vec<u8>, ParseIntError> = hash
            .chars()
            .chunks(2)
            .into_iter()
            .map(|byte| {
                let ord_string = String::from_iter(byte);
                // attempt to parse those two characters as a u8
                u8::from_str_radix(&ord_string, 16)
            })
            .collect();

        match result {
            // if there was an error at any point, return the original value
            Err(_) => hash,
            // otherwise, encode with ecoji
            Ok(bytes) => ecoji::encode_to_string(&mut &*bytes).unwrap_or_else(|_| hash),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn format_hash_works() {
        use super::EcojiLine;
        use crate::Line;

        assert_eq!(
            EcojiLine::format_hash("cc6917b830dae305766d1d72d7bd9fdc673272b2".to_string()),
            "🚭🕹💿🈳🤘🔃🐮🕋🌽🚩📀👰🤞🕒🍤🗽"
        );
    }

    #[test]
    fn format_hash_doesnt_crash_on_non_base16_characters() {
        use super::EcojiLine;
        use crate::Line;

        EcojiLine::format_hash("ASDF".to_string());
        EcojiLine::format_hash("😄".to_string());
    }
}

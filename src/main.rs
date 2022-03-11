use std::io;
use std::str::FromStr;
use clap::Parser;

use coloursum::{ANSIColouredLine, EcojiLine, Line, OnePasswordLine};

#[derive(PartialEq, Debug)]
enum FormattingMode {
    ANSIColours,
    Ecoji,
    OnePassword,
}

impl FormattingMode {
    fn variants() -> [&'static str; 3] {
        ["ansi-colours", "ecoji", "1password"]
    }
}

impl FromStr for FormattingMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ansi-colours" => Ok(FormattingMode::ANSIColours),
            "ecoji" => Ok(FormattingMode::Ecoji),
            "1password" => Ok(FormattingMode::OnePassword),
            _ => Err("Unexpected formatting mode type".to_string()),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(version)]
struct Options {
    /// What sort of formatting to use for checksum values.
    #[clap(
        short,
        long,
        ignore_case = true,
        possible_values = FormattingMode::variants(),
        default_value = "ansi-colours"
    )]
    mode: FormattingMode,
}

fn main() -> io::Result<()> {
    let options = Options::parse();

    let stdin = io::stdin();
    let locked_stdin = stdin.lock();

    let stdout = io::stdout();
    let locked_stdout = stdout.lock();

    match options.mode {
        FormattingMode::ANSIColours => ANSIColouredLine::coloursum(locked_stdin, locked_stdout),
        FormattingMode::Ecoji => EcojiLine::coloursum(locked_stdin, locked_stdout),
        FormattingMode::OnePassword => OnePasswordLine::coloursum(locked_stdin, locked_stdout),
    }
}

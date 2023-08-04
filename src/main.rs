use clap::{Parser, ValueEnum};
use std::io;

use coloursum::{ANSIColouredLine, EcojiLine, Line, OnePasswordLine};

#[derive(Clone, PartialEq, Debug, ValueEnum)]
enum FormattingMode {
    ANSIColours,
    Ecoji,
    #[value(name = "1password")]
    OnePassword,
}

#[derive(Parser, Debug)]
#[clap(version)]
struct Options {
    /// What sort of formatting to use for checksum values.
    #[clap(
        short,
        long,
        ignore_case = true,
        value_enum,
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

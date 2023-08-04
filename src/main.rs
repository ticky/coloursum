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

impl ToString for FormattingMode {
    fn to_string(&self) -> String {
        match self {
            FormattingMode::ANSIColours => "ansi-colours",
            FormattingMode::Ecoji => "ecoji",
            FormattingMode::OnePassword => "1password",
        }
        .to_string()
    }
}

#[derive(Parser, Debug)]
#[clap(version)]
struct Options {
    #[clap(flatten)]
    main_options: MainOptions,

    #[clap(subcommand)]
    cmd: Option<Subcommand>,
}

#[derive(Parser, Debug)]
struct MainOptions {
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

#[derive(Parser, Debug)]
#[clap(rename_all = "kebab-case")]
enum Subcommand {
    /// Configure the current shell environment to use coloursum
    ///
    /// Automatically detects the currently used shell, and configures
    /// it to use coloursum with the specified options
    ///
    /// For example, to configure your zsh shell to use 1Password-style
    /// formatting, add the line
    /// `eval "$(coloursum --mode 1password shell-setup)"` to your ~/.zshrc
    /// file
    ///
    /// Or, to configure your fish shell to use ecoji formatting, but only
    /// for `sha256sum`, add the line
    /// `status --is-interactive; and coloursum --mode ecoji shell-setup sha256sum | source`
    /// to your ~/.config/fish/config.fish file
    #[cfg(unix)]
    #[clap(override_usage = r#"
    # for bash, zsh, and other similar shells
    eval "$(coloursum [OPTIONS] shell-setup [command])"

    # for fish
    status --is-interactive; and coloursum [OPTIONS] shell-setup [command] | source"#)]
    ShellSetup(ShellSetupOptions),
}

#[derive(Parser, Debug)]
struct ShellSetupOptions {
    /// Checksum command to set up shell integration for. If omitted, coloursum
    /// will attempt to automatically detect installed checksum commands.
    /// If specified, coloursum will configure only that checksum command.
    /// You may call shell-setup repeatedly to configure integration for
    /// multiple manually-configured checksum commands.
    command: Option<String>,
}

fn coloursum(options: &MainOptions) -> io::Result<()> {
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

#[cfg(unix)]
static SUM_EXECNAMES: &[&str] = &[
    "md5",
    "md5sum",
    "gmd5sum",
    "shasum",
    "sha1sum",
    "gsha1sum",
    "sha2",
    "sha256sum",
    "gsha256sum",
    "sha224sum",
    "gsha224sum",
    "sha384sum",
    "gsha384sum",
    "sha512sum",
    "gsha512sum",
];

#[cfg(unix)]
fn shell_setup(
    options: &MainOptions,
    shell_setup_options: &ShellSetupOptions,
) -> Result<(), std::io::Error> {
    // detect the calling shell's name
    let shell_name = match get_shell_name() {
        Some(shell_name) => shell_name,
        // fall back to "sh" if not detectable/something weird happens
        None => "sh".to_string(),
    };

    println!("# coloursum: generated setup for `{}`", shell_name);

    if let Some(command) = &shell_setup_options.command {
        print_shell_function(options, shell_name.as_ref(), command.to_string());
    } else {
        for executable in SUM_EXECNAMES {
            if let Ok(_path) = which::which(executable) {
                print_shell_function(options, shell_name.as_ref(), (*executable).to_string())
            }
        }
    }

    Ok(())
}

#[cfg(unix)]
fn get_shell_name() -> Option<String> {
    use sysinfo::{ProcessExt, System, SystemExt};

    let system = System::new_with_specifics(sysinfo::RefreshKind::new().with_processes());

    // find the current process by pid, fall back to 0
    // (which is invalid/reserved, so will return None early)
    let process = system.get_process(sysinfo::get_current_pid().unwrap_or(0))?;

    // find the parent process via the current process's parent ID
    let parent = system.get_process(process.parent()?)?;

    // and if we make it all the way here, return some name
    Some(parent.name().to_string())
}

#[cfg(unix)]
fn print_shell_function(options: &MainOptions, shell_name: &str, command: String) {
    // TODO: work out how to print this losslessly
    let exe_name = match std::env::current_exe() {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(_) => "coloursum".to_string(),
    };

    match shell_name {
        "fish" => println!(
            "function {0}\n\
            \tcommand {0} $argv | {1} --mode {2}\n\
            end",
            command,
            exe_name,
            options.mode.to_string()
        ),
        "ksh" => println!(
            "function {0} {{\n\
            \tcommand {0} \"$@\" | {1} --mode {2}\n\
            }}",
            command,
            exe_name,
            options.mode.to_string()
        ),
        _ => println!(
            "function {0}() {{\n\
            \tcommand {0} \"$@\" | {1} --mode {2}\n\
            }}",
            command,
            exe_name,
            options.mode.to_string()
        ),
    }
}

fn main() -> Result<(), std::io::Error> {
    let options = Options::parse();

    if let Some(command) = options.cmd {
        match command {
            #[cfg(unix)]
            Subcommand::ShellSetup(shell_setup_options) => {
                shell_setup(&options.main_options, &shell_setup_options)
            }
        }
    } else {
        coloursum(&options.main_options)
    }
}

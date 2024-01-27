#[macro_use]
extern crate log;

mod commands;
mod components;

use std::io::{self, Write};

use ciri::args::SystemSubCommands;
use ciri::validators::package::find;
use ciri::{Cli, System};
use clap::Parser;
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use flexi_logger::DeferredNow;
use log::Record;
use miette::IntoDiagnostic;

use self::commands::package;
use self::commands::system;

fn colorize_string(input: &str, color: Color) -> String {
    format!("{}{}{}", SetForegroundColor(color), input, ResetColor)
}

fn my_format(
    write: &mut dyn Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> std::io::Result<()> {
    let color = match record.level() {
        log::Level::Trace => Color::DarkBlue,
        log::Level::Debug => Color::Blue,
        log::Level::Info => Color::Green,
        log::Level::Warn => Color::Yellow,
        log::Level::Error => Color::Red,
    };

    write!(
        write,
        "{} [{}] {}",
        colorize_string(&record.level().to_string(), color),
        record.target(),
        record.args()
    )
}

fn main() -> miette::Result<()> {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        disable_raw_mode().into_diagnostic().unwrap();
        crossterm::execute!(io::stdout(), LeaveAlternateScreen)
            .into_diagnostic()
            .unwrap();
        original_hook(panic);
    }));

    flexi_logger::Logger::try_with_str("info")
        .expect("Logger config string formatted incorrectly")
        .format(my_format)
        .start()
        .expect("Failed to start logger");

    let cli = Cli::parse();

    if cli.health {
        find()?;
        return Ok(());
    }

    // TODO: And check for watch utility. Native or third party
    if let Some(subsommands) = cli.subcommands {
        match subsommands {
            ciri::SubCommands::System(cmd) => package_subcommand(cmd)?,
            ciri::SubCommands::New(args) => package::new(args)?,
            ciri::SubCommands::Run(args) => package::run(args)?,
            ciri::SubCommands::Build(args) => {
                info!("{:?} {:?} {}", args.name, args.script, args.watch);
                println!("Building...");
            }

            _ => todo!(),
        }
    } else {
        error!("No operation provided. (Use '-h' for help)");
    }

    Ok(())
}

fn package_subcommand(cmd: System) -> miette::Result<()> {
    if let Some(subcommands) = cmd.subcommands {
        match subcommands {
            SystemSubCommands::List(args) => system::list(args)?,
            _ => todo!(),
        }
    } else {
        error!("No operation provided. (Use '-h' for help)");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colorize_string_test() {
        assert_eq!(
            colorize_string("test", Color::Red),
            format!("\u{1b}[38;5;9mtest\u{1b}[0m")
        );
        assert_eq!(
            colorize_string("test", Color::Blue),
            format!("\u{1b}[38;5;12mtest\u{1b}[0m")
        );
    }
}

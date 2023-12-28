#[macro_use]
extern crate log;

mod commands;
mod components;
mod parsers;

use std::io::{self, Write};

use ciri::args::{PackageSubCommands, SystemSubCommands};
use ciri::validators::package::find;
use ciri::{Cli, Package, System};
use clap::Parser;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use flexi_logger::DeferredNow;
use log::Record;
use miette::IntoDiagnostic;
use nu_ansi_term::Color;

use self::commands::package;
use self::commands::system;

fn my_format(
    write: &mut dyn Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> std::io::Result<()> {
    let color = match record.level() {
        log::Level::Trace => Color::Blue,
        log::Level::Debug => Color::LightBlue,
        log::Level::Info => Color::Green,
        log::Level::Warn => Color::Yellow,
        log::Level::Error => Color::Red,
    };

    write!(
        write,
        "{} [{}] {}",
        color.paint(record.level().to_string()),
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

    let logger_config = "info";
    flexi_logger::Logger::try_with_str(logger_config)
        .expect("Logger config string formatted incorrectly")
        .format(my_format)
        .start()
        .expect("Failed to start logger");

    let cli = Cli::parse();

    if cli.health {
        find()?;
        return Ok(());
    }

    if let Some(subsommands) = cli.subcommands {
        match subsommands {
            ciri::SubCommands::System(cmd) => package_subcommand(cmd)?,
            ciri::SubCommands::Package(cmd) => project_subcommand(cmd)?,
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

fn project_subcommand(cmd: Package) -> miette::Result<()> {
    if let Some(subcommands) = cmd.subcommands {
        match subcommands {
            PackageSubCommands::Build(args) => {
                info!("{:?} {:?} {}", args.name, args.script, args.watch);
                println!("Building...");
            }

            PackageSubCommands::Run(args) => {
                info!("{:?} {:?} {}", args.name, args.build, args.watch);
                println!("Running...");
            }

            _ => todo!(),
        }
    } else {
        error!("No operation provided. (Use '-h' for help)");
    }

    Ok(())
}

#[macro_use]
extern crate log;

mod commands;
mod components;
mod parsers;
mod validators;

use std::io::Write;

use ciri::args::{PackageSubCommands, ProjectSubCommands};
use ciri::Cli;
use clap::Parser;
use flexi_logger::DeferredNow;
use log::Record;
use nu_ansi_term::Color;

use self::commands::package;
use self::validators::package::find;

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
            ciri::SubCommands::Package(cmd) => {
                if let Some(subcommands) = cmd.subcommands {
                    match subcommands {
                        PackageSubCommands::List(args) => package::list(args)?,
                        _ => todo!(),
                    }
                }
            }
            ciri::SubCommands::Project(cmd) => {
                if let Some(subcommands) = cmd.subcommands {
                    match subcommands {
                        ProjectSubCommands::Build(args) => {
                            info!("{:?} {:?} {}", args.name, args.script, args.watch);
                            println!("Building...");
                        }

                        ProjectSubCommands::Run(args) => {
                            info!("{:?} {:?} {}", args.name, args.build, args.watch);
                            println!("Running...");
                        }

                        _ => todo!(),
                    }
                }
            }
        }
    } else {
        error!("No operation provided. (Use '-h' for help)");
    }

    Ok(())
}

#[macro_use]
extern crate log;

mod commands;

use ciri::args::{PackageSubCommands, ProjectSubCommands};
use ciri::Cli;
use clap::Parser;

use self::commands::package;

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cli = Cli::parse();
    match cli.subcommands.unwrap() {
        ciri::SubCommands::Package(cmd) => match cmd.subcommands.unwrap() {
            PackageSubCommands::List(args) => package::list(args),
            _ => todo!(),
        },
        ciri::SubCommands::Project(cmd) => match cmd.subcommands.unwrap() {
            ProjectSubCommands::Build(args) => {
                info!("{:?} {:?} {}", args.name, args.script, args.watch);
                println!("Building...");
            }

            ProjectSubCommands::Run(args) => {
                info!("{:?} {:?} {}", args.name, args.build, args.watch);
                println!("Running...");
            }

            _ => todo!(),
        },
    }
}

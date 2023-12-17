#[macro_use]
extern crate log;

// use std::process::Command as ShellCommand;

use ciri::{Cli, PackageSubCommands, ProjectSubCommands};
use clap::Parser;

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cli = Cli::parse();
    match cli.subcommands.unwrap() {
        ciri::SubCommands::Package(cmd) => match cmd.subcommands.unwrap() {
            PackageSubCommands::List {
                installed,
                not_installed,
            } => {
                info!("{} {}", installed, not_installed);
                // let val = ShellCommand::new("pacman")
                //     .args(["-Q", "-e"])
                //     .output()
                //     .unwrap();
                // println!("{}", String::from_utf8(val.stdout).unwrap().trim());
            }

            _ => todo!(),
        },
        ciri::SubCommands::Project(cmd) => match cmd.subcommands.unwrap() {
            ProjectSubCommands::Build {
                name,
                script,
                watch,
            } => {
                info!("{:?} {:?} {}", name, script, watch);
                println!("Building...");
            }

            ProjectSubCommands::Run { name, build, watch } => {
                info!("{:?} {:?} {}", name, build, watch);
                println!("Running...");
            }

            _ => todo!(),
        },
    }
}

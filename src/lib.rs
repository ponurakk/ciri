#[macro_use]
extern crate log;

pub mod args;
pub mod components;
pub mod validators;

use std::collections::HashMap;

use clap::{Args, ColorChoice, Parser, Subcommand};
use lazy_static::lazy_static;

use self::args::{PackageSubCommands, SystemSubCommands};

#[derive(Parser)]
#[command(author, version, about, long_about = None, color = ColorChoice::Always)]
pub struct Cli {
    /// Check which tools are installed
    #[arg(long)]
    pub health: bool,

    #[command(subcommand)]
    pub subcommands: Option<SubCommands>,
}

#[derive(Subcommand)]
pub enum SubCommands {
    /// Manage system wide packages
    #[command(visible_alias = "sys")]
    System(System),

    /// Manage your project packages
    #[clap(visible_alias = "pr")]
    Package(Package),
}

#[derive(Args)]
pub struct System {
    #[command(subcommand)]
    pub subcommands: Option<SystemSubCommands>,
}

#[derive(Args)]
pub struct Package {
    #[command(subcommand)]
    pub subcommands: Option<PackageSubCommands>,
}

lazy_static! {
    static ref PACKAGE_MANAGERS: Vec<&'static str> = Vec::from([
        "bun", "cargo", "clang", "clang++", "composer", "dart", "deno", "flutter", "g++", "gcc",
        "go", "gradle", "groovy", "java", "kotlin", "lua", "maven", "node", "npm", "php", "pip",
        "pnpm", "python", "ruby", "scala", "swift", "yarn", "zig",
    ]);
}

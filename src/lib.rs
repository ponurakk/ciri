pub mod args;

use clap::{Args, ColorChoice, Parser, Subcommand};

use self::args::{PackageSubCommands, ProjectSubCommands};

#[derive(Parser)]
#[command(author, version, about, long_about = None, color = ColorChoice::Always)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommands: Option<SubCommands>,
}

#[derive(Subcommand)]
pub enum SubCommands {
    /// Manage packages
    #[clap(visible_alias = "pkg")]
    Package(Package),

    /// Manage projects
    #[clap(visible_alias = "pr")]
    Project(Project),
}

#[derive(Args)]
pub struct Package {
    #[command(subcommand)]
    pub subcommands: Option<PackageSubCommands>,
}

#[derive(Args)]
pub struct Project {
    #[command(subcommand)]
    pub subcommands: Option<ProjectSubCommands>,
}

use std::path::PathBuf;

use clap::{Args, ColorChoice, Parser, Subcommand};

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

// TODO: Detect package manager (pacman, apt, rpm, xbps, apk)
#[derive(Subcommand)]
pub enum PackageSubCommands {
    /// Download fresh package info
    #[clap(visible_alias = "f")]
    Fresh {
        /// Name of package to add
        name: String,
    },

    /// Adds a dependency
    #[clap(visible_alias = "a")]
    Add {
        /// Name of package to add
        name: String,
    },

    /// Removes a dependency
    #[clap(visible_alias = "rm")]
    Remove {
        /// Name of package to remove
        name: String,
    },

    /// Updates dependencies or just one
    #[clap(visible_alias = "up")]
    Update {
        /// Name of package to update
        name: Option<String>,
    },

    /// List available packages
    #[clap(visible_alias = "l")]
    List {
        /// List only installed packages
        #[arg(short, long)]
        installed: bool,

        /// List only *not*installed packages
        #[arg(short, long)]
        not_installed: bool,
    },
}

#[derive(Args)]
pub struct Project {
    #[command(subcommand)]
    pub subcommands: Option<ProjectSubCommands>,
}

// TODO: Detect package manager (cargo, npm*, pip, go, gradle, maven)
// And check for watch utility. Native or third party
#[derive(Subcommand)]
pub enum ProjectSubCommands {
    /// Build executable or execute build script
    #[clap(visible_alias = "n")]
    New {
        /// Name of project
        name: String,
    },

    /// Build executable or execute build script
    #[clap(visible_alias = "b")]
    Build {
        /// Specific file to build
        name: Option<PathBuf>,

        /// Path to sh script (it should mostly be autodetected)
        #[arg(short, long)]
        script: Option<PathBuf>,

        /// Watch changes and restart
        #[arg(long)]
        watch: bool,
    },

    /// Run executable or execute run script
    #[clap(visible_alias = "r")]
    Run {
        /// Specific file to run
        name: Option<PathBuf>,

        /// Should build be run before executing
        #[arg(long)]
        build: bool,

        /// Watch changes and restart
        #[arg(long)]
        watch: bool,
    },

    /// Runs tests on project
    #[clap(visible_alias = "t")]
    Test {
        /// Specific file to test
        name: Option<PathBuf>,

        /// Watch changes and restart
        #[arg(long)]
        watch: bool,
    },

    /// Adds a dependency
    #[clap(visible_alias = "a")]
    Add {
        /// Name of package to add
        name: String,
    },

    /// Removes a dependency
    #[clap(visible_alias = "rm")]
    Remove {
        /// Name of package to remove
        name: String,
    },

    /// Updates dependencies or just one
    #[clap(visible_alias = "up")]
    Update {
        /// Name of package to update
        name: Option<String>,
    },
}

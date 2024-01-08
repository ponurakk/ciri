use std::path::PathBuf;

use clap::{Args, Subcommand};

use crate::PackageManagers;

#[derive(Args)]
pub struct New {
    /// Name of project
    pub name: Option<String>,

    /// Select package manager you would like to use
    #[arg(short, long)]
    pub manager: PackageManagers,

    /// Don't ask additional prompts about package creation and go with defaults
    #[arg(short, long)]
    pub defaults: bool,
}

#[derive(Args)]
pub struct Build {
    /// Specific file to build
    pub name: Option<PathBuf>,

    /// Path to sh script (it should mostly be autodetected)
    #[arg(short, long)]
    pub script: Option<PathBuf>,

    /// Watch changes and restart
    #[arg(long)]
    pub watch: bool,
}

#[derive(Args)]
pub struct Run {
    /// Specific file to run
    pub name: Option<PathBuf>,

    /// Should build be run before executing
    #[arg(long)]
    pub build: bool,

    /// Watch changes and restart
    #[arg(long)]
    pub watch: bool,
}

#[derive(Args)]
pub struct Test {
    /// Specific file to test
    pub name: Option<PathBuf>,

    /// Watch changes and restart
    #[arg(long)]
    pub watch: bool,
}

#[derive(Args)]
pub struct Add {
    /// Name of package to add
    pub name: String,
}

#[derive(Args)]
pub struct Remove {
    /// Name of package to remove
    pub name: String,
}

#[derive(Args)]
pub struct Update {
    /// Name of package to update
    pub name: Option<String>,
}

// TODO: Detect package manager (cargo, npm*, pip, go, gradle, maven)
// And check for watch utility. Native or third party
#[derive(Subcommand)]
pub enum PackageSubCommands {
    /// Build executable or execute build script
    #[clap(visible_alias = "n")]
    New(New),

    /// Build executable or execute build script
    #[clap(visible_alias = "b")]
    Build(Build),

    /// Run executable or execute run script
    #[clap(visible_alias = "r")]
    Run(Run),

    /// Runs tests on project
    #[clap(visible_alias = "t")]
    Test(Test),

    /// Adds a dependency
    #[clap(visible_alias = "a")]
    Add(Add),

    /// Removes a dependency
    #[clap(visible_alias = "rm")]
    Remove(Remove),

    /// Updates dependencies or just one
    #[clap(visible_alias = "up")]
    Update(Update),
}

use std::path::PathBuf;

use clap::Args;

use crate::PackageManagers;

#[derive(Args, Debug)]
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

#[derive(Args, Debug)]
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

#[derive(Args, Debug)]
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

#[derive(Args, Debug)]
pub struct Test {
    /// Specific file to test
    pub name: Option<PathBuf>,

    /// Watch changes and restart
    #[arg(long)]
    pub watch: bool,
}

#[derive(Args, Debug)]
pub struct Add {
    /// Name of package to add
    pub name: String,
}

#[derive(Args, Debug)]
pub struct Remove {
    /// Name of package to remove
    pub name: String,
}

#[derive(Args, Debug)]
pub struct Update {
    /// Name of package to update
    pub name: Option<String>,
}

use clap::{Args, Subcommand};

#[derive(Args)]
pub struct Fresh {
    /// Name of package to add
    pub name: String,
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

#[derive(Args)]
pub struct List {
    /// List only installed packages
    #[arg(short, long)]
    pub installed: bool,

    /// List only *not*installed packages
    #[arg(short, long)]
    pub not_installed: bool,
}

// TODO: Detect package manager (pacman, apt, rpm, xbps, apk)
#[derive(Subcommand)]
pub enum PackageSubCommands {
    /// Download fresh package info
    #[clap(visible_alias = "f")]
    Fresh(Fresh),

    /// Adds a dependency
    #[clap(visible_alias = "a")]
    Add(Add),

    /// Removes a dependency
    #[clap(visible_alias = "rm")]
    Remove(Remove),

    /// Updates dependencies or just one
    #[clap(visible_alias = "up")]
    Update(Update),

    /// List available packages
    #[clap(visible_alias = "l")]
    List(List),
}

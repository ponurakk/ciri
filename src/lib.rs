#[macro_use]
extern crate log;

pub mod args;
pub mod components;
pub mod entities;
pub mod parsers;
pub mod validators;

use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

use clap::{Args, ColorChoice, Parser, Subcommand, ValueEnum};
use lazy_static::lazy_static;
use miette::{bail, IntoDiagnostic};
use serde::{Deserialize, Serialize};

use self::args::package::{Add, Build, New, Remove, Run, Test, Update};
use self::args::SystemSubCommands;
use self::entities::managers::*;

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

    /// Create new project structure
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

#[derive(Args)]
pub struct System {
    #[command(subcommand)]
    pub subcommands: Option<SystemSubCommands>,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum PackageManagers {
    Bun,
    Cargo,
    Clang,
    #[clap(name = "clang++")]
    Clangpp,
    Composer,
    Dart,
    Deno,
    Flutter,
    #[clap(name = "g++")]
    Gpp,
    Gcc,
    Go,
    Gradle,
    Groovy,
    Java,
    Kotlin,
    Lua,
    Maven,
    Node,
    Npm,
    Php,
    Pip,
    Pnpm,
    Python,
    Ruby,
    Scala,
    Swift,
    Yarn,
    Zig,
}

impl PackageManagers {
    pub fn to_vec() -> Vec<String> {
        [
            Self::Bun,
            Self::Cargo,
            Self::Clang,
            Self::Clangpp,
            Self::Composer,
            Self::Dart,
            Self::Deno,
            Self::Flutter,
            Self::Gpp,
            Self::Gcc,
            Self::Go,
            Self::Gradle,
            Self::Groovy,
            Self::Java,
            Self::Kotlin,
            Self::Lua,
            Self::Maven,
            Self::Node,
            Self::Npm,
            Self::Php,
            Self::Pip,
            Self::Pnpm,
            Self::Python,
            Self::Ruby,
            Self::Scala,
            Self::Swift,
            Self::Yarn,
            Self::Zig,
        ]
        .iter()
        .map(std::string::ToString::to_string)
        .collect()
    }

    pub fn to_manager(&self) -> miette::Result<Manager> {
        match self {
            Self::Cargo => Ok(CARGO_MANAGER),
            Self::Npm => Ok(NPM_MANAGER),
            Self::Pnpm => Ok(PNPM_MANAGER),
            Self::Yarn => Ok(YARN_MANAGER),
            Self::Bun => Ok(BUN_MANAGER),
            Self::Gpp => Ok(GPP_MANAGER),
            &_ => bail!("Package manager was not yet implemented"),
        }
    }
}

impl FromStr for PackageManagers {
    type Err = miette::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bun" => Ok(Self::Bun),
            "cargo" => Ok(Self::Cargo),
            "clang" => Ok(Self::Clang),
            "clang++" => Ok(Self::Clangpp),
            "composer" => Ok(Self::Composer),
            "dart" => Ok(Self::Dart),
            "deno" => Ok(Self::Deno),
            "flutter" => Ok(Self::Flutter),
            "g++" => Ok(Self::Gpp),
            "gcc" => Ok(Self::Gcc),
            "go" => Ok(Self::Go),
            "gradle" => Ok(Self::Gradle),
            "groovy" => Ok(Self::Groovy),
            "java" => Ok(Self::Java),
            "kotlin" => Ok(Self::Kotlin),
            "lua" => Ok(Self::Lua),
            "maven" => Ok(Self::Maven),
            "node" => Ok(Self::Node),
            "npm" => Ok(Self::Npm),
            "php" => Ok(Self::Php),
            "pip" => Ok(Self::Pip),
            "pnpm" => Ok(Self::Pnpm),
            "python" => Ok(Self::Python),
            "ruby" => Ok(Self::Ruby),
            "scala" => Ok(Self::Scala),
            "swift" => Ok(Self::Swift),
            "yarn" => Ok(Self::Yarn),
            "zig" => Ok(Self::Zig),
            &_ => bail!("No manager found"),
        }
    }
}

impl Display for PackageManagers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bun => write!(f, "bun"),
            Self::Cargo => write!(f, "cargo"),
            Self::Clang => write!(f, "clang"),
            Self::Clangpp => write!(f, "clangpp"),
            Self::Composer => write!(f, "composer"),
            Self::Dart => write!(f, "dart"),
            Self::Deno => write!(f, "deno"),
            Self::Flutter => write!(f, "flutter"),
            Self::Gpp => write!(f, "gpp"),
            Self::Gcc => write!(f, "gcc"),
            Self::Go => write!(f, "go"),
            Self::Gradle => write!(f, "gradle"),
            Self::Groovy => write!(f, "groovy"),
            Self::Java => write!(f, "java"),
            Self::Kotlin => write!(f, "kotlin"),
            Self::Lua => write!(f, "lua"),
            Self::Maven => write!(f, "maven"),
            Self::Node => write!(f, "node"),
            Self::Npm => write!(f, "npm"),
            Self::Php => write!(f, "php"),
            Self::Pip => write!(f, "pip"),
            Self::Pnpm => write!(f, "pnpm"),
            Self::Python => write!(f, "python"),
            Self::Ruby => write!(f, "ruby"),
            Self::Scala => write!(f, "scala"),
            Self::Swift => write!(f, "swift"),
            Self::Yarn => write!(f, "yarn"),
            Self::Zig => write!(f, "zig"),
        }
    }
}

lazy_static! {
    static ref LANGUAGES: HashMap<&'static str, Vec<&'static str>> = HashMap::from([
        (
            "c",
            vec![
                // Extensions
                "c", "h",
            ],
        ),
        (
            "g++",
            vec![
                // Files
                "CMakeLists.txt",
                // Extensions
                "cpp",
                "c++",
                "hpp",
            ],
        ),
        // ("clang++", vec![".cpp", ".c++", ".hpp"]),
        (
            "dart",
            vec![
                // Files
                ".dart",
                "pubspec.yaml",
                "pubspec.yml",
                "pubspec.lock",
                // Directories
                ".dart_tool",
            ],
        ),
        (
            "go",
            vec![
                // Files
                "go.mod",
                "go.sum",
                "go.work",
                "glide.yaml",
                "Gopkg.yml",
                "Gopkg.lock",
                ".go-version",
                // Directories
                "Godeps",
                // Extensions
                ".go",
            ],
        ),
        (
            "java",
            vec![
                // Files
                "pom.xml",
                "build.gradle.kts",
                "build.sbt",
                ".java-version",
                "deps.edn",
                "project.clj",
                "build.boot",
                ".sdkmanrc",
                // Extensions
                ".java",
                ".class",
                ".gradle",
                ".jar",
                ".clj",
                ".cljc",
            ],
        ),
        (
            "kotlin",
            vec![
                // Extensions
                "kt", "kts",
            ],
        ),
        (
            "lua",
            vec![
                // Files
                ".lua-version",
                // Directories/Extensions
                "lua",
            ],
        ),
        (
            "bun",
            vec![
                // Files
                "bun.lockb",
                "bunfig.toml",
                "package.json",
                // Directories
                "node_modules",
            ],
        ),
        (
            "npm",
            vec![
                // Files
                "package-lock.json",
                "package.json",
                // Directories
                "node_modules",
            ],
        ),
        (
            "pnpm",
            vec![
                // Files
                "pnpm-lock.yaml",
                "package.json",
                // Directories
                "node_modules",
            ],
        ),
        (
            "yarn",
            vec![
                // Files
                "yarn.lock",
                "package.json",
                // Directories
                "node_modules",
            ],
        ),
        (
            "php",
            vec![
                // Files
                "composer.json",
                ".php-version",
                // Extensions
                "php",
            ],
        ),
        (
            // TODO: Add virtual env detection
            "python",
            vec![
                // Files
                ".python-version",
                "Pipfile",
                "__init__.py",
                "pyproject.toml",
                "requirements.txt",
                "setup.py",
                "tox.ini",
                // Extensions
                "py",
            ],
        ),
        (
            "ruby",
            vec![
                // Files
                "Gemfile",
                ".ruby-version",
                // Extensions
                "rb",
            ],
        ),
        (
            "scala",
            vec![
                // Files
                "build.sbt",
                // Directories
                ".metals",
                // Extensions
                "scalaenv",
                "sbtenv",
                "scala",
                "sbt",
            ],
        ),
        (
            "cargo",
            vec![
                // Files
                "Cargo.toml",
                // Extensions
                "rs",
            ],
        ),
        (
            "swift",
            vec![
                // Files
                "Package.swift",
                // Extensions
                "swift",
            ],
        ),
        (
            "zig",
            vec![
                // Files
                "build.zig",
                // Directories
                "zig-cache",
                "zig-out",
                // Extensions
                "zig",
            ],
        ),
    ]);
}

pub trait Util {
    fn to_tuple(&self) -> miette::Result<(&str, &str)>;
}

impl Util for &str {
    fn to_tuple(&self) -> miette::Result<(&str, &str)> {
        let p = self.split_whitespace().collect::<Vec<_>>();
        Ok((
            p.get(0).ok_or(Error::NoArgument).into_diagnostic()?,
            p.get(1).ok_or(Error::NoArgument).into_diagnostic()?,
        ))
    }
}

#[derive(Debug)]
enum Error {
    NoArgument,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::NoArgument => write!(f, "Command doesn't have enough arguments"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub bin_name: Option<String>,
    pub prefered_project_manager: Option<String>,
}

impl Config {
    pub fn new(bin_name: Option<String>, prefered_project_manager: Option<String>) -> Self {
        Self {
            bin_name,
            prefered_project_manager,
        }
    }

    pub fn save(&self, path: Option<&str>) -> miette::Result<()> {
        let path_str = format!("{}/.ciri.toml", path.unwrap_or("."));
        let path = Path::new(&path_str);
        if !path.exists() {
            let to_save = toml::to_string_pretty(&self).into_diagnostic()?;
            File::create(path)
                .into_diagnostic()?
                .write_all(to_save.as_bytes())
                .into_diagnostic()?;
        }
        Ok(())
    }

    pub fn update(&self) -> miette::Result<()> {
        let to_save = toml::to_string_pretty(&self).into_diagnostic()?;
        File::create(".ciri.toml")
            .into_diagnostic()?
            .write_all(to_save.as_bytes())
            .into_diagnostic()?;
        Ok(())
    }

    pub fn read() -> miette::Result<Self> {
        let mut file: File = File::open(".ciri.toml").into_diagnostic()?;
        let mut data: String = String::new();
        file.read_to_string(&mut data).into_diagnostic()?;
        let json: Self = toml::from_str(&data).into_diagnostic()?;
        Ok(Self {
            bin_name: json.bin_name,
            prefered_project_manager: json.prefered_project_manager,
        })
    }
}

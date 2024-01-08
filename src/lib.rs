#[macro_use]
extern crate log;

pub mod args;
pub mod components;
pub mod entities;
pub mod parsers;
pub mod validators;

use std::collections::HashMap;
use std::fmt::Display;

use clap::{Args, ColorChoice, Parser, Subcommand, ValueEnum};
use lazy_static::lazy_static;

use self::args::{PackageSubCommands, SystemSubCommands};
use self::entities::managers::{Manager, CARGO_MANAGER, NPM_MANAGER};

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

#[derive(Debug, Clone, ValueEnum)]
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

    pub fn to_manager(&self) -> Manager {
        match self {
            Self::Cargo => CARGO_MANAGER,
            Self::Npm => NPM_MANAGER,
            &_ => CARGO_MANAGER,
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
        ("c", vec![".c", ".h"]),
        ("cpp", vec![".cpp", ".c++", ".hpp"]),
        (
            "dart",
            vec![
                ".dart",
                ".dart_tool", // dir
                "pubspec.yaml",
                "pubspec.yml",
                "pubspec.lock",
            ],
        ),
        (
            "go",
            vec![
                "go.mod",
                "go.sum",
                "go.work",
                "glide.yaml",
                "Gopkg.yml",
                "Gopkg.lock",
                ".go-version",
                "Godeps", // dir
                ".go",
            ],
        ),
        (
            "java",
            vec![
                "pom.xml",
                "build.gradle.kts",
                "build.sbt",
                ".java-version",
                "deps.edn",
                "project.clj",
                "build.boot",
                ".sdkmanrc",
                ".java",
                ".class",
                ".gradle",
                ".jar",
                ".clj",
                ".cljc ",
            ],
        ),
        ("kotlin", vec![".kt", ".kts"]),
        (
            "lua",
            vec![
                ".lua-version",
                "lua", // dir
                ".lua",
            ],
        ),
        (
            "bun",
            vec!["bun.lockb", "bunfig.toml", "package.json", "node_modules"],
        ),
        (
            "npm",
            vec!["package-lock.json", "package.json", "node_modules"],
        ),
        (
            "pnpm",
            vec!["pnpm-lock.yaml", "package.json", "node_modules"],
        ),
        ("yarn", vec!["yarn.lock", "package.json", "node_modules"]),
        ("php", vec!["composer.json", ".php-version", ".php"]),
        (
            // TODO: Add virtual env detection
            "python",
            vec![
                ".python-version",
                "Pipfile",
                "__init__.py",
                "pyproject.toml",
                "requirements.txt",
                "setup.py",
                "tox.ini",
                ".py",
            ],
        ),
        ("ruby", vec!["Gemfile", ".ruby-version", ".rb"]),
        (
            "scala",
            vec![
                "build.sbt",
                ".scalaenv",
                ".sbtenv",
                ".scala",
                ".sbt",
                ".metals", // dir
            ],
        ),
        ("rust", vec!["Cargo.toml", ".rs"]),
        ("swift", vec!["Package.swift", ".swift"]),
        (
            "zig",
            vec![
                ".zig",
                "zig-cache", // dir
                "zig-out",   // dir
            ],
        ),
    ]);
}

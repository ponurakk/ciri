use std::env;

use ciri::args::package::New;
use ciri::validators::inquire::{
    license_validator, lowercase_validator, url_friendliness_validator, version_validator,
};
use ciri::PackageManagers;
use clap::builder::OsStr;
use inquire::{required, Select, Text};
use miette::IntoDiagnostic;

mod cargo;
mod cpp;
mod node;

pub fn new(args: New) -> miette::Result<()> {
    match args.manager {
        PackageManagers::Npm | PackageManagers::Pnpm | PackageManagers::Yarn => {
            node::new(args)?;
            Ok(())
        }
        PackageManagers::Bun => node::new_bun(args),
        PackageManagers::Cargo => cargo::new(args),
        PackageManagers::Gpp | PackageManagers::Clangpp => cpp::new(args),
        _ => todo!(),
    }
}

pub fn prompt_version() -> miette::Result<String> {
    Text::new("version")
        .with_default("1.0.0")
        .with_validator(version_validator)
        .prompt()
        .into_diagnostic()
}

pub fn prompt_name(arg_name: Option<String>) -> miette::Result<String> {
    let mut name = Text::new("package name");
    let binding;

    let current_dir = env::current_dir().into_diagnostic()?;
    let str = OsStr::from("");
    let current_dir = current_dir
        .file_name()
        .unwrap_or(&str)
        .to_str()
        .unwrap_or("");

    if let Some(args_name) = arg_name {
        binding = args_name.clone();
        name = name.with_default(&binding);
    } else {
        name = name
            .with_default(current_dir)
            .with_validator(required!("Name can not be empty"));
    }

    name.with_validator(url_friendliness_validator)
        .with_validator(lowercase_validator)
        .prompt()
        .into_diagnostic()
}

pub fn prompt_license() -> miette::Result<String> {
    Text::new("license")
        .with_default("ISC")
        .with_validator(license_validator)
        .prompt()
        .into_diagnostic()
}

// TODO: Detect the author by other means
pub fn prompt_author() -> miette::Result<String> {
    Text::new("author").prompt().into_diagnostic()
}

pub fn prompt_description() -> miette::Result<String> {
    Text::new("description").prompt().into_diagnostic()
}

fn prompt_type<'a>() -> miette::Result<&'a str> {
    Select::new(
        "Would you like to create a binary or a library?",
        vec!["bin", "lib"],
    )
    .prompt()
    .into_diagnostic()
}

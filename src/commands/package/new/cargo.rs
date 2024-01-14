use std::process::Command;

use ciri::args::package::New;
use miette::IntoDiagnostic;

use super::{prompt_name, prompt_type};

pub fn new(args: New) -> miette::Result<()> {
    let name = if let Some(new_name) = args.name {
        new_name
    } else {
        prompt_name(args.name)?
    };

    let _type = prompt_type()?;

    Command::new("cargo")
        .arg("new")
        .arg(name)
        .arg(format!("--{}", _type))
        .output()
        .into_diagnostic()?;

    Ok(())
}

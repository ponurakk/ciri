use ciri::args::package::New;
use duct::cmd;
use miette::IntoDiagnostic;

use super::{prompt_name, prompt_type};

pub fn new(args: New) -> miette::Result<()> {
    let name = if let Some(new_name) = args.name {
        new_name
    } else {
        prompt_name(args.name)?
    };

    let _type = prompt_type()?;

    cmd!("cargo", "new", name, format!("--{}", _type))
        .run()
        .into_diagnostic()?;

    Ok(())
}

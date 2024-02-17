use ciri::args::package::New;
use ciri::Config;
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

    cmd!("cargo", "new", name.clone(), format!("--{}", _type))
        .run()
        .into_diagnostic()?;

    let config = Config::new(Some(name.clone()), None);
    config.save(Some(&name))?;

    Ok(())
}

use std::cmp::Ordering;
use std::str::FromStr;

use ciri::args::package::Run;
use ciri::validators::detect_language;
use ciri::PackageManagers;
use duct::cmd;
use miette::{bail, IntoDiagnostic};

pub fn run(args: Run) -> miette::Result<()> {
    let langs = detect_language()?;
    info!("{:#?}", langs);
    match langs.len().cmp(&1) {
        Ordering::Less => handle_none()?,
        Ordering::Equal => run_one(langs.get(0).unwrap(), args)?,
        Ordering::Greater => run_multiple(langs, args)?,
    }

    Ok(())
}

fn run_one(lang: &str, args: Run) -> miette::Result<()> {
    let bind = PackageManagers::from_str(lang)?;
    let pkg = bind.to_manager()?;
    if let Some(name) = args.name {
        let p = pkg.run.split_whitespace().collect::<Vec<_>>();
        cmd!(*p.get(0).unwrap(), p.get(1).unwrap(), name)
            .run()
            .into_diagnostic()?;
    } else {
        if let Some(default_exec) = pkg.default_exec {
            cmd!(default_exec, ".").run().into_diagnostic()?;
        } else {
            bail!("Run script or executable file not found");
        }
    }
    Ok(())
}

fn run_multiple(langs: Vec<String>, args: Run) -> miette::Result<()> {
    info!("{:#?}", langs);
    Ok(())
}

fn handle_none() -> miette::Result<()> {
    bail!("No valid package manager was detected")
}

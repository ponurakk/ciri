use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

use ciri::args::package::Run;
use ciri::entities::managers::Manager;
use ciri::validators::detect_language;
use ciri::{PackageManagers, Util};
use clap::builder::OsStr;
use duct::cmd;
use inquire::Select;
use miette::{bail, IntoDiagnostic};

pub fn run(args: Run) -> miette::Result<()> {
    let langs = detect_language()?;
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

    match bind {
        PackageManagers::Bun
        | PackageManagers::Npm
        | PackageManagers::Yarn
        | PackageManagers::Pnpm
        | PackageManagers::Cargo => run_from_manager(args, pkg),
        PackageManagers::Gpp => run_from_binary(args, pkg),
        _ => todo!(),
    }
}

fn run_from_manager(args: Run, pkg: Manager) -> miette::Result<()> {
    if let Some(name) = args.name {
        let p = pkg.run.unwrap();
        let p = p.to_tuple()?;
        cmd!(p.0, p.1, name).run().into_diagnostic()?;
    } else {
        if let Some(default_exec) = pkg.default_exec {
            let p = pkg.default_exec.unwrap();
            if let Ok(tuple) = p.to_tuple() {
                cmd!(tuple.0, tuple.1).run().into_diagnostic()?;
            } else {
                cmd!(default_exec, ".").run().into_diagnostic()?;
            }
        } else {
            bail!("Run script or executable file not found");
        }
    }

    Ok(())
}

fn run_from_binary(args: Run, pkg: Manager) -> miette::Result<()> {
    if let Some(name) = args.name {
        let p = pkg.run.unwrap();
        cmd!(format!("{}{}", p, name.display()))
            .run()
            .into_diagnostic()?;
    } else {
        if let Some(default_exec) = pkg.default_exec {
            let current_dir = env::current_dir().into_diagnostic()?;
            let str = OsStr::from("");
            let current_dir = current_dir
                .file_name()
                .unwrap_or(&str)
                .to_str()
                .unwrap_or("");

            cmd!(format!("{}{}", default_exec, current_dir))
                .run()
                .into_diagnostic()?;
        } else {
            bail!("Run script or executable file not found");
        }
    }

    Ok(())
}

fn run_multiple(langs: Vec<String>, args: Run) -> miette::Result<()> {
    let manager = Select::new("What package manager would you use?", langs)
        .prompt()
        .into_diagnostic()?;
    run_one(manager.as_str(), args)
}

fn handle_none() -> miette::Result<()> {
    bail!("No valid package manager was detected")
}

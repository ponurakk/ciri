use std::cmp::Ordering;
use std::str::FromStr;

use ciri::args::package::Build;
use ciri::entities::managers::Manager;
use ciri::validators::detect_language;
use ciri::{PackageManagers, Util};
use duct::cmd;
use inquire::Select;
use miette::{bail, IntoDiagnostic};

pub fn build(args: Build) -> miette::Result<()> {
    let langs = detect_language()?;
    match langs.len().cmp(&1) {
        Ordering::Less => handle_none()?,
        Ordering::Equal => build_one(langs.get(0).unwrap(), args)?,
        Ordering::Greater => build_multiple(langs, args)?,
    }

    Ok(())
}

fn build_one(lang: &str, args: Build) -> miette::Result<()> {
    let bind = PackageManagers::from_str(lang)?;
    let pkg = bind.to_manager()?;

    match bind {
        PackageManagers::Bun
        | PackageManagers::Npm
        | PackageManagers::Yarn
        | PackageManagers::Pnpm
        | PackageManagers::Cargo => build_from_manager(args, pkg),
        PackageManagers::Gpp => build_from_binary(args, pkg),
        _ => todo!(),
    }
}

pub fn build_from_manager(args: Build, pkg: Manager) -> miette::Result<()> {
    if let Some(_) = args.name {
        bail!("Invalid argument \"name\"");
        // if let Some(p) = pkg.build {
        //     let p = p.to_tuple()?;
        //     cmd!(p.0, p.1, name).run().into_diagnostic()?;
        // } else {
        //     run_from_script(pkg, name.to_str().unwrap_or_default())?;
        // }
    } else {
        if let Some(build) = pkg.build {
            let p = build.to_tuple()?;
            cmd!(p.0, p.1).run().into_diagnostic()?;
        } else {
            bail!("Build script or executable file not found");
        }
    }

    Ok(())
}

pub fn build_from_binary(args: Build, pkg: Manager) -> miette::Result<()> {
    if let Some(name) = args.name {
        let p = pkg.build.unwrap();
        cmd!(format!("{}{}", p, name.display()))
            .run()
            .into_diagnostic()?;
    } else {
        if let Some(build) = pkg.build {
            for x in build.split(" && ").collect::<Vec<_>>() {
                let p = x.to_tuple()?;
                cmd!(p.0, p.1).run().into_diagnostic()?;
            }
        } else {
            bail!("Build script or executable file not found");
        }
    }

    Ok(())
}

fn build_multiple(langs: Vec<String>, args: Build) -> miette::Result<()> {
    let manager = Select::new("What package manager would you use?", langs)
        .prompt()
        .into_diagnostic()?;
    build_one(manager.as_str(), args)
}

fn handle_none() -> miette::Result<()> {
    bail!("No valid package manager was detected")
}

#[cfg(test)]
#[serial_test::serial]
mod tests {

    use super::*;

    use std::env;

    fn prepare_run_test(name: &str) -> anyhow::Result<()> {
        std::fs::create_dir_all(format!("/tmp/ciri/build_test/{}", name))?;
        cmd!(
            "cp",
            "-r",
            format!("{}/example_projects/{}/.", env!("CARGO_MANIFEST_DIR"), name),
            format!("/tmp/ciri/build_test/{}", name)
        )
        .run()?;

        env::set_current_dir(format!("/tmp/ciri/build_test/{}", name))?;

        Ok(())
    }

    fn clean(name: &str) -> anyhow::Result<()> {
        std::fs::remove_dir_all(format!("/tmp/ciri/build_test/{}", name))?;
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn build_rust_test() -> anyhow::Result<()> {
        prepare_run_test("rust")?;

        let res = build(Build::new(None, None, false));
        assert!(res.is_ok());

        let res = build(Build::new(Some("example".into()), None, false));
        assert!(res.is_err());

        clean("rust")?;
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn build_node_test() -> anyhow::Result<()> {
        prepare_run_test("node")?;

        let res = build(Build::new(None, None, false));
        assert!(res.is_err());

        let res = build(Build::new(Some("example".into()), None, false));
        assert!(res.is_err());

        clean("node")?;
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn build_cpp_test() -> anyhow::Result<()> {
        prepare_run_test("cpp")?;

        let res = build(Build::new(None, None, false));
        assert!(res.is_ok());

        let res = build(Build::new(Some("example".into()), None, false));
        assert!(res.is_err());

        clean("cpp")?;
        Ok(())
    }

    #[test]
    #[should_panic]
    fn no_manager_test() {
        prepare_run_test("").unwrap();

        let res = build(Build::new(None, None, false));
        assert!(res.is_ok());
    }
}

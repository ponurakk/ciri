use std::cmp::Ordering;
use std::env;
// use std::path::PathBuf;
use std::str::FromStr;

use ciri::args::package::{Build, Run};
use ciri::entities::managers::Manager;
// use ciri::entities::manifest::PackageJson;
use ciri::validators::detect_language;
use ciri::{Config, PackageManagers, Util};
use clap::builder::OsStr;
use duct::cmd;
use inquire::Select;
use miette::{bail, Context, IntoDiagnostic};

use super::build::{build_from_binary, build_from_manager};

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
        | PackageManagers::Cargo => {
            if args.build {
                build_from_manager(Build::new(args.name.clone(), None, args.watch), pkg.clone())?;
            }
            run_from_manager(args, pkg)
        }
        PackageManagers::Gpp => {
            if args.build {
                build_from_binary(Build::new(None, None, args.watch), pkg.clone())?;
            }
            run_from_binary(args, pkg)
        }
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

// fn run_from_script(pkg: Manager, name: &str) -> miette::Result<()> {
//     if vec!["npm", "yarn", "pnpm", "bun"].contains(&pkg.agent) {
//         let package_json = PackageJson::try_from(PathBuf::from("package.json"))?;
//         let script = package_json.scripts.get(name);
//         if let Some(script) = script {
//             info!("{script:#?}");
//         }
//     }
//     Ok(())
// }

fn run_from_binary(args: Run, pkg: Manager) -> miette::Result<()> {
    if let Some(name) = args.name {
        let p = pkg.run.unwrap();
        cmd!(format!("{}{}", p, name.display()))
            .run()
            .into_diagnostic()?;
    } else {
        if let Some(default_exec) = pkg.default_exec {
            let mut config = Config::new(None);
            config.read()?;

            let current_dir = env::current_dir().into_diagnostic()?;
            let str = OsStr::from("");
            let current_dir = current_dir
                .file_name()
                .unwrap_or(&str)
                .to_str()
                .unwrap_or("");

            cmd!(format!(
                "{}{}",
                default_exec,
                config.bin_name.unwrap_or(current_dir.to_owned())
            ))
            .run()
            .into_diagnostic()
            .wrap_err(
                "Check if project was build successfully or update/set bin_name in \".ciri.toml\"",
            )
            .wrap_err("Executable wasn't found.")?;
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

#[cfg(test)]
#[serial_test::serial]
mod tests {
    use super::*;

    use std::env;

    fn prepare_run_test(name: &str) -> anyhow::Result<()> {
        std::fs::create_dir_all(format!("/tmp/ciri/run_test/{}", name))?;
        cmd!(
            "cp",
            "-r",
            format!("{}/example_projects/{}/.", env!("CARGO_MANIFEST_DIR"), name),
            format!("/tmp/ciri/run_test/{}", name)
        )
        .run()?;

        env::set_current_dir(format!("/tmp/ciri/run_test/{}", name))?;

        Ok(())
    }

    fn clean(name: &str) -> anyhow::Result<()> {
        std::fs::remove_dir_all(format!("/tmp/ciri/run_test/{}", name))?;
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn run_rust_test() -> anyhow::Result<()> {
        prepare_run_test("rust")?;

        let res = run(Run::new(None, false, false));
        assert!(res.is_ok());

        let res = run(Run::new(None, true, false));
        assert!(res.is_ok());

        let res = run(Run::new(Some("example".into()), false, false));
        assert!(res.is_ok());

        let res = run(Run::new(Some("example".into()), true, false));
        assert!(res.is_err());

        clean("rust")?;
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn run_node_test() -> anyhow::Result<()> {
        prepare_run_test("node")?;

        let res = run(Run::new(None, false, false));
        assert!(res.is_ok());

        let res = run(Run::new(None, true, false));
        assert!(res.is_err());

        let res = run(Run::new(Some("build".into()), false, false));
        assert!(res.is_ok());

        let res = run(Run::new(Some("build".into()), true, false));
        assert!(res.is_err());

        clean("node")?;
        Ok(())
    }

    // NOTE: This test will fail for now because of incorect
    // binary name for cpp.
    #[test]
    #[serial_test::serial]
    fn run_cpp_test() -> anyhow::Result<()> {
        prepare_run_test("cpp")?;

        let res = run(Run::new(None, false, false));
        assert!(res.is_err());

        let res = run(Run::new(None, true, false));
        assert!(res.is_err());

        let res = run(Run::new(Some("example".into()), false, false));
        assert!(res.is_ok());

        let res = run(Run::new(Some("example".into()), true, false));
        assert!(res.is_ok());

        clean("cpp")?;
        Ok(())
    }

    #[test]
    #[should_panic]
    fn no_manager_test() {
        prepare_run_test("").unwrap();

        let res = run(Run::new(None, false, false));
        assert!(res.is_ok());
    }
}

use std::cmp::Ordering;
use std::str::FromStr;

use ciri::validators::detect_language;
use ciri::PackageManagers;
use inquire::Select;
use miette::{bail, IntoDiagnostic};

pub fn clean() -> miette::Result<()> {
    let langs = detect_language()?;
    match langs.len().cmp(&1) {
        Ordering::Less => handle_none()?,
        Ordering::Equal => clean_one(langs.get(0).unwrap())?,
        Ordering::Greater => clean_multiple(langs)?,
    }

    Ok(())
}

fn clean_one(lang: &str) -> miette::Result<()> {
    let bind = PackageManagers::from_str(lang)?;

    let folder = match bind {
        PackageManagers::Bun
        | PackageManagers::Npm
        | PackageManagers::Yarn
        | PackageManagers::Pnpm => "node_modules",
        PackageManagers::Cargo => "target",
        PackageManagers::Gpp => "build",
        _ => todo!(),
    };

    std::fs::remove_dir_all(format!("./{}", folder)).into_diagnostic()?;
    Ok(())
}

fn clean_multiple(langs: Vec<String>) -> miette::Result<()> {
    let manager = Select::new("What package manager would you use?", langs)
        .prompt()
        .into_diagnostic()?;
    clean_one(manager.as_str())
}

fn handle_none() -> miette::Result<()> {
    bail!("No valid package manager was detected")
}

#[cfg(test)]
#[serial_test::serial]
mod tests {

    use crate::commands::package::build;
    use ciri::args::package::Build;

    use super::*;

    use duct::cmd;
    use std::env;

    fn prepare_run_test(name: &str) -> anyhow::Result<()> {
        std::fs::create_dir_all(format!("/tmp/ciri/clean_test/{}", name))?;
        cmd!(
            "cp",
            "-r",
            format!("{}/example_projects/{}/.", env!("CARGO_MANIFEST_DIR"), name),
            format!("/tmp/ciri/clean_test/{}", name)
        )
        .run()?;

        env::set_current_dir(format!("/tmp/ciri/clean_test/{}", name))?;

        Ok(())
    }

    fn clean_test(name: &str) -> anyhow::Result<()> {
        std::fs::remove_dir_all(format!("/tmp/ciri/clean_test/{}", name))?;
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn clean_rust_test() -> anyhow::Result<()> {
        prepare_run_test("rust")?;

        build(Build {
            name: None,
            script: None,
            watch: false,
        })
        .unwrap();
        let res = clean();
        assert!(res.is_ok());

        clean_test("rust")?;
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn clean_node_test() -> anyhow::Result<()> {
        prepare_run_test("node")?;

        // Imitate installing packages
        std::fs::create_dir("node_modules")?;

        let res = clean();
        assert!(res.is_ok());

        clean_test("node")?;
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn clean_cpp_test() -> anyhow::Result<()> {
        prepare_run_test("cpp")?;

        build(Build {
            name: None,
            script: None,
            watch: false,
        })
        .unwrap();
        let res = clean();
        assert!(res.is_ok());

        clean_test("cpp")?;
        Ok(())
    }

    #[test]
    #[should_panic]
    fn no_manager_test() {
        prepare_run_test("").unwrap();

        let res = clean();
        assert!(res.is_ok());
    }
}

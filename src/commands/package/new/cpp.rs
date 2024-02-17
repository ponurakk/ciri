use std::fs::{self, File};
use std::io::Write;

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

    let folder = if _type == "bin" {
        "src"
    } else if _type == "lib" {
        "lib"
    } else {
        "src"
    };

    create_structure(&name, folder)?;
    create_files(&name, folder)?;
    Ok(())
}

fn create_structure(name: &str, folder: &str) -> miette::Result<()> {
    fs::create_dir_all(format!("{}/{}", name, folder)).into_diagnostic()?;
    fs::create_dir(format!("{}/include", name)).into_diagnostic()?;
    let config = Config::new(Some(name.to_owned()), None);
    config.save(Some(name))?;
    Ok(())
}

fn create_files(name: &str, folder: &str) -> miette::Result<()> {
    File::create(format!("{}/{}/main.cpp", name, folder))
        .into_diagnostic()?
        .write_all(
            br#"#include <iostream>

int main(int argc, char *argv[]) {
  std::cout << "Hello, World!" << std::endl;
  return 0;
}"#,
        )
        .into_diagnostic()?;

    File::create(format!("{}/CMakeLists.txt", name))
        .into_diagnostic()?
        .write_all(
            format!(
                "cmake_minimum_required(VERSION 3.21)
project({} VERSION 0.1.0 LANGUAGES CXX)

set(SOURCE_FILES
  {}/main.cpp
)

add_executable(${{PROJECT_NAME}} ${{SOURCE_FILES}})

target_include_directories(${{PROJECT_NAME}} PRIVATE include)",
                name, folder
            )
            .as_bytes(),
        )
        .into_diagnostic()?;

    File::create(format!("{}/.gitignore", name))
        .into_diagnostic()?
        .write_all(b"/build")
        .into_diagnostic()?;

    cmd!("git", "init", name)
        .stdout_null()
        .stderr_null()
        .run()
        .into_diagnostic()?;

    Ok(())
}

use std::fs::File;
use std::io::Write;

use ciri::args::package::New;
use ciri::entities::manifest::PackageJson;
use inquire::{Select, Text};
use miette::IntoDiagnostic;

use crate::commands::package::new::{prompt_name, prompt_version};

use super::{prompt_author, prompt_description, prompt_license};

pub struct ManifestQuestions {
    name: String,
    version: String,
    description: String,
    entry: String,
    author: String,
    license: String,
}

/// Get language from user
fn prompt_language<'a>() -> miette::Result<&'a str> {
    Select::new(
        "What language would you like to choose?",
        vec!["javascript", "typescript"],
    )
    .prompt()
    .into_diagnostic()
}

/// Get entry based on chosen language
fn prompt_entry(ts_js: &str) -> miette::Result<String> {
    let mut entry = Text::new("entry point");

    match ts_js {
        "typescript" => entry = entry.with_default("index.ts"),
        "javascript" => entry = entry.with_default("index.js"),
        &_ => {}
    }
    entry.prompt().into_diagnostic()
}

/// Prompt user for manifest configuartions
pub fn get_values(args: New) -> miette::Result<ManifestQuestions> {
    let name = prompt_name(args.name)?;
    let version;
    let description;
    let entry;
    let author;
    let license;
    if args.defaults {
        version = "1.0.0".to_owned();
        description = "".to_owned();
        entry = "index.js".to_owned();
        author = "".to_owned();
        license = "ISC".to_owned();
    } else {
        let ts_js = prompt_language()?;
        version = prompt_version()?;
        description = prompt_description()?;
        entry = prompt_entry(ts_js)?;
        author = prompt_author()?;
        license = prompt_license()?;
    }

    Ok(ManifestQuestions {
        name,
        version,
        description,
        entry,
        author,
        license,
    })
}

/// Creates new node project
///
/// Returns true if typescript project was created
pub fn new(args: New) -> miette::Result<bool> {
    // Package shell complete
    // let pkg = Command::new("all-the-package-names")
    //     .output()
    //     .into_diagnostic()?;
    // let out = from_utf8(&pkg.stdout).into_diagnostic()?;
    // let packages: Vec<&str> = out.split("\n").collect();
    // let filtered: Vec<&str> = packages
    //     .iter()
    //     .filter(|package| package.starts_with("testzz"))
    //     .cloned()
    //     .collect();
    // println!("{:#?}", filtered);

    let manager = args.manager.to_string();
    let values = get_values(args)?;
    let entry = values.entry.clone();
    let package_json = PackageJson::new(
        values.name,
        values.version,
        values.description,
        values.entry,
        values.author,
        values.license,
        manager,
    );
    let package_json_str = serde_json::to_string_pretty(&package_json).into_diagnostic()?;

    File::create("package.json")
        .into_diagnostic()?
        .write_all(package_json_str.as_bytes())
        .into_diagnostic()?;

    File::create(&entry)
        .into_diagnostic()?
        .write_all(b"console.log(\"Hello, World!\");")
        .into_diagnostic()?;

    Ok(entry.ends_with(".ts"))
}

pub fn new_bun(args: New) -> miette::Result<()> {
    new(args)?;

    File::create("tsconfig.json")
        .into_diagnostic()?
        .write_all(
            br#"{
  "compilerOptions": {
    "lib": ["ESNext"],
    "module": "esnext",
    "target": "esnext",
    "moduleResolution": "bundler",
    "moduleDetection": "force",
    "allowImportingTsExtensions": true,
    "noEmit": true,
    "composite": true,
    "strict": true,
    "downlevelIteration": true,
    "skipLibCheck": true,
    "jsx": "react-jsx",
    "allowSyntheticDefaultImports": true,
    "forceConsistentCasingInFileNames": true,
    "allowJs": true,
    "types": [
      "bun-types" // add Bun global
    ]
  }
}"#,
        )
        .into_diagnostic()?;

    Ok(())
}

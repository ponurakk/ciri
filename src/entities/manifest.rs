use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PackageJson {
    name: String,
    version: String,
    description: String,
    main: String,
    pub scripts: HashMap<String, String>,
    keywords: Vec<String>,
    author: String,
    license: String,
}

impl PackageJson {
    pub fn new(
        name: String,
        version: String,
        description: String,
        main: String,
        author: String,
        license: String,
        pkg: String,
    ) -> Self {
        Self {
            name,
            version,
            description,
            main,
            scripts: HashMap::from([
                (
                    "test".to_owned(),
                    "echo \"Error: no test specified\" && exit 1".to_owned(),
                ),
                ("preinstall".to_owned(), format!("npx only-allow {}", pkg)),
            ]),
            keywords: vec![],
            author,
            license,
        }
    }
}

impl TryFrom<PathBuf> for PackageJson {
    type Error = miette::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let mut file: File = File::open(value).into_diagnostic()?;
        let mut data: String = String::new();
        file.read_to_string(&mut data).into_diagnostic()?;
        let json: Self = serde_json::from_str(&data).into_diagnostic()?;
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_json_creation() {
        let pkg_name = String::from("my_package");
        let pkg_version = String::from("1.0.0");
        let pkg_description = String::from("A test package");
        let pkg_main = String::from("index.js");
        let pkg_author = String::from("John Doe");
        let pkg_license = String::from("MIT");
        let pkg_name_for_test = String::from("test-package");

        let package_json = PackageJson::new(
            pkg_name.clone(),
            pkg_version.clone(),
            pkg_description.clone(),
            pkg_main.clone(),
            pkg_author.clone(),
            pkg_license.clone(),
            pkg_name_for_test.clone(),
        );

        assert_eq!(package_json.name, pkg_name);
        assert_eq!(package_json.version, pkg_version);
        assert_eq!(package_json.description, pkg_description);
        assert_eq!(package_json.main, pkg_main);
        assert_eq!(
            package_json.scripts.get("test").unwrap(),
            &"echo \"Error: no test specified\" && exit 1".to_owned()
        );
        assert_eq!(
            package_json.scripts.get("preinstall").unwrap(),
            &format!("npx only-allow {}", pkg_name_for_test)
        );
        assert_eq!(package_json.keywords, Vec::<String>::new());
        assert_eq!(package_json.author, pkg_author);
        assert_eq!(package_json.license, pkg_license);
    }
}

use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct PackageJson {
    name: String,
    version: String,
    description: String,
    main: String,
    scripts: HashMap<String, String>,
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

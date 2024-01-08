use std::fs;

use crate::LANGUAGES;

pub fn detect_language() -> Vec<&'static str> {
    let paths = fs::read_dir("./")
        .unwrap()
        .map(|v| v.unwrap().path().display().to_string())
        .collect::<Vec<_>>();

    let mut managers: Vec<&str> = Vec::new();

    for (key, val) in LANGUAGES.iter() {
        if paths.iter().any(|v| val.contains(&&v[2..])) {
            managers.push(key);
        }
    }

    managers
}

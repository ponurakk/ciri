use clap::builder::OsStr;

use crate::LANGUAGES;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn detect_language() -> miette::Result<Vec<String>> {
    let paths = fs::read_dir("./")
        .unwrap()
        .map(|v| v.unwrap().path().display().to_string())
        .collect::<Vec<_>>();

    let mut managers: Vec<&str> = Vec::new();

    for (key, val) in LANGUAGES.iter() {
        if paths
            .iter()
            .filter(|v| val.contains(&format!("{}", v).strip_prefix("./").unwrap()))
            .any(|v| val.contains(&&v[2..]))
        {
            managers.push(key);
        }
    }

    filter_false(&managers, &paths)
}

fn filter_false(langs: &Vec<&str>, paths: &Vec<String>) -> miette::Result<Vec<String>> {
    let mut hash: HashMap<&str, usize> = HashMap::new();
    langs.iter().for_each(|lang| {
        hash.insert(lang, count_paths_for_language(lang, paths));
    });

    let max_value = hash.values().cloned().max();
    match max_value {
        Some(max_value) => Ok(hash
            .iter()
            .filter(|v| *v.1 == max_value)
            .map(|k| format!("{}", k.0))
            .collect()),
        None => miette::bail!("No manager found"),
    }
}

fn count_paths_for_language(language: &str, paths: &Vec<String>) -> usize {
    paths
        .iter()
        .filter(|v| {
            LANGUAGES.get(language).unwrap_or(&vec![]).contains(
                &Path::new(v)
                    .file_name()
                    .unwrap_or(&OsStr::from(""))
                    .to_str()
                    .unwrap_or(""),
            )
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_false_test() {
        let mut managers_same_file = filter_false(
            &vec!["npm", "pnpm", "yarn"],
            &vec!["package.json".to_owned()],
        )
        .unwrap();
        // Sort is needed for the same result every time
        managers_same_file.sort();
        assert_eq!(
            managers_same_file,
            vec!["npm".to_owned(), "pnpm".to_owned(), "yarn".to_owned()]
        );

        let mut managers_one_match = filter_false(
            &vec!["npm", "pnpm", "yarn"],
            &vec!["pnpm-lock.yaml".to_owned()],
        )
        .unwrap();
        managers_one_match.sort();
        assert_eq!(managers_one_match, vec!["pnpm".to_owned()]);

        let mut managers_multiple_matches = filter_false(
            &vec!["npm", "pnpm", "yarn"],
            &vec!["package-lock.json".to_owned(), "pnpm-lock.yaml".to_owned()],
        )
        .unwrap();
        managers_multiple_matches.sort();
        assert_eq!(
            managers_multiple_matches,
            vec!["npm".to_owned(), "pnpm".to_owned()]
        );

        let mut managers_multiple_matches = filter_false(
            &vec!["cargo", "npm"],
            &vec!["package-lock.json".to_owned(), "Cargo.toml".to_owned()],
        )
        .unwrap();
        managers_multiple_matches.sort();
        assert_eq!(
            managers_multiple_matches,
            vec!["cargo".to_owned(), "npm".to_owned()]
        );
    }

    #[test]
    fn count_paths_for_language_test() {
        let paths_number = count_paths_for_language(
            "npm",
            &vec!["./package.json".to_owned(), "package-lock.json".to_owned()],
        );
        assert_eq!(paths_number, 2);

        let no_paths = count_paths_for_language("npm", &vec![]);
        assert_eq!(no_paths, 0);

        let invalid_manager = count_paths_for_language(
            "invalid",
            &vec!["./package.json".to_owned(), "package-lock.json".to_owned()],
        );
        assert_eq!(invalid_manager, 0);

        let paths_number = count_paths_for_language("cargo", &vec!["main.rs".to_owned()]);
        assert_eq!(paths_number, 1);
    }
}

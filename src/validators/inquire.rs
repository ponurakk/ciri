use std::error::Error;

use inquire::validator::Validation;
use nom::AsChar;
use spdx::Expression;

use crate::parsers::url_friendliness_parser;
use crate::parsers::version::full_version_parser;

type ValidateResult = Result<Validation, Box<dyn Error + Send + Sync>>;

pub fn license_validator(input: &str) -> ValidateResult {
    match Expression::parse(input) {
        Ok(_) => Ok(Validation::Valid),
        Err(_) => Ok(Validation::Invalid(
            "License should be in a valid SPXD Expression".into(),
        )),
    }
}

pub fn version_validator(input: &str) -> ValidateResult {
    match full_version_parser(input) {
        Ok(_) => Ok(Validation::Valid),
        Err(_) => Ok(Validation::Invalid("Invalid version format".into())),
    }
}

pub fn url_friendliness_validator(input: &str) -> ValidateResult {
    match url_friendliness_parser(input) {
        Ok(_) => Ok(Validation::Valid),
        Err(_) => Ok(Validation::Invalid(
            r#"Name can only contain URL-friendly characters. (a-z, 0-9, "-", ".", "_", "~")"#
                .into(),
        )),
    }
}

pub fn lowercase_validator(input: &str) -> ValidateResult {
    match input
        .chars()
        .filter(|char| char.is_alpha())
        .all(|char| char.is_lowercase())
    {
        true => Ok(Validation::Valid),
        false => Ok(Validation::Invalid(
            r#"Name cannot have capital letters"#.into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn license_validator_test() {
        let input = "MIT AND Apache-2.0";
        assert_eq!(license_validator(input).unwrap(), Validation::Valid);

        let input = "Invalid license";
        assert_ne!(license_validator(input).unwrap(), Validation::Valid);
    }

    #[test]
    fn version_validator_test() {
        let input = "1.0.0-DEV";
        assert_eq!(version_validator(input).unwrap(), Validation::Valid);

        let input = "Invalid version";
        assert_ne!(version_validator(input).unwrap(), Validation::Valid);
    }

    #[test]
    fn url_friendliness_validator_test() {
        let input = "ciri";
        assert_eq!(
            url_friendliness_validator(input).unwrap(),
            Validation::Valid
        );

        let input = "ciri#1";
        assert_ne!(
            url_friendliness_validator(input).unwrap(),
            Validation::Valid
        );
    }

    #[test]
    fn lowercase_validator_test() {
        let input = "ciri";
        assert_eq!(lowercase_validator(input).unwrap(), Validation::Valid);

        let input = "Ciri";
        assert_ne!(lowercase_validator(input).unwrap(), Validation::Valid);

        let input = "ciri#";
        assert_eq!(lowercase_validator(input).unwrap(), Validation::Valid);
    }
}

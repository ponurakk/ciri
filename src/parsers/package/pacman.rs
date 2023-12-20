use std::borrow::ToOwned;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, anychar, one_of, space0};
use nom::combinator::{map, opt, rest};
use nom::error::{context, VerboseError};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn valid_string(input: &str) -> Res<&str, String> {
    context(
        "Valid String",
        map(
            many1(one_of(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.",
            )),
            |v| v.iter().cloned().collect::<String>(),
        ),
    )(input)
}

fn dependency_description(input: &str) -> Res<&str, (Option<String>, bool)> {
    context(
        "Dependency Description",
        map(tuple((opt(take_until("[")), rest)), |(v1, v2)| {
            let mut desc: Option<String> = v1.map(|v: &str| v.trim().to_owned());
            let mut is_installed = false;
            if v2 == "[installed]" {
                is_installed = true;
                if v1 == Some(" ") {
                    desc = None;
                }
            } else {
                if v2 == "" {
                    desc = None;
                } else {
                    desc = Some(v2.to_owned());
                }
            }

            (desc, is_installed)
        }),
    )(input)
}

fn optional_dependency(input: &str) -> Res<&str, (String, Option<String>, bool)> {
    context(
        "Optional Dependency",
        map(
            separated_pair(
                tuple((opt(double_word), many1(tag(" ")))),
                opt(tag(": ")),
                separated_pair(valid_string, opt(tag(": ")), dependency_description),
            ),
            |(_, (v1, v2))| (v1.to_owned(), v2.0, v2.1),
        ),
    )(input)
}

fn double_word(input: &str) -> Res<&str, (&str, Option<&str>)> {
    context(
        "Double Word",
        separated_pair(alpha1, opt(tag(" ")), opt(alpha1)),
    )(input)
}

fn field(input: &str) -> Res<&str, (String, Option<String>, bool)> {
    context(
        "Field",
        alt((
            map(
                separated_pair(
                    map(double_word, |v| {
                        format!(
                            "{}{}{}",
                            v.0,
                            if v.1.unwrap_or("") != "" { " " } else { "" },
                            v.1.unwrap_or("")
                        )
                    }),
                    delimited(space0, tag(":"), space0),
                    map(many0(anychar), |v| {
                        Some(v.iter().cloned().collect::<String>())
                    }),
                ),
                |(v1, v2)| (v1, v2, false),
            ),
            optional_dependency,
        )),
    )(input)
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub architecture: String,
    pub url: Option<String>,
    pub licenses: Vec<String>,
    pub groups: Option<String>,
    pub provides: Option<String>,
    pub depends_on: Option<String>,
    pub optional_deps: Option<Vec<OptionalDependency>>,
    pub required_by: Option<String>,
    pub optional_for: Option<String>,
    pub conflicts_with: Option<String>,
    pub replaces: Option<String>,
    pub installed_size: String,
    pub packager: String,
    pub build_date: String,
    pub install_date: String,
    pub install_reason: String,
    pub install_script: String,
    pub validated_by: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct OptionalDependency {
    pub name: String,
    pub description: Option<String>,
    pub is_installed: bool,
}

impl Package {
    fn add(&mut self, input: String, value: String) {
        let value = if value == "None".to_string() {
            None
        } else {
            Some(value.to_string())
        };

        match input.as_str() {
            "Name" => self.name = value.expect("Error in parsing name"),
            "Version" => self.version = value.expect("Error in parsing version"),
            "Description" => self.description = value.expect("Error in parsing description"),
            "Architecture" => self.architecture = value.expect("Error in parsing architecture"),
            "URL" => self.url = value,
            "Licenses" => {
                self.licenses = value
                    .expect("Error in parsing licenses")
                    .split(" ")
                    .map(ToOwned::to_owned)
                    .collect();

                self.licenses.retain_mut(|s| !s.is_empty());
            }
            "Groups" => self.groups = value,
            "Provides" => self.provides = value,
            "Depends On" => self.depends_on = value,
            "Optional Deps" | "" => {
                if self.optional_deps.is_none() && value.is_some() {
                    self.optional_deps = Some(Vec::new());
                }

                if value.is_none() {
                    return;
                }

                let str = format!(" : {}", value.unwrap());
                let parsed = optional_dependency(&str).unwrap();
                let info = parsed.1;

                self.optional_deps
                    .as_mut()
                    .unwrap()
                    .push(OptionalDependency {
                        name: info.0,
                        description: info.1,
                        is_installed: info.2,
                    });
            }
            "Required By" => self.required_by = value,
            "Optional For" => self.optional_for = value,
            "Conflicts With" => self.conflicts_with = value,
            "Replaces" => self.replaces = value,
            "Installed Size" => {
                self.installed_size = value.expect("Error in parsing installed size")
            }
            "Packager" => self.packager = value.expect("Error in parsing packager info"),
            "Build Date" => self.build_date = value.expect("Error in parsing build date"),
            "Install Date" => self.install_date = value.expect("Error in parsing install date"),
            "Install Reason" => {
                self.install_reason = value.expect("Error in parsing install reason")
            }
            "Install Script" => {
                self.install_script = value.expect("Error in parsing install script")
            }
            "Validated By" => self.validated_by = value,
            &_ => {
                println!("{:#?} {:#?}", input, value);
            }
        }
    }
}

pub fn package(input: &str) -> Package {
    let mut pkg: Package = Package::default();

    input.split("\n").for_each(|v| {
        let current_field = field(v).unwrap();
        if current_field
            .1
             .0
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
        {
            pkg.add(current_field.1 .0, current_field.1 .1.unwrap());
        } else {
            pkg.add(
                "".to_owned(),
                format!(
                    "{}: {}",
                    current_field.1 .0,
                    current_field.1 .1.unwrap_or("".to_owned())
                ),
            );
        }
    });

    pkg
}

pub fn packages(input: &str) -> Vec<Package> {
    let mut vec: Vec<Package> = Vec::new();

    input.split("\n\n").for_each(|v| {
        vec.push(package(v));
    });

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_with_space_test() {
        let input = "Hello World";
        assert_eq!(double_word(input), Ok(("", ("Hello", Some("World")))));
    }

    #[test]
    fn word_without_space_test() {
        let input = "HelloWorld";
        assert_eq!(double_word(input), Ok(("", ("HelloWorld", None))));
    }

    #[test]
    fn one_line_test() {
        let input = "Name            : pkg";
        assert_eq!(
            field(input),
            Ok(("", ("Name".to_string(), Some("pkg".to_string()), false)))
        );

        let input = "Name            :      pkg2";
        assert_eq!(
            field(input),
            Ok(("", ("Name".to_string(), Some("pkg2".to_string()), false)))
        );
    }

    #[test]
    fn one_line_version_test() {
        let input = "Version            : 2.2.1-1";
        assert_eq!(
            field(input),
            Ok((
                "",
                ("Version".to_string(), Some("2.2.1-1".to_string()), false)
            ))
        );
    }

    #[test]
    fn one_line_name_space_test() {
        let input = "Install Script  : No";
        assert_eq!(
            field(input),
            Ok((
                "",
                ("Install Script".to_owned(), Some("No".to_owned()), false)
            ))
        );
    }

    #[test]
    fn optional_dependency_test() {
        assert_eq!(
            optional_dependency("Optional Deps   : dep: dependency description"),
            Ok((
                "",
                (
                    "dep".to_owned(),
                    Some("dependency description".to_owned()),
                    false
                )
            ))
        );

        assert_eq!(
            optional_dependency("Optional Deps   : dep: dependency description [installed]"),
            Ok((
                "",
                (
                    "dep".to_owned(),
                    Some("dependency description".to_owned()),
                    true
                )
            ))
        );

        assert_eq!(
            optional_dependency("                  : dep: dependency description"),
            Ok((
                "",
                (
                    "dep".to_owned(),
                    Some("dependency description".to_owned()),
                    false
                )
            ))
        );

        assert_eq!(
            optional_dependency("                  : dep: dependency description [installed]"),
            Ok((
                "",
                (
                    "dep".to_owned(),
                    Some("dependency description".to_owned()),
                    true
                )
            ))
        );

        assert_eq!(
            optional_dependency("                  : dep"),
            Ok(("", ("dep".to_owned(), None, false)))
        );

        assert_eq!(
            optional_dependency("                  : dep [installed]"),
            Ok(("", ("dep".to_owned(), None, true)))
        );
    }

    #[test]
    fn full_package_test() {
        let input = r#"Name            : pkg
Version         : 2.2.1-1
Description     : Some package description
Architecture    : x86_64
URL             : https://example.com/pkg
Licenses        : Apache
Groups          : None
Provides        : None
Depends On      : otherdependency1
Optional Deps   : dep: my description [installed]
                : dep2: my other description
Required By     : otherdependency2
Optional For    : None
Conflicts With  : None
Replaces        : None
Installed Size  : 2137.69 KiB
Packager        : My Name <user@example.com>
Build Date      : Mon 01 Jan 1970 00:00:00 AM CET
Install Date    : Mon 01 Jan 1970 00:00:00 PM CET
Install Reason  : Installed as a dependency for another package
Install Script  : No
Validated By    : Signature"#;
        assert_eq!(
            package(input),
            Package {
                name: "pkg".to_string(),
                version: "2.2.1-1".to_string(),
                description: "Some package description".to_string(),
                architecture: "x86_64".to_string(),
                url: Some("https://example.com/pkg".to_string()),
                licenses: vec!["Apache".to_string()],
                groups: None,
                provides: None,
                depends_on: Some("otherdependency1".to_string()),
                optional_deps: Some(vec![
                    OptionalDependency {
                        name: "dep".to_owned(),
                        description: Some("my description".to_owned()),
                        is_installed: true,
                    },
                    OptionalDependency {
                        name: "dep2".to_owned(),
                        description: Some("my other description".to_owned()),
                        is_installed: false,
                    }
                ]),
                required_by: Some("otherdependency2".to_string()),
                optional_for: None,
                conflicts_with: None,
                replaces: None,
                installed_size: "2137.69 KiB".to_string(),
                packager: "My Name <user@example.com>".to_string(),
                build_date: "Mon 01 Jan 1970 00:00:00 AM CET".to_string(),
                install_date: "Mon 01 Jan 1970 00:00:00 PM CET".to_string(),
                install_reason: "Installed as a dependency for another package".to_string(),
                install_script: "No".to_string(),
                validated_by: Some("Signature".to_string()),
            },
        );
    }

    #[test]
    fn multiple_packages_test() {
        let input = r#"Name            : pkg
Version         : 2.2.1-1
Description     : Some package description
Architecture    : x86_64
URL             : https://example.com/pkg
Licenses        : Apache MIT
Groups          : None
Provides        : None
Depends On      : otherdependency1
Optional Deps   : None
Required By     : otherdependency2
Optional For    : None
Conflicts With  : None
Replaces        : None
Installed Size  : 2137.69 KiB
Packager        : My Name <user@example.com>
Build Date      : Mon 01 Jan 1970 00:00:00 AM CET
Install Date    : Mon 01 Jan 1970 00:00:00 PM CET
Install Reason  : Installed as a dependency for another package
Install Script  : No
Validated By    : Signature

Name            : pkg2
Version         : 2.2.1-1
Description     : Some package description
Architecture    : x86_64
URL             : https://example.com/pkg
Licenses        : GPL
Groups          : None
Provides        : None
Depends On      : otherdependency1
Optional Deps   : somedep1: somedep1 description [installed]
                  somedep2: somedep2 description
Required By     : otherdependency2
Optional For    : None
Conflicts With  : None
Replaces        : None
Installed Size  : 2137.69 KiB
Packager        : My Name <user@example.com>
Build Date      : Mon 01 Jan 1970 00:00:00 AM CET
Install Date    : Mon 01 Jan 1970 00:00:00 PM CET
Install Reason  : Installed as a dependency for another package
Install Script  : No
Validated By    : Signature"#;
        assert_eq!(
            packages(input),
            vec![
                Package {
                    name: "pkg".to_string(),
                    version: "2.2.1-1".to_string(),
                    description: "Some package description".to_string(),
                    architecture: "x86_64".to_string(),
                    url: Some("https://example.com/pkg".to_string()),
                    licenses: vec!["Apache".to_string(), "MIT".to_string()],
                    groups: None,
                    provides: None,
                    depends_on: Some("otherdependency1".to_string()),
                    optional_deps: None,
                    required_by: Some("otherdependency2".to_string()),
                    optional_for: None,
                    conflicts_with: None,
                    replaces: None,
                    installed_size: "2137.69 KiB".to_string(),
                    packager: "My Name <user@example.com>".to_string(),
                    build_date: "Mon 01 Jan 1970 00:00:00 AM CET".to_string(),
                    install_date: "Mon 01 Jan 1970 00:00:00 PM CET".to_string(),
                    install_reason: "Installed as a dependency for another package".to_string(),
                    install_script: "No".to_string(),
                    validated_by: Some("Signature".to_string()),
                },
                Package {
                    name: "pkg2".to_string(),
                    version: "2.2.1-1".to_string(),
                    description: "Some package description".to_string(),
                    architecture: "x86_64".to_string(),
                    url: Some("https://example.com/pkg".to_string()),
                    licenses: vec!["GPL".to_string()],
                    groups: None,
                    provides: None,
                    depends_on: Some("otherdependency1".to_string()),
                    optional_deps: Some(vec![
                        OptionalDependency {
                            name: "somedep1".to_string(),
                            description: Some("somedep1 description".to_string()),
                            is_installed: true
                        },
                        OptionalDependency {
                            name: "somedep2".to_string(),
                            description: Some("somedep2 description".to_string()),
                            is_installed: false
                        }
                    ]),
                    required_by: Some("otherdependency2".to_string()),
                    optional_for: None,
                    conflicts_with: None,
                    replaces: None,
                    installed_size: "2137.69 KiB".to_string(),
                    packager: "My Name <user@example.com>".to_string(),
                    build_date: "Mon 01 Jan 1970 00:00:00 AM CET".to_string(),
                    install_date: "Mon 01 Jan 1970 00:00:00 PM CET".to_string(),
                    install_reason: "Installed as a dependency for another package".to_string(),
                    install_script: "No".to_string(),
                    validated_by: Some("Signature".to_string()),
                }
            ]
        );
    }
}

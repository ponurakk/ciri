use nom::branch::alt;
use nom::character::complete::{alphanumeric1, char, digit1};
use nom::combinator::{map, map_res, opt};
use nom::error::context;
use nom::multi::many1;
use nom::sequence::{separated_pair, tuple};

use super::Res;

type VersionIdentifier<'a> = Option<Vec<(String, &'a str)>>;

pub fn version_identifier_parser(input: &str) -> Res<&str, (u64, VersionIdentifier)> {
    context(
        "Version Identifier",
        tuple((
            map_res(digit1, |s: &str| s.parse::<u64>()),
            opt(many1(tuple((
                map(many1(alt((char('-'), char('+'), char('.')))), |v| {
                    v.into_iter().collect::<String>()
                }),
                alphanumeric1,
            )))),
        )),
    )(input)
}

pub fn full_version_parser(input: &str) -> Res<&str, (u64, u64, u64, VersionIdentifier)> {
    context(
        "Full Version",
        map(
            separated_pair(
                map_res(digit1, |s: &str| s.parse::<u64>()),
                char('.'),
                separated_pair(
                    map_res(digit1, |s: &str| s.parse::<u64>()),
                    char('.'),
                    version_identifier_parser,
                ),
            ),
            |v| (v.0, v.1 .0, v.1 .1 .0, v.1 .1 .1),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_version_parser_test() {
        let input = "1.0.0";
        assert_eq!(full_version_parser(input), Ok(("", (1, 0, 0, None))));

        let input = "1.0.0-prerelease+meta";
        assert_eq!(
            full_version_parser(input),
            Ok((
                "",
                (
                    1,
                    0,
                    0,
                    Some(vec![
                        ("-".to_owned(), "prerelease"),
                        ("+".to_owned(), "meta")
                    ])
                )
            ))
        );
        let input = "1.0.0+meta";
        assert_eq!(
            full_version_parser(input),
            Ok(("", (1, 0, 0, Some(vec![("+".to_owned(), "meta")]))))
        );
        let input = "1.2.3----RC-SNAPSHOT.12.9.1--.12+788";
        assert_eq!(
            full_version_parser(input),
            Ok((
                "",
                (
                    1,
                    2,
                    3,
                    Some(vec![
                        ("----".to_owned(), "RC"),
                        ("-".to_owned(), "SNAPSHOT"),
                        (".".to_owned(), "12"),
                        (".".to_owned(), "9"),
                        (".".to_owned(), "1"),
                        ("--.".to_owned(), "12"),
                        ("+".to_owned(), "788")
                    ])
                )
            ))
        );

        let input = "1.0.0-rc.1+build.1";
        assert_eq!(
            full_version_parser(input),
            Ok((
                "",
                (
                    1,
                    0,
                    0,
                    Some(vec![
                        ("-".to_owned(), "rc"),
                        (".".to_owned(), "1"),
                        ("+".to_owned(), "build"),
                        (".".to_owned(), "1")
                    ])
                )
            ))
        )
    }
}

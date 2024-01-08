use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::combinator::all_consuming;
use nom::error::{context, VerboseError};
use nom::multi::many1;
use nom::IResult;

pub mod system;
pub mod version;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn url_friendliness_parser(input: &str) -> Res<&str, Vec<&str>> {
    context(
        "Url",
        all_consuming(many1(alt((
            alphanumeric1,
            tag("-"),
            tag("."),
            tag("_"),
            tag("~"),
        )))),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_parser_test() {
        let input = "ciri";
        assert_eq!(url_friendliness_parser(input), Ok(("", vec!["ciri"])));

        let input = "ciri5.0";
        assert_eq!(
            url_friendliness_parser(input),
            Ok(("", vec!["ciri5", ".", "0"]))
        );

        let input = "ciri#as";
        assert_ne!(url_friendliness_parser(input), Ok(("", vec!["ciri"])));
    }
}

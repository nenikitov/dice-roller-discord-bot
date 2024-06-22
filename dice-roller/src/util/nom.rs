use nom;

macro_rules! re_export {
    ($path:ident) => {
        #[doc = concat!("Re-exports all `nom::", stringify!($path), "::complete` items.")]
        pub mod $path {
            #[allow(unused_imports)]
            pub use nom::$path::complete::*;
        }
    };
}

pub mod parser {
    pub mod number {
        use std::str::FromStr;

        #[allow(unused_imports)]
        pub use nom::number::complete::*;

        use super::super::*;

        #[allow(unused)]
        pub fn digit0<O: FromStr>(input: &str) -> IResult<&str, Option<O>> {
            combinator::map_res(
                combinator::recognize(sequence::preceded(
                    combinator::opt(parser::bytes::tag("-")),
                    parser::character::digit0,
                )),
                |s: &str| {
                    if !s.is_empty() {
                        s.parse().map(Some)
                    } else {
                        Ok(None)
                    }
                },
            )(input)
        }

        pub fn digit0_unwrap_or<O: FromStr + Clone>(
            default: O,
        ) -> impl FnMut(&str) -> IResult<&str, O> {
            move |input| {
                let (input, value) = self::digit0::<O>(input)?;
                Ok((input, value.unwrap_or(default.clone())))
            }
        }

        pub fn digit0_unwrap_or_default<O: FromStr + Default>(input: &str) -> IResult<&str, O> {
            let (input, value) = self::digit0::<O>(input)?;
            Ok((input, value.unwrap_or_default()))
        }

        #[allow(unused)]
        pub fn digit1<O: FromStr>(input: &str) -> nom::IResult<&str, O> {
            combinator::map_res(
                combinator::recognize(sequence::preceded(
                    combinator::opt(parser::bytes::tag("-")),
                    parser::character::digit1,
                )),
                str::parse,
            )(input)
        }
    }

    re_export!(bits);
    re_export!(bytes);
    re_export!(character);
}

pub use nom::{branch, combinator, sequence, IResult};

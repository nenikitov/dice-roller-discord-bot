use std::num::NonZeroI8;

use super::*;
use crate::util::nom::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Constant {
    value: NonZeroI8,
}

impl Constant {
    pub fn new(value: NonZeroI8) -> Self {
        Self { value }
    }
}

impl TokenValue for Constant {
    fn value(&self) -> i16 {
        i8::from(self.value) as i16
    }
}

impl Parse for Constant {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, value) = parser::number::digit1(input)?;

        Ok((input, Self::new(value)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_3_works() {
        assert_eq!(
            Constant::parse("3"),
            Ok((
                "",
                Constant {
                    value: NonZeroI8::new(3).unwrap()
                }
            ))
        );
    }

    #[test]
    fn parse_neg1_works() {
        assert_eq!(
            Constant::parse("-1"),
            Ok((
                "",
                Constant {
                    value: NonZeroI8::new(-1).unwrap()
                }
            ))
        );
    }

    // #[test]
    // fn parse_0_fails() {
    //     assert_eq!(
    //         Constant::parse("0"),
    //         Ok((
    //             "",
    //             Constant {
    //                 value: NonZeroI8::new(-1).unwrap()
    //             }
    //         ))
    //     );
    // }
}

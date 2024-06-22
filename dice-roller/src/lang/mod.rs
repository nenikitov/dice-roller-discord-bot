mod constant;
mod dice;

use constant::*;
use dice::*;

use crate::util::nom::*;

pub trait Parse
where
    Self: Sized,
{
    fn parse(input: &str) -> IResult<&str, Self>;
}

pub trait TokenValue {
    fn value(&self) -> i16;
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Dice(Dice),
    Constant(Constant),
}

impl TokenValue for Vec<Token> {
    fn value(&self) -> i16 {
        self.iter()
            .map(|t| match t {
                Token::Dice(t) => t.value(),
                Token::Constant(t) => t.value(),
            })
            .sum()
    }
}

impl Parse for Vec<Token> {
    fn parse(input: &str) -> IResult<&str, Self> {
        let tokens = input
            .split_whitespace()
            .map(|input| {
                branch::alt((
                    combinator::map(combinator::all_consuming(Dice::parse), Token::Dice),
                    combinator::map(combinator::all_consuming(Constant::parse), Token::Constant),
                ))(input)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let input = if let Some(last) = tokens.last() {
            last.0
        } else {
            input
        };

        let tokens = tokens.into_iter().map(|(_, token)| token).collect();

        Ok((input, tokens))
    }
}

#[cfg(test)]
mod test {
    use std::num::{NonZeroI8, NonZeroU8};

    use super::*;

    #[test]
    fn parse_tokens_work() {
        assert_eq!(
            Vec::<Token>::parse("2d20:adv d6 -3"),
            Ok((
                "",
                vec![
                    Token::Dice(Dice::new(
                        vec![
                            Die::new(NonZeroU8::new(20).unwrap()),
                            Die::new(NonZeroU8::new(20).unwrap())
                        ],
                        Some(Modifier::Advantage(NonZeroU8::new(1).unwrap())),
                    )),
                    Token::Dice(Dice::new(vec![Die::new(NonZeroU8::new(6).unwrap()),], None)),
                    Token::Constant(Constant::new(NonZeroI8::new(-3).unwrap()))
                ]
            ))
        );
    }
}

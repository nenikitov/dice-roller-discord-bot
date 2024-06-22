use std::num::NonZeroU8;

use rand::Rng;

use super::*;
use crate::util::nom::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Die {
    value: NonZeroU8,
    sides: NonZeroU8,
}

impl Die {
    pub fn new(sides: NonZeroU8) -> Self {
        let value = NonZeroU8::new(rand::thread_rng().gen_range(1..=sides.into()))
            .expect("Generated dice values are generated from 1..=sides");

        Self { value, sides }
    }
}

impl PartialOrd for Die {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Die {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl TokenValue for Die {
    fn value(&self) -> i16 {
        u8::from(self.value) as i16
    }
}

#[derive(Debug, PartialEq)]
pub enum Modifier {
    Advantage(NonZeroU8),
    Disadvantage(NonZeroU8),
}

impl Parse for Modifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        branch::alt((
            |input| {
                let (input, _) = parser::bytes::tag("adv")(input)?;
                let (input, leave) =
                    parser::number::digit0_unwrap_or(NonZeroU8::new(1).unwrap())(input)?;

                Ok((input, Modifier::Advantage(leave)))
            },
            |input| {
                let (input, _) = parser::bytes::tag("dis")(input)?;
                let (input, leave) =
                    parser::number::digit0_unwrap_or(NonZeroU8::new(1).unwrap())(input)?;

                Ok((input, Modifier::Disadvantage(leave)))
            },
        ))(input)
    }
}

#[derive(Debug)]
pub struct Dice {
    dice: Vec<Die>,
    modifier: Option<Modifier>,
}

impl Dice {
    pub fn new(dice: Vec<Die>, modifier: Option<Modifier>) -> Self {
        Self { dice, modifier }
    }
}

impl PartialEq for Dice {
    fn eq(&self, other: &Self) -> bool {
        self.dice.len() == other.dice.len() && self.modifier == other.modifier
    }
}

impl TokenValue for Dice {
    fn value(&self) -> i16 {
        let mut dice = self.dice.clone();
        dice.sort();

        let dice = match self.modifier {
            Some(Modifier::Advantage(take)) => dice
                .into_iter()
                .rev()
                .take(u8::from(take) as usize)
                .collect(),
            Some(Modifier::Disadvantage(take)) => {
                dice.into_iter().take(u8::from(take) as usize).collect()
            }
            None => dice,
        };

        dice.iter().map(Die::value).sum()
    }
}

impl Parse for Dice {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, count) = parser::number::digit0_unwrap_or(NonZeroU8::new(1).unwrap())(input)?;
        let (input, _) = parser::bytes::tag("d")(input)?;
        let (input, sides) = parser::number::digit1::<NonZeroU8>(input)?;

        let (input, modifier) = combinator::opt(parser::bytes::tag(":"))(input)?;
        let (input, modifier) = combinator::cond(modifier.is_some(), Modifier::parse)(input)?;

        let dice = (0..u8::from(count)).map(|_| Die::new(sides)).collect();

        Ok((input, Self { dice, modifier }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_d4_works() {
        assert_eq!(
            Dice::parse("d4"),
            Ok((
                "",
                Dice {
                    dice: vec![Die::new(NonZeroU8::new(4).unwrap())],
                    modifier: None
                }
            ))
        );
    }

    #[test]
    fn parse_2d20_works() {
        assert_eq!(
            Dice::parse("2d20"),
            Ok((
                "",
                Dice {
                    dice: vec![
                        Die::new(NonZeroU8::new(20).unwrap()),
                        Die::new(NonZeroU8::new(20).unwrap())
                    ],
                    modifier: None
                }
            ))
        );
    }

    #[test]
    fn parse_2d12adv_works() {
        assert_eq!(
            Dice::parse("2d20:adv"),
            Ok((
                "",
                Dice {
                    dice: vec![
                        Die::new(NonZeroU8::new(12).unwrap()),
                        Die::new(NonZeroU8::new(12).unwrap())
                    ],
                    modifier: Some(Modifier::Advantage(NonZeroU8::new(1).unwrap()))
                }
            ))
        );
    }

    #[test]
    fn parse_2d12dis_works() {
        assert_eq!(
            Dice::parse("2d20:dis"),
            Ok((
                "",
                Dice {
                    dice: vec![
                        Die::new(NonZeroU8::new(12).unwrap()),
                        Die::new(NonZeroU8::new(12).unwrap())
                    ],
                    modifier: Some(Modifier::Disadvantage(NonZeroU8::new(1).unwrap()))
                }
            ))
        );
    }

    #[test]
    fn parse_4d6adv3_works() {
        assert_eq!(
            Dice::parse("4d6:adv3"),
            Ok((
                "",
                Dice {
                    dice: vec![
                        Die::new(NonZeroU8::new(6).unwrap()),
                        Die::new(NonZeroU8::new(6).unwrap()),
                        Die::new(NonZeroU8::new(6).unwrap()),
                        Die::new(NonZeroU8::new(6).unwrap()),
                    ],
                    modifier: Some(Modifier::Advantage(NonZeroU8::new(3).unwrap()))
                }
            ))
        );
    }

    #[test]
    fn parse_4d6dis3_works() {
        assert_eq!(
            Dice::parse("4d6:dis3"),
            Ok((
                "",
                Dice {
                    dice: vec![
                        Die::new(NonZeroU8::new(6).unwrap()),
                        Die::new(NonZeroU8::new(6).unwrap()),
                        Die::new(NonZeroU8::new(6).unwrap()),
                        Die::new(NonZeroU8::new(6).unwrap()),
                    ],
                    modifier: Some(Modifier::Disadvantage(NonZeroU8::new(3).unwrap()))
                }
            ))
        );
    }
}

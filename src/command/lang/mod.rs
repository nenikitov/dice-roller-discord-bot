mod error;
mod error_token;
mod parse;

use std::num::{NonZeroI16, NonZeroU8};

use rand::Rng;

use error_token::*;

#[derive(Debug, PartialEq)]
pub enum Modifier {
    Advantage(NonZeroU8),
    Disadvantage(NonZeroU8),
}

impl Modifier {
    fn apply(&self, dice: &Vec<Die>) -> Vec<Die> {
        let mut sorted = dice.clone();
        sorted.sort();

        match self {
            Modifier::Advantage(take) => {
                sorted.into_iter().take(u8::from(*take) as usize).collect()
            }
            Modifier::Disadvantage(take) => {
                sorted.into_iter().take(u8::from(*take) as usize).collect()
            }
        }
    }
}

#[derive(Debug, Eq, PartialOrd, Clone)]
pub struct Die {
    sides: NonZeroU8,
    value: i16,
}

impl Die {
    pub fn new(sides: NonZeroU8) -> Self {
        Self {
            sides,
            value: rand::thread_rng().gen_range(1..=u8::from(sides) as i16),
        }
    }

    pub fn sides(&self) -> NonZeroU8 {
        self.sides
    }

    pub fn value(&self) -> i16 {
        self.value
    }
}

impl PartialEq for Die {
    fn eq(&self, other: &Self) -> bool {
        self.sides == other.sides
    }
}

impl Ord for Die {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Die(Vec<Die>, Option<Modifier>),
    Constant(NonZeroI16),
}

impl Token {
    fn value(&self) -> i16 {
        match self {
            Token::Die(dice, modifier) => if let Some(modifier) = modifier {
                modifier.apply(dice)
            } else {
                dice.clone()
            }
            .iter()
            .map(Die::value)
            .sum(),
            Token::Constant(value) => i16::from(*value),
        }
    }
}

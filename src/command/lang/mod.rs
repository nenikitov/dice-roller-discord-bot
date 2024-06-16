mod error;
mod error_token;
mod parse;

use std::{
    fmt::Display,
    num::{NonZeroI16, NonZeroU8},
};

use rand::Rng;

use error_token::*;

use super::table::{Alignment, Table, TableRow};

#[derive(Debug, PartialEq)]
pub enum Modifier {
    Advantage(NonZeroU8),
    Disadvantage(NonZeroU8),
}

impl Modifier {
    fn apply(&self, dice: &[Die]) -> Vec<Die> {
        let mut sorted = dice.to_vec();
        sorted.sort();

        match self {
            Modifier::Advantage(take) => sorted
                .into_iter()
                .rev()
                .take(u8::from(*take) as usize)
                .collect(),
            Modifier::Disadvantage(take) => {
                sorted.into_iter().take(u8::from(*take) as usize).collect()
            }
        }
    }
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Modifier::Advantage(take) => format!("Advantage (take {take})"),
                Modifier::Disadvantage(take) => format!("Disadvantage (take {take})"),
            }
        )
    }
}

#[derive(Debug, Eq, Clone)]
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

impl PartialOrd for Die {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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

impl Into<Table> for &Token {
    fn into(self) -> Table {
        match self {
            Token::Die(dice, modifier) => {
                let mut table = Table::new(if let Some(modifier) = modifier {
                    vec![TableRow::FullWidth(
                        format!("// {modifier}"),
                        Alignment::Center,
                    )]
                } else {
                    vec![]
                });
                table.append_rows(
                    &dice
                        .iter()
                        .map(|d| {
                            TableRow::Columns(vec![
                                (d.value().to_string(), Alignment::Right),
                                (format!("(d{})", d.sides()), Alignment::Left),
                            ])
                        })
                        .collect::<Vec<_>>(),
                );
                table
            }
            Token::Constant(value) => Table::new(vec![TableRow::Columns(vec![(
                value.to_string(),
                Alignment::Right,
            )])]),
        }
    }
}

pub struct Tokens(Vec<Token>);

impl Into<Table> for Tokens {
    fn into(self) -> Table {
        let mut result = Table::new(vec![]);

        for (i, token) in self.0.iter().enumerate() {
            if i > 0 {
                result.append_row(&TableRow::Separator('-'));
            }
            result.append_table(&token.into());
        }
        result.append_row(&TableRow::Separator('='));

        result.append_row(&TableRow::Columns(vec![(
            self.0.iter().map(Token::value).sum::<i16>().to_string(),
            Alignment::Right,
        )]));

        result
    }
}

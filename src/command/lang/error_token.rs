use std::{
    fmt::Display,
    num::{NonZeroU8, ParseIntError},
};

use super::error::ParseError;

pub type ParseNumberError = ParseError<ParseIntError>;

#[derive(Debug, PartialEq)]
pub enum ModifierCountError {
    Parse(ParseNumberError),
    MoreThanDice { take: NonZeroU8, count: NonZeroU8 },
}

impl Display for ModifierCountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ModifierCountError::Parse(e) => format!("{e}"),
                ModifierCountError::MoreThanDice { take, count } =>
                    format!("dice to leave ({take}) cannot be more than dice thrown ({count})"),
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum ModifierErrorKind {
    Invalid,
    Advantage(ModifierCountError),
    Disadvantage(ModifierCountError),
}

impl Display for ModifierErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ModifierErrorKind::Invalid => "Invalid modifier".to_string(),
                ModifierErrorKind::Advantage(e) => format!("Advantage count: {e}"),
                ModifierErrorKind::Disadvantage(e) => format!("Disadvantage count: {e}"),
            }
        )
    }
}

pub type ParseModifierError = ParseError<ModifierErrorKind>;

#[derive(Debug, PartialEq)]
pub enum DieErrorKind {
    Count(ParseIntError),
    Sides(ParseIntError),
    Modifier(ParseModifierError),
}

impl Display for DieErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DieErrorKind::Count(e) => format!("Count: {e}"),
                DieErrorKind::Sides(e) => format!("Sides: {e}"),
                DieErrorKind::Modifier(e) => format!("Modifier: {e}"),
            }
        )
    }
}

pub type ParseDieError = ParseError<DieErrorKind>;

pub type ParseConstantError = ParseNumberError;

#[derive(Debug, PartialEq)]
pub enum TokenErrorKind {
    Invalid,
    Die(ParseDieError),
    Constant(ParseConstantError),
}

impl Display for TokenErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenErrorKind::Invalid => "Invalid token".to_string(),
                TokenErrorKind::Die(e) => format!("Die: {e}"),
                TokenErrorKind::Constant(e) => format!("Constant: {e}"),
            }
        )
    }
}

pub type ParseTokenError = ParseError<TokenErrorKind>;

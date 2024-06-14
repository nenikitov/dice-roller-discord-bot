use std::{
    num::{NonZeroU8, ParseIntError},
    str::FromStr,
};

use itertools::Itertools;
use regex::Regex;

use crate::command::*;

#[derive(Debug, Clone)]
struct Die {
    sides: NonZeroU8,
}

impl Die {
    fn roll(&self) -> u8 {
        rand::random::<u8>() % self.sides + 1
    }
}

#[derive(Debug)]
struct Dice(Vec<Die>);

#[derive(Debug)]
enum RollConversionError {
    DieCount(ParseIntError),
    DieSides(ParseIntError),
    Format,
}

impl std::error::Error for RollConversionError {}
impl std::fmt::Display for RollConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}. {}",
            match self {
                RollConversionError::DieCount(e) => format!("Invalid number of dice ({e})"),
                RollConversionError::DieSides(e) => format!("Invalid die sides ({e})"),
                RollConversionError::Format => format!("Could not parse die"),
            },
            "Format is `d2` or `2d6`"
        )?;

        Ok(())
    }
}

impl FromStr for Dice {
    type Err = RollConversionError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d*)d(\d+)$").unwrap();

        let dice = s
            .split_whitespace()
            .map(|s| {
                if let Some(captures) = re.captures(s) {
                    let count = captures.get(1).map_or(
                        Ok(NonZeroU8::new(1).expect("Value is hardcoded")),
                        |m| {
                            if m.as_str().is_empty() {
                                Ok(NonZeroU8::new(1).expect("Value is hardcoded"))
                            } else {
                                m.as_str()
                                    .parse::<NonZeroU8>()
                                    .map_err(|e| RollConversionError::DieCount(e))
                            }
                        },
                    )?;

                    let sides = captures
                        .get(2)
                        .ok_or(RollConversionError::Format)
                        .and_then(|m| {
                            m.as_str()
                                .parse::<NonZeroU8>()
                                .map_err(|e| RollConversionError::DieSides(e))
                        })?;

                    Ok(vec![Die { sides }; u8::from(count) as usize])
                } else {
                    Err(RollConversionError::Format)
                }
            })
            .collect::<std::result::Result<Vec<_>, _>>()?;

        let dice = dice.into_iter().flatten().collect();

        Ok(Self(dice))
    }
}

/// Roll some dice.
#[poise::command(slash_command, prefix_command)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "A dice to roll, space separated, might need to be quoted"] dice: Dice,
) -> Result {
    let rolls: Vec<_> = dice.0.iter().map(Die::roll).collect();

    let sum: u16 = rolls.iter().map(|e| *e as u16).sum();
    let sum_len = sum.to_string().len();

    let table = std::iter::zip(&dice.0, &rolls)
        .map(|(die, roll)| format!("{roll: >sum_len$} (d{})", die.sides))
        .join("\n");

    ctx.say(format!(
        "```rust\n{table}\n{}\n{sum}```",
        "-".repeat(sum_len)
    ))
    .await?;

    Ok(())
}

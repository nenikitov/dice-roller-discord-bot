// use lang::Tokens;
// use table::Table;

use crate::command::*;

/// Roll some dice.
///
/// Valid tokens are:
/// - `1` - constant, bonus
/// - `d20` - 20-sided die
/// - `4d6` - 4 6-sided dice
/// - `2d20:adv` - 2 20-sided dice, pick highest
/// - `4d6:dis3` - 4 6-sided dice, pick 3 lowest
#[poise::command(slash_command, prefix_command)]
pub async fn roll(
    ctx: Context<'_>,
    //    #[description = "What to roll, space separated"] tokens: Tokens,
) -> Result {
    //     let table: Table = tokens.into();
    //
    //     ctx.say(format!(
    //         "\
    // ```rust
    // {table}
    // ```
    // "
    //     ))
    //     .await?;
    ctx.say("Rolled").await?;

    Ok(())
}

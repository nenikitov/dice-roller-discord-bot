use crate::command::*;

/// Show all available commands and how to use them.
#[poise::command(prefix_command, track_edits)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result {
    let config = poise::builtins::HelpConfiguration::default();
    poise::builtins::help(ctx, command.as_deref(), config).await?;

    Ok(())
}

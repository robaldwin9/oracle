use crate::{Context, Error};
use super::pick_random_response;

#[poise::command(slash_command, rename = "divine")]
pub async fn decide(
    ctx: Context<'_>,
    #[description = "Options for the oracle to choose between"] 
    options: String,
) ->  Result<(), Error> {
     let choices: Vec<String> = options
        .split(',')
        .map(str::trim)
        .filter(|choice| !choice.is_empty())
        .map(str::to_owned)
        .collect();

    if choices.len() < 2 {
        ctx.say("Provide at least two choices, separated by commas.")
            .await?;
        return Ok(());
    }

    let choice = pick_random_response(&choices)?;

    ctx.say(format!("🔮 The Oracle chooses: {}", choice))
        .await?;

    Ok(())
}


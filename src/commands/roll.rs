use crate::{Context, Error};

#[poise::command(slash_command, rename="cast")]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "oracle will roll and n sides dice to decide your fate"] 
    #[min = 2]
    #[max = 1000]
    sides: Option<i64>,
) ->  Result<(), Error> {
    let sides = sides.unwrap_or(6);
    let num = rand::random_range(1..=sides);
    let response = format!("🦴 The bones reveal: **{num}**");
    ctx.say(response).await?;
    Ok(())
}

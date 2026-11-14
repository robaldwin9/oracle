use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::Mentionable;
use super::pick_random_response;
 
#[poise::command(slash_command, rename = "bless")]
pub async fn compliment(
    ctx: Context<'_>,
    #[description = "Let the oracle give you, or another a compliment"] user: Option<serenity::User>,
) ->  Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let answer = pick_random_response(&ctx.data().compliments)?;
    let response = format!("✨ {} {}", u.mention(), answer);
    ctx.send(
        poise::CreateReply::default()
            .content(response)
            .allowed_mentions(serenity::CreateAllowedMentions::new().users([u.id])),
    )
    .await?;
    Ok(())
}

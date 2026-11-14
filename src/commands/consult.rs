use crate::{Context, Error};
use super::pick_random_response;

#[poise::command(slash_command)]
pub async fn consult(
    ctx: Context<'_>,
    #[description = "Oracle will answer your questions"] 
    question: String,
) ->  Result<(), Error> {
    let answer = pick_random_response(&ctx.data().answers)?;
    let response = format!("🔮 **Question:** {question}\n**The Oracle says:** {answer}");
    ctx.say(response).await?;
    Ok(())
}


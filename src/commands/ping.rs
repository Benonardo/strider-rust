use poise::CreateReply;
use serenity::builder::CreateEmbed;

use super::Context;
use crate::Error;

/// Show the ping of the bot
#[poise::command(slash_command)]
pub(crate) async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let ping = ctx.ping().await.as_millis();
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .description(format!("Avg. Ping: {ping}"))
                .color(if ping > 100 { (255, 0, 0) } else { (0, 255, 0) }),
        ).ephemeral(true)
    )
    .await?;

    Ok(())
}

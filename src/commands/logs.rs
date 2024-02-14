use serenity::all::Message;

use crate::Error;

use super::Context;

#[poise::command(context_menu_command = "Upload Logs")]
pub(crate) async fn logs(ctx: Context<'_>, message: Message) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    crate::log_upload::check_for_logs(ctx.serenity_context(), &message).await?;

    ctx.say("Uploaded, see below").await?;

    Ok(())
}

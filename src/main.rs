mod commands;
mod constants;
mod log_upload;

use std::collections::HashMap;

use anyhow::Context;
use poise::{Framework, FrameworkOptions};
use serenity::{all::GatewayIntents, client::{ClientBuilder, FullEvent}};

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Default)]
pub(crate) struct Data {
    pub(crate) tags: HashMap<String, commands::tag::EmbedData>
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("DISCORD_TOKEN").with_context(|| "couldn't get DISCORD_TOKEN")?;
    let intents = GatewayIntents::all(); // TODO change

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![commands::version(), commands::ping(), commands::logs(), commands::tag()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data::default())
            })
        })
        .build();

    let mut client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;
    client.start().await?;

    Ok(())
}

async fn event_handler(
    ctx: &serenity::all::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Message {
            new_message
        } => {
            log_upload::check_for_logs(ctx, new_message).await
        }
        _ => Ok(())
    }
}
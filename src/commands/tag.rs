use anyhow::Context;
use poise::CreateReply;
use serde::Deserialize;
use serenity::builder::CreateEmbed;

use crate::{Data, Error};

#[derive(Deserialize)]
pub(crate) struct EmbedData {
    title: Option<String>,
    description: Option<String>,
    image: Option<String>,
    thumbnail: Option<String>,
    #[serde(default = "EmbedData::get_default_color")]
    color: u32,
    #[serde(default)]
    fields: Vec<EmbedField>,
}

impl EmbedData {
    const DEFAULT_COLOR: u32 = 0x202225;

    const fn get_default_color() -> u32 {
        Self::DEFAULT_COLOR
    }
}

#[derive(Deserialize)]
struct EmbedField {
    #[serde(default)]
    inline: bool,
    #[serde(default = "EmbedField::get_zero_width_string")]
    name: String,
    #[serde(default = "EmbedField::get_zero_width_string")]
    value: String,
}

impl EmbedField {
    const ZERO_WIDTH_CHAR: char = '\u{200E}';

    fn get_zero_width_string() -> String {
        Self::ZERO_WIDTH_CHAR.to_string()
    }
}

/// Show a tag
#[poise::command(slash_command)]
pub(crate) async fn tag(
    ctx: super::Context<'_>,
    #[description = "The tag to show"] tag: String,
) -> Result<(), Error> {
    let embed_data = ctx.data().tags.get(&tag);

    let Some(embed_data) = embed_data else {
        ctx.say(format!("Error 69: tag {} not found", tag)).await?;
        return Ok(());
    };

    let mut embed = CreateEmbed::default().color(embed_data.color).fields(
        embed_data
            .fields
            .iter()
            .map(|field| (&field.name, &field.value, field.inline)),
    );
    if let Some(title) = &embed_data.title {
        embed = embed.title(title);
    }
    if let Some(description) = &embed_data.description {
        embed = embed.description(description);
    }
    if let Some(image) = &embed_data.image {
        embed = embed.image(image);
    }
    if let Some(thumbnail) = &embed_data.thumbnail {
        embed = embed.thumbnail(thumbnail);
    }

    ctx.send(CreateReply::default().embed(CreateEmbed::default()))
        .await?;

    Ok(())
}

// Reload the bot commands
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub(crate) async fn reload(ctx: super::Context<'_>) -> Result<(), Error> {
    ctx.send(
        CreateReply::default()
            .content("Tags synced")
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

async fn update_tags(data: &Data) -> Result<(), Error> {
    if let Ok(dir) = tokio::fs::read_dir(std::env::var("TAG_FOLDER").with_context(|| "couldn't get TAG_FOLDER")?).await {
        
    }

    Ok(())
}

use std::{collections::HashMap, io::ErrorKind};

use anyhow::Context;
use async_compression::futures::bufread::GzipDecoder;
use serde::{Deserialize, Serialize};
use serenity::{
    all::{Attachment, Message},
    builder::{CreateActionRow, CreateButton, EditMessage},
    futures::{AsyncReadExt, TryStreamExt},
};

use crate::Error;

#[derive(Deserialize, Clone)]
struct LogData {
    success: bool,
    url: Option<String>,
    _error: Option<String>,
}

#[derive(Serialize)]
struct LogUpload {
    content: String,
}

pub(crate) async fn check_for_logs(
    ctx: &serenity::all::Context,
    message: &Message,
) -> Result<(), Error> {
    let file_extensions = get_log_file_extensions()?;

    let attachments: Vec<_> = message
        .attachments
        .iter()
        .filter(|attachment| is_valid_log(attachment, &file_extensions))
        .collect();

    if attachments.is_empty() {
        return Ok(());
    }

    let mut reply = message.reply(ctx, "Logs detected, uploading...").await?;
    let logs = upload_log_files(&attachments).await?;

    let edit = if logs.is_empty() {
        EditMessage::default().content("Failed to upload!")
    } else {
        EditMessage::default()
            .content(format!("Uploaded {} logs", logs.len()))
            .components(vec![CreateActionRow::Buttons(
                logs.iter()
                    .filter(|(_, log)| log.url.is_some())
                    .map(|(name, log)| CreateButton::new_link(log.url.clone().unwrap()).label(name))
                    .collect(),
            )])
    };

    reply.edit(ctx, edit).await?;

    Ok(())
}

fn is_valid_log<T: AsRef<str>>(attachment: &Attachment, allowed_extensions: &[T]) -> bool {
    attachment.size < 1_000_000
        && (allowed_extensions
            .iter()
            .any(|extension| attachment.filename.ends_with(extension.as_ref())))
}

fn get_log_file_extensions() -> Result<Vec<String>, Error> {
    Ok(std::env::var("LOG_EXTENSIONS")
        .with_context(|| "couldn't get LOG_EXTENSIONS")?
        .trim()
        .split(',')
        .map(|str| str.to_owned())
        .collect())
}

async fn upload_log_files(attachments: &[&Attachment]) -> Result<HashMap<String, LogData>, Error> {
    let mut responses = HashMap::new();

    for attachment in attachments {
        let log = if attachment.filename.ends_with(".gz") {
            let mut reader = GzipDecoder::new(
                reqwest::get(&attachment.url)
                    .await?
                    .bytes_stream()
                    .map_err(|e| std::io::Error::new(ErrorKind::Other, e))
                    .into_async_read(),
            );

            let mut string = String::new();
            reader.read_to_string(&mut string).await?;
            string
        } else {
            String::from_utf8(attachment.download().await?)?
        };

        responses.insert(attachment.filename.clone(), upload(log).await?);
    }

    Ok(responses
        .iter()
        .filter(|(_, response)| response.success)
        .map(|(file, log)| (file.clone(), log.clone()))
        .collect())
}

async fn upload(log: String) -> Result<LogData, Error> {
    let client = reqwest::Client::new();

    Ok(client
        .post(format!("{}/1/log", crate::constants::MCLOGS_BASE_URL))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(serde_urlencoded::to_string(LogUpload { content: log })?)
        .send()
        .await?
        .json()
        .await?)
}

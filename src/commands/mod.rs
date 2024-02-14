mod version;
mod ping;
mod logs;
pub(crate) mod tag;

pub(self) use super::Data;

type Context<'a> = poise::Context<'a, Data, crate::Error>;

pub(crate) use version::version;
pub(crate) use ping::ping;
pub(crate) use logs::logs;
pub(crate) use tag::tag;
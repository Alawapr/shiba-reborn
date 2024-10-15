use std::vec;

use poise::{serenity_prelude::CreateEmbed, CreateReply};

use super::super::{EMBED_COLOR, INVITE_LINK, SHIBA_MAIN_IMAGE_URL, SUPPORT_SERVER};
use crate::Error;

/// Invite Shiba to your server
#[poise::command(slash_command, broadcast_typing)]
pub async fn invite(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    ctx.defer().await?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .color(EMBED_COLOR)
                    .field(
                        "Invite Shiba",
                        format!("Click [this link]({INVITE_LINK}) to invite Shiba to your server!\nHaving issues? Click [here]({SUPPORT_SERVER}) for help!"),
                        false,
                    )
                    .thumbnail(SHIBA_MAIN_IMAGE_URL),
        ),
    )
    .await?;

    Ok(())
}

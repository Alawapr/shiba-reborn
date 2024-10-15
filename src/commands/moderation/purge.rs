use crate::{
    commands::{EMBED_COLOR, ERROR_EMBED_COLOR, ERROR_EMOJI},
    Error,
};
use poise::serenity_prelude::Permissions;
use poise::{
    serenity_prelude::{CreateEmbed, CreateEmbedAuthor, GetMessages},
    CreateReply,
};

/// Purge a maximum of 100 messages or an specified amount. Can't delete messages older than 2 weeks.
#[poise::command(
    slash_command,
    broadcast_typing,
    required_permissions = "MANAGE_MESSAGES"
)]
pub async fn purge(
    ctx: poise::Context<'_, (), Error>,
    #[description = "Amount of messages to purge. Can't be more than 99"] amount: Option<u8>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let perms = ctx
        .author_member()
        .await
        .expect("Couldn't get author member")
        .permissions
        .expect("Couldn't get permissions")
        .contains(Permissions::MANAGE_MESSAGES);
    if !perms {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Error")
                    .description(format!(
                        "{ERROR_EMOJI} You don't have permission to use this command.\nPermission needed: `MANAGE_MESSAGES`.",
                    ))
                    .color(ERROR_EMBED_COLOR),
            ),
        )
        .await?;
        return Ok(());
    }

    let http = ctx.http();

    if let Some(amount) = amount {
        // Need to add 1 to the amount to account for our own message
        let amount = amount + 1;
        if amount > 100 {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::new()
                        .author(
                            CreateEmbedAuthor::new("Error")
                                .icon_url("https://i.imgur.com/tTvxuiG.png"),
                        )
                        .description("Can't purge more than 100 messages at a time.")
                        .color(ERROR_EMBED_COLOR),
                ),
            )
            .await?;
        }
        let builder = GetMessages::new().limit(amount);
        let mut messages = ctx.channel_id().messages(http, builder).await?;
        // Remove the first message since it's our own
        messages.remove(0);
        let message_ids = messages
            .iter()
            .map(|message| message.id)
            .collect::<Vec<_>>();

        ctx.channel_id().delete_messages(http, message_ids).await?;

        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("Messages Purged")
                    .description(format!("Purged {} messages.", amount - 1))
                    .color(EMBED_COLOR),
            ),
        )
        .await?;
    } else {
        let builder = GetMessages::new().limit(100);
        let messages = ctx.channel_id().messages(http, builder).await?;
        let message_ids = messages
            .iter()
            .map(|message| message.id)
            .collect::<Vec<_>>();

        ctx.channel_id().delete_messages(http, message_ids).await?;

        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("Messages Purged")
                    .description("Purged 100 messages.")
                    .color(EMBED_COLOR),
            ),
        )
        .await?;
    }

    Ok(())
}

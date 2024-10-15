use std::vec;

use poise::{serenity_prelude::CreateEmbed, CreateReply};
use rayon::prelude::*;

use super::super::EMBED_COLOR;
use crate::Error;

/// Display info about a user
#[poise::command(slash_command, broadcast_typing)]
pub async fn userinfo(
    ctx: poise::Context<'_, (), Error>,
    #[description = "The user to get info from."] user: Option<poise::serenity_prelude::User>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let mut user = user;
    if user.is_none() {
        user = Some(ctx.author().clone());
    }

    // User will never be `None` since we've already assured it isn't.
    let user = user.expect("Unreachable");

    let in_a_guild = ctx.guild_id().is_some();

    let banner = ctx.http().get_user(user.id).await?.banner_url();

    let embed = CreateEmbed::default()
        .title(format!("Information about {}:", user.name))
        .field("User ID", format!("`{}`", user.id.get()), true)
        .field("Username", format!("`{}`", user.name), true)
        .field(
            "Display Name",
            format!(
                "`{}`",
                user.global_name
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string())
            ),
            true,
        )
        .field(
            "Created at",
            format!("<t:{}:R>", user.created_at().unix_timestamp()),
            true,
        )
        .field(
            "Is it a bot?",
            format!("`{}`", if user.bot { "Yes" } else { "No" }),
            true,
        )
        .thumbnail(
            user.avatar_url()
                .unwrap_or_else(|| "https://cdn.discordapp.com/embed/avatars/0.png".to_string()),
        )
        .color(EMBED_COLOR);

    let reply = if let Some(banner) = banner {
        CreateReply::default().embed(embed.image(banner))
    } else if in_a_guild {
        let member = ctx
            .guild_id()
            .expect("Couldn't get guild's ID")
            .member(ctx.http(), user.id)
            .await?;
        let roles = member
            .roles
            .par_iter()
            .map(poise::serenity_prelude::Mentionable::mention)
            .map(|m| ToString::to_string(&m))
            .collect::<Vec<_>>()
            .join(", ");

        CreateReply::default().embed(embed.field("Roles", roles, true))
    } else {
        CreateReply::default().embed(embed)
    };

    ctx.send(reply).await?;

    Ok(())
}

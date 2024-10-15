use std::vec;

use poise::{
    serenity_prelude::{CreateEmbed, CreateEmbedAuthor},
    CreateReply,
};
use rayon::prelude::*;

use super::super::EMBED_COLOR;
use crate::{commands::ERROR_EMBED_COLOR, Error};

/// Display info about the server
#[poise::command(slash_command, broadcast_typing)]
pub async fn serverinfo(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    ctx.defer().await?;

    if ctx.guild_id().is_none() {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    // .title(format!("{} Error", SHIBA_ERROR_EMOJI))
                    .author(
                        CreateEmbedAuthor::new("Error").icon_url("https://i.imgur.com/tTvxuiG.png"),
                    )
                    .description("This command can only be used in a server!")
                    .color(ERROR_EMBED_COLOR),
            ),
        )
        .await?;
        return Ok(());
    }

    // .unwrap() is safe to use since we've made sure that we're in a guild.
    let g_id = ctx.guild_id().expect("Unreachable");
    let bots = ctx
        .serenity_context()
        .http
        .get_guild_members(g_id, None, None)
        .await?
        .par_iter()
        .filter(|m| m.user.bot)
        .count();

    let guild = ctx.serenity_context().http.get_guild(g_id).await?;

    let humans = ctx
        .serenity_context()
        .http
        .get_guild_members(g_id, None, None)
        .await?
        .len()
        - bots;

    let icon = guild.icon_url().map_or_else(
        || "https://cdn.discordapp.com/embed/avatars/0.png".to_string(),
        |url| url,
    );

    let banner = guild.banner_url();

    let region = ctx.serenity_context().http.get_guild_regions(g_id).await?;
    // It's safe to use .expect() here since `region` will never be empty. I hope.
    let region = region.first().expect("Unreachable");

    let embed = CreateEmbed::default()
        .title(format!("Information about {}:", guild.name))
        .field("Server Name", format!("`{}`", guild.name), true)
        .field("Server ID", format!("`{}`", guild.id.get()), true)
        .field("Server Owner", format!("`{}`", guild.owner_id.get()), true)
        .field("Member Count", format!("`{}`", humans + bots), true)
        .field("Bot Count", format!("`{bots}`"), true)
        .field("Human Count", format!("`{humans}`"), true)
        .field("Server Region", format!("`{}`", region.name), true)
        .field(
            "Created at ",
            format!("`{}`", g_id.created_at().format("%Y-%m-%d %H:%M:%S")),
            true,
        )
        .thumbnail(icon)
        .color(EMBED_COLOR);

    let reply = if let Some(banner) = banner {
        CreateReply::default().embed(embed.image(banner))
    } else {
        CreateReply::default().embed(embed)
    };

    ctx.send(reply).await?;

    Ok(())
}

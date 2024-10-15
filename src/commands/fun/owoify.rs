use std::vec;

use crate::Error;

use super::super::EMBED_COLOR;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
use poise::CreateReply;

/// Owoify
#[poise::command(slash_command, broadcast_typing)]
pub async fn owoify(
    ctx: poise::Context<'_, (), Error>,
    #[description = "Your text."] text: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let transformed_text: String = text
        .chars()
        .map(|c| match c {
            'L' | 'R' => 'W',
            'l' | 'r' => 'w',
            'O' => 'U',
            _ => c,
        })
        .collect();

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("Owoifier 2900:")
                .description(transformed_text)
                .color(EMBED_COLOR)
                .footer(CreateEmbedFooter::new(format!(
                    "Translated from \"{text}\""
                ))),
        ),
    )
    .await?;

    Ok(())
}

use std::vec;

use crate::commands::WARNING_EMOJI;
use crate::Error;

use super::super::EMBED_COLOR;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
use poise::CreateReply;
use rand::Rng;

#[derive(serde::Deserialize)]
struct MojangResponse {
    name: String,
}

#[inline]
fn generate_random_text(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut text = String::new();
    for _ in 0..length {
        // choose either a letter or a digit
        let c = if rng.gen::<bool>() {
            (b'a' + rng.gen_range(0..26)) as char
        } else {
            (b'0' + rng.gen_range(0..10)) as char
        };
        text.push(c);
    }

    text
}

/// Grab someone's Minecraft skin
#[poise::command(slash_command, broadcast_typing)]
pub async fn skin(
    ctx: poise::Context<'_, (), Error>,
    #[description = "Minecraft username/UUID."] username: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let mut username = username.replace('-', "");
    let rndm_string = generate_random_text(5);

    if username.len() == 32 {
        let response: MojangResponse =
            reqwest::get(format!("https://mc-heads.net/minecraft/profile/{username}"))
                .await?
                .json()
                .await?;

        username = response.name;
    }

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title(format!("Skin of {username}"))
                .description(format!(
                    "Download it [here](https://mc-heads.net/download/{username})."
                ))
                .image(format!(
                    "https://mc-heads.net/body/{username}/{rndm_string}"
                ))
                .color(EMBED_COLOR)
                .footer(
                    CreateEmbedFooter::new("Notice: The skin may take some time to be up-to-date.")
                        .icon_url(WARNING_EMOJI),
                )
                .thumbnail(format!(
                    "https://mc-heads.net/combo/{username}/{rndm_string}"
                )),
        ),
    )
    .await?;

    Ok(())
}

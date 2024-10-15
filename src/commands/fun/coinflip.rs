use std::vec;

use crate::Error;

use super::super::EMBED_COLOR;
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;
use rand::Rng;

/// Flip a coin!
#[poise::command(slash_command, broadcast_typing)]
pub async fn coinflip(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    ctx.defer().await?;

    let mut rng = <rand::rngs::SmallRng as rand::SeedableRng>::from_entropy();
    let random_number: f64 = rng.gen();

    let img_url;
    let ans = if random_number < 0.48 {
        img_url = "https://i.imgur.com/BpuCUPH.png";
        "landed on heads"
    } else if random_number < 0.96 {
        img_url = "https://i.imgur.com/tpnNRIR.png";
        "landed on tails"
    } else {
        img_url = "https://i.imgur.com/RKcIpar.png";
        "landed on the side"
    };

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("Coinflip")
                .field(format!("The coin {ans}!"), "", false)
                .color(EMBED_COLOR)
                .thumbnail(img_url),
        ),
    )
    .await?;

    Ok(())
}

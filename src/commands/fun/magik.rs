use std::vec;

use crate::Error;

use super::super::EMBED_COLOR;
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;
use rand::seq::SliceRandom;

const MAGIK_IMAGE: &str = "https://i.imgur.com/pZlxaVI.png";

const RESPONSES: [&str; 18] = [
    "As I see it, yes.",
    "Without a doubt.",
    "Yes.",
    "It is certain.",
    "It is decidedly so.",
    "Most likely.",
    "Outlook good.",
    "Signs point to yes.",
    "Don't count on it.",
    "My reply is no.",
    "My sources say no.",
    "Outlook not so good.",
    "Ask again later.",
    "Better not tell you now.",
    "Cannot predict now.",
    "Concentrate and ask again.",
    "Reply hazy, try again.",
    "Very doubtful.",
];

/// Ask the 8ball a question
#[poise::command(slash_command, broadcast_typing)]
pub async fn magik(
    ctx: poise::Context<'_, (), Error>,
    #[description = "Your question."] question: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    // It's safe to use .expect() since `RESPONSES` will never be empty.
    // .choose() will only return `None` if it is empty.
    let ans = RESPONSES
        .choose(&mut rand::thread_rng())
        .expect("Unreachable");

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("Magik 8Ball says:")
                .description(format!("Q: {question}\nA: {ans}"))
                .color(EMBED_COLOR)
                .thumbnail(MAGIK_IMAGE),
        ),
    )
    .await?;

    Ok(())
}

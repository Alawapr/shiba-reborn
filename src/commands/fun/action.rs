use std::vec;

use crate::{database, Error, ACTION_COUNTS};

use super::super::{EMBED_COLOR, ERROR_EMBED_COLOR, ERROR_EMOJI};
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter, Mentionable};
use poise::CreateReply;
use rand::Rng;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://shiba-gifs.pages.dev";

const ACTIONS: [&str; 11] = [
    "bite", "cuddle", "highfive", "hug", "kiss", "lick", "pat", "poke", "punch", "shoot", "slap",
];

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Counts {
    bite: usize,
    cuddle: usize,
    highfive: usize,
    hug: usize,
    kiss: usize,
    lick: usize,
    pat: usize,
    poke: usize,
    punch: usize,
    shoot: usize,
    slap: usize,
}

/// Roleplay an action to someone
#[poise::command(slash_command, broadcast_typing)]
pub async fn action(
    ctx: poise::Context<'_, (), Error>,
    #[description = "The action you want to perform."]
    #[autocomplete = "actions_autocomplete"]
    action: String,
    #[description = "The user you'd like to perform this action on."]
    user: poise::serenity_prelude::User,
) -> Result<(), Error> {
    // Can't use defer since it makes poise edit the message with the content
    // instead of sending the message directly. This makes it not ping when it
    // mentions users.
    // ctx.defer().await?;

    // It IS used
    #[allow(unused)]
    let mut title = String::new();
    #[allow(unused)]
    let mut footer = String::new();
    #[allow(unused)]
    let mut gif_count = 0;
    let mut new_count = 0;
    if ACTIONS.contains(&action.as_str()) {
        let action_count = database::get_action_count(&action, ctx.author().id, user.id)?;
        new_count = action_count + 1;
        database::update_action_count(&action, ctx.author().id, user.id, new_count)?;
    }

    if unsafe { ACTION_COUNTS.is_none() } {
        return Err("Action counts not loaded.".into());
    }

    match action.as_str() {
        "bite" => {
            title = format!("{} bites {}!", ctx.author().name, user.name);
            footer = format!(
                "{} has bitten {} {} times.",
                ctx.author().name,
                user.name,
                new_count
            );
            gif_count = get_counts().bite;
        }

        "cuddle" => {
            title = format!("{} {}s {}!", ctx.author().name, action, user.name);
            footer = format!(
                "{} has {}d {} {} times.",
                ctx.author().name,
                action,
                user.name,
                new_count
            );
            gif_count = get_counts().cuddle;
        }

        "highfive" => {
            title = format!("{} {}s {}!", ctx.author().name, action, user.name);
            footer = format!(
                "{} has {}d {} {} times.",
                ctx.author().name,
                action,
                user.name,
                new_count
            );
            gif_count = get_counts().highfive;
        }

        "hug" => {
            title = format!("{} hugs {}!", ctx.author().name, user.name);
            footer = format!(
                "{} has hugged {} {} times.",
                ctx.author().name,
                user.name,
                new_count
            );
            gif_count = get_counts().hug;
        }

        "kiss" => {
            title = format!("{} kisses {}!", ctx.author().name, user.name);
            footer = format!(
                "{} has kissed {} {} times.",
                ctx.author().name,
                user.name,
                new_count
            );
            gif_count = get_counts().kiss;
        }

        "lick" => {
            title = format!("{} {}s {}!", ctx.author().name, action, user.name);
            footer = format!(
                "{} has {}ed {} {} times.",
                ctx.author().name,
                action,
                user.name,
                new_count
            );
            gif_count = get_counts().lick;
        }

        "pat" => {
            title = format!("{} {}s {}!", ctx.author().name, action, user.name);
            footer = format!(
                "{} has {}ted {} {} times.",
                ctx.author().name,
                action,
                user.name,
                new_count
            );
            gif_count = get_counts().pat;
        }

        "poke" => {
            title = format!("{} {}s {}!", ctx.author().name, action, user.name);
            footer = format!(
                "{} has {}ed {} {} times.",
                ctx.author().name,
                action,
                user.name,
                new_count
            );
            gif_count = get_counts().poke;
        }

        "punch" => {
            title = format!("{} punches {}!", ctx.author().name, user.name);
            footer = format!(
                "{} has punched {} {} times.",
                ctx.author().name,
                user.name,
                new_count
            );
            gif_count = get_counts().punch;
        }

        "shoot" => {
            title = format!(
                "<:shibaGun:1200035732794912798> {} shoots {}!",
                ctx.author().name,
                user.name
            );
            footer = format!(
                "{} has shot {} {} times.",
                ctx.author().name,
                user.name,
                new_count
            );
            gif_count = get_counts().shoot;
        }

        "slap" => {
            title = format!("{} slaps {}!", ctx.author().name, user.name);
            footer = format!(
                "{} has slapped {} {} times.",
                ctx.author().name,
                user.name,
                new_count
            );
            gif_count = get_counts().slap;
        }

        _ => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Error")
                        .description(format!("{ERROR_EMOJI} Action not found.",))
                        .color(ERROR_EMBED_COLOR),
                ),
            )
            .await?;
            return Ok(());
        }
    }

    if new_count == 1 {
        // kinda dirty but it works
        footer = footer.replace("times", "time");
    }

    let mut rng = <rand::rngs::SmallRng as rand::SeedableRng>::from_entropy();
    let random_number: usize = rng.gen_range(1..gif_count);

    ctx.send(
        CreateReply::default()
            .content(user.mention().to_string())
            .embed(
                CreateEmbed::default()
                    .title(title)
                    .image(format!("{BASE_URL}/{action}/{random_number}.gif"))
                    .footer(CreateEmbedFooter::new(footer))
                    .color(EMBED_COLOR),
            ),
    )
    .await?;

    Ok(())
}

// This function needs to be async for poise to use it.
#[allow(clippy::unused_async)]
async fn actions_autocomplete(_: poise::Context<'_, (), Error>, partial: &str) -> Vec<String> {
    let partial = partial.to_lowercase();
    let options = ACTIONS
        .iter()
        .filter(move |action| action.starts_with(&partial))
        .collect::<Vec<_>>();

    options.iter().map(|&s| (*s).to_string()).collect()
}

#[allow(clippy::missing_errors_doc)]
pub async fn initialize_action_count() -> Result<Counts, Error> {
    Ok(reqwest::get(format!("{BASE_URL}/api.json"))
        .await?
        .json()
        .await?)
}

#[inline]
fn get_counts() -> Counts {
    unsafe { ACTION_COUNTS.expect("Unreachable") }
}

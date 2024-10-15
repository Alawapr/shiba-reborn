use std::vec;

use crate::Error;

use super::super::{API_THANKS_EMOJI, EMBED_COLOR, ERROR_EMBED_COLOR, ERROR_EMOJI};
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
use poise::CreateReply;

/// Get an image of a random animal
#[poise::command(slash_command, broadcast_typing)]
pub async fn animal(
    ctx: poise::Context<'_, (), Error>,
    #[description = "The name of the animal."]
    #[autocomplete = "animal_autocomplete"]
    animal: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    match animal.as_str() {
        "cat" => send_cat(ctx).await?,
        "dog" => send_dog(ctx).await?,
        "shiba" => send_shiba(ctx).await?,
        "fox" => send_fox(ctx).await?,
        "duck" => send_duck(ctx).await?,
        _ => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Error")
                        .description(format!("{ERROR_EMOJI} Animal not found.",))
                        .color(ERROR_EMBED_COLOR),
                ),
            )
            .await?;
        }
    }

    Ok(())
}

#[derive(serde::Deserialize)]
struct Shiba {
    #[serde(rename = "0")]
    url: String,
}

async fn send_shiba(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    let response = match reqwest::get("https://shibe.online/api/shibes").await {
        Ok(response) => response,
        Err(error) => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Error")
                        .footer(CreateEmbedFooter::new(
                            "Please contact our developers about this in the support server!",
                        ))
                        .description(format!("{ERROR_EMOJI} {error}"))
                        .color(ERROR_EMBED_COLOR),
                ),
            )
            .await?;
            return Err(error.into());
        }
    };

    let response: Shiba = response.json().await?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("Have a look at this shiba!")
                .image(response.url)
                .color(EMBED_COLOR)
                .footer(
                    CreateEmbedFooter::new("Powered by Shibe.online").icon_url(API_THANKS_EMOJI),
                ),
        ),
    )
    .await?;

    Ok(())
}

#[derive(serde::Deserialize)]
struct DogResponse {
    #[serde(rename = "fileSizeBytes")]
    #[allow(unused)]
    file_size_bytes: u64,
    url: String,
}

async fn send_dog(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    let response = match reqwest::get("https://random.dog/woof.json").await {
        Ok(response) => response,
        Err(error) => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Error")
                        .footer(CreateEmbedFooter::new(
                            "Please contact our developers about this in the support server!",
                        ))
                        .description(format!("{ERROR_EMOJI} {error}"))
                        .color(ERROR_EMBED_COLOR),
                ),
            )
            .await?;
            return Err(error.into());
        }
    };

    let response: DogResponse = response.json().await?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("Have a look at this dog!")
                .image(response.url)
                .color(EMBED_COLOR)
                .footer(CreateEmbedFooter::new("Powered by random.dog").icon_url(API_THANKS_EMOJI)),
        ),
    )
    .await?;

    Ok(())
}

#[derive(serde::Deserialize)]
struct CatResponse {
    #[serde(rename = "0")]
    inner: CatInner,
}

#[derive(serde::Deserialize)]
#[allow(unused)]
struct CatInner {
    id: String,
    url: String,
    width: i32,
    height: i32,
}

async fn send_cat(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    let response = match reqwest::get("https://api.thecatapi.com/v1/images/search").await {
        Ok(response) => response,
        Err(error) => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Error")
                        .footer(CreateEmbedFooter::new(
                            "Please contact our developers about this in the support server!",
                        ))
                        .description(format!("{ERROR_EMOJI} {error}"))
                        .color(ERROR_EMBED_COLOR),
                ),
            )
            .await?;
            return Err(error.into());
        }
    };

    let response: CatResponse = response.json().await?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("Have a look at this cat!")
                .image(response.inner.url)
                .color(EMBED_COLOR)
                .footer(
                    CreateEmbedFooter::new("Powered by thecatapi.com").icon_url(API_THANKS_EMOJI),
                ),
        ),
    )
    .await?;

    Ok(())
}

#[derive(serde::Deserialize)]
struct FoxResponse {
    image: String,
}

async fn send_fox(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    let response = match reqwest::get("https://randomfox.ca/floof").await {
        Ok(response) => response,
        Err(error) => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Error")
                        .footer(CreateEmbedFooter::new(
                            "Please contact our developers about this in the support server!",
                        ))
                        .description(format!("{ERROR_EMOJI} {error}"))
                        .color(ERROR_EMBED_COLOR),
                ),
            )
            .await?;
            return Err(error.into());
        }
    };

    let response: FoxResponse = response.json().await?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("Have a look at this fox!")
                .image(response.image)
                .color(EMBED_COLOR)
                .footer(
                    CreateEmbedFooter::new("Powered by randomfox.ca").icon_url(API_THANKS_EMOJI),
                ),
        ),
    )
    .await?;

    Ok(())
}

#[derive(serde::Deserialize)]
struct DuckResponse {
    url: String,
}

async fn send_duck(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    let response = match reqwest::get("https://random-d.uk/api/v2/random").await {
        Ok(response) => response,
        Err(error) => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Error")
                        .footer(CreateEmbedFooter::new(
                            "Please contact our developers about this in the support server!",
                        ))
                        .description(format!("{ERROR_EMOJI} {error}"))
                        .color(ERROR_EMBED_COLOR),
                ),
            )
            .await?;
            return Err(error.into());
        }
    };

    let response: DuckResponse = response.json().await?;
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("Have a look at this duck!")
                .image(response.url)
                .color(EMBED_COLOR)
                .footer(
                    CreateEmbedFooter::new("Powered by random-d.uk").icon_url(API_THANKS_EMOJI),
                ),
        ),
    )
    .await?;

    Ok(())
}

// This function needs to be async for poise to use it.
#[allow(clippy::unused_async)]
async fn animal_autocomplete(_: poise::Context<'_, (), Error>, partial: &str) -> Vec<String> {
    let options = [
        "cat".to_string(),
        "dog".to_string(),
        "shiba".to_string(),
        "fox".to_string(),
        "duck".to_string(),
    ];
    let partial = partial.to_lowercase();
    let options = options
        .iter()
        .filter(move |action| action.starts_with(&partial))
        .collect::<Vec<_>>();

    options.iter().map(|&s| (*s).to_string()).collect()
}

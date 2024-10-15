use std::vec;

use poise::{serenity_prelude::CreateEmbed, CreateReply};

use webhook::client::WebhookClient;
use super::super::EMBED_COLOR;
use crate::Error;
use poise::Modal;
type ApplicationContext<'a> = poise::ApplicationContext<'a, (), Error>;

#[derive(Modal)]
#[name = "Suggest a feature"]
struct SuggestionModal {
    #[name = "Be as clear and concise as possible."]
    #[paragraph]
    #[min_length = 10]
    // Discord has a max message length of 2000, the 100 is extra headroom
    #[max_length = 1900]
    second_input: String,
}

/// Suggest a feature for Shiba
#[poise::command(slash_command, broadcast_typing)]
pub async fn suggest(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    // ctx.defer().await?; -- using this results in an error here

    let data = SuggestionModal::execute(ctx).await?;

    // This can never be None, I think.
    let unwrapped_data = data.expect("Failed to unwrap modal data");
    
    let client = WebhookClient::new(&std::env::var("SUGGESTIONS_WEBHOOK").expect("You must provide a webhook URL to post suggestions to"));

    client
    .send(|msg| msg.content(&format!("**<@{}>**: {}", ctx.author().id, unwrapped_data.second_input.replace('@', "`@`"))))
        .await
        .expect("Failed to send message to suggestions webhook");

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .color(EMBED_COLOR)
                    .field(
                        "Thank you for your suggestion!",
                        "Your suggestion has been received and will be reviewed.",
                        false,
                    )
        ),
    )
    .await?;

    Ok(())
}
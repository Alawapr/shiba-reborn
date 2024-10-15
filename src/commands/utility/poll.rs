use log::error;
use poise::{
    serenity_prelude::{CreateEmbed, EditMessage, ReactionType},
    CreateReply,
};

use crate::{commands::EMBED_COLOR, Error};

/// Make your own poll about a topic
#[poise::command(slash_command, broadcast_typing)]
pub async fn poll(
    ctx: poise::Context<'_, (), Error>,
    #[description = "What should the poll be about?"] topic: String,
    #[description = "How long should the poll last in seconds?"] seconds: i64,
) -> Result<(), Error> {
    ctx.defer().await?;
    let topic = topic.as_str();
    let end_timestamp = chrono::Utc::now().timestamp() + seconds;
    let timestamp = format!("<t:{end_timestamp}:R>");
    let timestamp = timestamp.as_str();

    let http = ctx.http();

    let message = ctx
        .send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("Poll")
                    .field("Topic", topic, true)
                    .field("Duration", timestamp, true)
                    .field("Currently winning", "None", true)
                    .color(EMBED_COLOR),
            ),
        )
        .await?;

    let mut message = message.into_message().await?;

    let check_reaction = ReactionType::Custom {
        animated: false,
        #[allow(clippy::unreadable_literal)]
        id: 1197545020329300049.into(),
        name: Some(String::from("shibaCheck4")),
    };
    message.react(http, check_reaction).await?;

    let cross_reaction = ReactionType::Custom {
        animated: false,
        #[allow(clippy::unreadable_literal)]
        id: 1197544989475995739.into(),
        name: Some(String::from("shibaCross4")),
    };
    message.react(http, cross_reaction).await?;

    let mut checks = 0;
    let mut crosses = 0;
    while end_timestamp > chrono::Utc::now().timestamp() {
        // We create new reactions each iteration because if we didn't, we would
        // still have to clone the values from outside of the loop.
        //
        // It's going to result in the same (bad) performance.
        let check_counter_reaction = ReactionType::Custom {
            animated: false,
            #[allow(clippy::unreadable_literal)]
            id: 1197545020329300049.into(),
            name: Some(String::from("shibaCheck")),
        };
        let cross_counter_reaction = ReactionType::Custom {
            animated: false,
            #[allow(clippy::unreadable_literal)]
            id: 1197544989475995739.into(),
            name: Some(String::from("shibaCross")),
        };

        let cross_reactions = message
            .reaction_users(http, check_counter_reaction, None, None)
            .await?
            .len();

        let check_reactions = message
            .reaction_users(http, cross_counter_reaction, None, None)
            .await?
            .len();

        let ret = match check_reactions.cmp(&cross_reactions) {
            std::cmp::Ordering::Greater => message.edit(
                ctx.http(),
                EditMessage::new().embed(
                    CreateEmbed::new()
                        .title("Poll")
                        .field("Topic", topic, true)
                        .field("Duration", timestamp, true)
                        .field("Currently winning", "No", true)
                        .color(EMBED_COLOR),
                ),
            ),
            std::cmp::Ordering::Less => message.edit(
                ctx.http(),
                EditMessage::new().embed(
                    CreateEmbed::new()
                        .title("Poll")
                        .field("Topic", topic, true)
                        .field("Duration", timestamp, true)
                        .field("Currently winning", "Yes", true)
                        .color(EMBED_COLOR),
                ),
            ),
            std::cmp::Ordering::Equal => message.edit(
                ctx.http(),
                EditMessage::new().embed(
                    CreateEmbed::new()
                        .title("Poll")
                        .field("Topic", topic, true)
                        .field("Duration", timestamp, true)
                        .field("Currently winning", "Tie", true)
                        .color(EMBED_COLOR),
                ),
            ),
        }
        .await;

        if ret.is_err() {
            error!("Failed to edit message in poll: {ret:?}");
        }

        checks = check_reactions;
        crosses = cross_reactions;
    }

    let ret = match checks.cmp(&crosses) {
        std::cmp::Ordering::Greater => message.edit(
            ctx.http(),
            EditMessage::new().embed(
                CreateEmbed::new()
                    .title("Poll")
                    .field("Topic", topic, true)
                    .field("Result:", "No", true)
                    .color(EMBED_COLOR),
            ),
        ),
        std::cmp::Ordering::Less => message.edit(
            ctx.http(),
            EditMessage::new().embed(
                CreateEmbed::new()
                    .title("Poll")
                    .field("Topic", topic, true)
                    .field("Result:", "Yes", true)
                    .color(EMBED_COLOR),
            ),
        ),
        std::cmp::Ordering::Equal => message.edit(
            ctx.http(),
            EditMessage::new().embed(
                CreateEmbed::new()
                    .title("Poll")
                    .field("Topic", topic, true)
                    .field("Result:", "Tie", true)
                    .color(EMBED_COLOR),
            ),
        ),
    }
    .await;

    if ret.is_err() {
        error!("Failed to edit message in poll: {ret:?}");
    }

    message.delete_reactions(ctx.http()).await?;

    let id = message
        .interaction
        .expect("Couldn't get message interaction")
        .user
        .id
        .get();
    ctx.send(CreateReply::default().content(format!("Hey <@{id}>, the poll has finished!")))
        .await?;

    Ok(())
}

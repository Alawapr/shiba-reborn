use log::error;
use poise::{
    serenity_prelude::{CreateEmbed, CreateEmbedFooter, UserId},
    CreateReply,
};
use reqwest::Client;

use crate::{
    commands::{
        time::{parse_timestamp, Unit},
        CONFIRM_EMBED_COLOR, ERROR_EMBED_COLOR,
    },
    database, Error,
};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Reminder {
    pub(crate) message: String,
    pub(crate) user_id: UserId,
    pub(crate) timestamp: u64,
    pub(crate) id: u64,
}

impl Reminder {
    #[must_use]
    pub const fn new(message: String, user_id: UserId, timestamp: u64, id: u64) -> Self {
        Self {
            message,
            user_id,
            timestamp,
            id,
        }
    }

    #[must_use]
    pub fn new_random(message: String, user_id: UserId, timestamp: u64) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(message, user_id, timestamp, rand::Rng::gen::<u64>(&mut rng))
    }
}

/// Set a reminder to happen after a set amount of time.
#[poise::command(slash_command, broadcast_typing)]
pub async fn add_reminder(
    ctx: poise::Context<'_, (), Error>,
    #[description = "What should we remind you about?"] message: String,
    #[description = "When should we remind you? Format: **<number><unit>**"] time: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    super::init_reminder_cache();

    let parsed_timestamp = parse_timestamp(&time);
    if parsed_timestamp.is_err() {
        ctx.send(
            CreateReply::default()
                .embed(
                    CreateEmbed::new()
                        .title("Error: Invalid time format")
                        .description("Valid format is: **<number><unit>**.\nValid units are: seconds, minutes, hours, days".to_string())
                        .footer(CreateEmbedFooter::new("Examples: 10s, 10 seconds, 10 mins"))
                        .color(ERROR_EMBED_COLOR),
                )
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }
    let parsed_timestamp = parsed_timestamp.expect("Unreachable");
    let formatted_time = {
        match parsed_timestamp.unit() {
            Unit::Seconds => format!("{} seconds", parsed_timestamp.seconds()),
            Unit::Minutes => format!("{} minutes", parsed_timestamp.minutes()),
            Unit::Hours => format!("{} hours", parsed_timestamp.hours()),
            Unit::Days => format!("{} days", parsed_timestamp.days()),
            Unit::Unknown => time,
        }
    };
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Reminder added!")
                .description(format!("I'll check in on you in {formatted_time}."))
                .color(CONFIRM_EMBED_COLOR),
        ),
    )
    .await?;

    let seconds = parsed_timestamp.seconds();
    let timestamp = std::time::SystemTime::now();
    let timestamp = timestamp.duration_since(std::time::UNIX_EPOCH)?.as_secs() + seconds;
    let reminder_struct = Reminder::new_random(message, ctx.author().id, timestamp);

    database::add_reminder(&reminder_struct)?;
    super::add_reminder_to_cache(&reminder_struct, true);
    add_reminder_thread(reminder_struct);

    Ok(())
}

pub(crate) fn add_reminder_thread(reminder: Reminder) {
    std::thread::spawn(move || {
        let mut binding = tokio::runtime::Builder::new_current_thread();
        let runtime = binding.enable_all();
        runtime.build().expect("Failed to build runtime").block_on(async {
        let timestamp = reminder.timestamp;
        let user_id = reminder.user_id;
        let reminder_str = &reminder.message;
        loop {
            if std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Failed to get time since epoch")
                .as_secs()
                >= timestamp
            {
                // Because of Rust lifetimes, we can't pass the ctx here.
                // So instead we send raw requests to the Discord API manually.
                #[cfg(not(feature = "prod"))]
                let token = std::env::var("DEV_TOKEN").expect("You must provide a developer token");
                #[cfg(feature = "prod")]
                let token =
                    std::env::var("PROD_TOKEN").expect("You must provide a production token");

                let client = Client::new();

                let data = serde_json::json!({
                    "recipients": [user_id],
                });
                let response = client
                    .post("https://discord.com/api/v10/users/@me/channels")
                    .header("Authorization", format!("Bot {token}"))
                    .json(&data)
                    .send()
                    .await
                    .map_err(|e| error!("Failed to create DM channel: {e}"))
                    .expect("Failed to create DM channel");

                let channel_id = &response
                    .json::<serde_json::Value>()
                    .await
                    .map_err(|e| error!("Failed to get channel_id: {e}"))
                    .expect("Failed to get channel_id")["id"];
                let channel_id = channel_id
                    .as_str()
                    .expect("Failed to get channel_id")
                    .replace('"', "");

                let url = format!("https://discord.com/api/v10/channels/{channel_id}/messages");
                let pretty_timestamp = chrono::DateTime::<chrono::Utc>::from_timestamp(
                    timestamp
                        .try_into()
                        .expect("Failed to convert timestamp to i64"),
                    0,
                )
                .expect("Failed to convert timestamp to DateTime");

                #[allow(clippy::unreadable_literal)]
                let data = serde_json::json!({
                "content": null,
                    "embeds": [
                        {
                            "title": "Reminder",
                            "description": reminder_str,
                            "color": 12957813,
                            "footer": {
                                "text": format!("You asked to be reminded at {}", pretty_timestamp)
                            }
                        }
                    ],
                });
                client
                    .post(url)
                    .header("Authorization", format!("Bot {token}"))
                    .json(&data)
                    .send()
                    .await
                    .map_err(|e| error!("Failed to send reminder: {e}"))
                    .expect("Failed to send reminder");

                database::remove_reminder(&reminder)
                    .map_err(|e| error!("{e}"))
                    .expect("DB failed to remove reminder");
                return;
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });
    });
}

use std::collections::HashMap;

use poise::{serenity_prelude::CreateEmbed, CreateReply};

use crate::{commands::CONFIRM_EMBED_COLOR, database, Error};

/// Remove a reminder
#[poise::command(slash_command, broadcast_typing)]
pub async fn remove_reminder(
    ctx: poise::Context<'_, (), Error>,
    #[autocomplete = "reminder_autocomplete"]
    #[description = "What was the reminder about?"]
    message: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let id = message
        .chars()
        .map(|c| if c.is_numeric() { c } else { ' ' })
        .collect::<String>()
        .trim()
        .parse::<u64>()?;

    super::remove_reminder_from_cache_by_id(id);
    database::remove_reminder_by_id(id)?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Reminder removed!")
                .description(format!("Successfully removed reminder with ID: {id}"))
                .color(CONFIRM_EMBED_COLOR),
        ),
    )
    .await?;

    Ok(())
}

// This function needs to be async for poise to use it.
#[allow(clippy::unused_async)]
async fn reminder_autocomplete(ctx: poise::Context<'_, (), Error>, partial: &str) -> Vec<String> {
    let user = ctx.author();
    let mut messages: HashMap<u64, String> = HashMap::new();
    let partial = partial.to_lowercase();
    if let Some(reminders) = super::get_reminders_of_user(user.id) {
        for reminder in reminders {
            messages.insert(reminder.id, reminder.message.clone());
        }
    } else {
        let options =
            database::get_reminders_of_user(user.id).expect("Couldn't get reminders for user");
        super::add_reminders_to_cache(options.clone());
        for reminder in options {
            messages.insert(reminder.id, reminder.message);
        }
    }

    messages
        .iter()
        .filter(move |action| action.1.starts_with(&partial))
        .map(|(id, message)| format!("{message} (ID: {id})"))
        .collect()
}

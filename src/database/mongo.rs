use std::env::var;

use crate::{commands::utility::add_reminder::Reminder, Error as Err};
use mongodb::{
    bson::{doc, Document},
    sync::Client,
};
use poise::serenity_prelude::UserId;

use super::DB_CONN;

const DB_NAME: &str = "ShibaBot";

pub fn initialize_db() -> Result<(), Err> {
    let uri = var("MONGODB_URI")?;
    let client = Client::with_uri_str(uri)?;
    unsafe {
        if DB_CONN.get().is_none() {
            DB_CONN.set(client).expect("Could not set DB_CONN.");
        }
    }
    Ok(())
}

fn get_db_conn() -> Result<&'static mut Client, Err> {
    unsafe {
        super::DB_CONN.get_mut().map_or_else(
            || Err(Err::from("Error when trying to get DB connection.")),
            Ok,
        )
    }
}

pub fn get_command_count() -> Result<i64, Err> {
    let conn = get_db_conn()?;

    Ok(conn
        .database(DB_NAME)
        .collection::<Document>("information")
        .find_one(None, None)?
        .expect("Error when trying to get command count.")
        .get_i64("total_commands")
        .unwrap_or(0))
}

pub fn update_command_count(count: i64) -> Result<(), Err> {
    let conn = get_db_conn()?;

    conn.database(DB_NAME)
        .collection::<Document>("information")
        .update_one(doc! {}, doc! { "$set": { "total_commands": count } }, None)?;

    Ok(())
}

pub fn add_webhook_qotd(webhook: String, channel_id: u64) -> Result<(), Err> {
    let conn = get_db_conn()?;

    conn.database(DB_NAME)
        .collection::<Document>("qotd_webhooks")
        .insert_one(
            // MongoDB doesn't have a 'u64' type, so we use a string.
            doc! { "webhook": webhook, "channel_id": channel_id.to_string() },
            None,
        )?;

    Ok(())
}

pub fn channel_id_exists_qotd(channel_id: u64) -> Result<bool, Err> {
    let conn = get_db_conn()?;

    Ok(conn
        .database(DB_NAME)
        .collection::<Document>("qotd_webhooks")
        .find_one(doc! { "channel_id": channel_id.to_string() }, None)?
        .is_some())
}

pub fn get_webhook_by_id_qotd(channel_id: u64) -> Result<String, Err> {
    let conn = get_db_conn()?;

    Ok(conn
        .database(DB_NAME)
        .collection::<Document>("qotd_webhooks")
        .find_one(doc! { "channel_id": channel_id.to_string() }, None)?
        .expect("Error when trying to get webhook.")
        .get_str("webhook")
        .expect("Error when trying to get webhook.")
        .to_string())
}

pub fn get_all_webhooks_qotd() -> Result<Vec<String>, Err> {
    let conn = get_db_conn()?;

    Ok(conn
        .database(DB_NAME)
        .collection::<Document>("qotd_webhooks")
        .find(None, None)?
        .map(|doc| {
            doc.expect("Error when trying to get webhook.")
                .get_str("webhook")
                .expect("Error when trying to get webhook.")
                .to_string()
        })
        .collect())
}

pub fn remove_webhook_qotd(webhook_url: String) -> Result<(), Err> {
    let conn = get_db_conn()?;

    conn.database(DB_NAME)
        .collection::<Document>("qotd_webhooks")
        .delete_one(doc! { "webhook": webhook_url }, None)?;

    Ok(())
}

pub fn remove_webhook_qotd_with_id(channel_id: u64) -> Result<(), Err> {
    let conn = get_db_conn()?;

    conn.database(DB_NAME)
        .collection::<Document>("qotd_webhooks")
        .delete_one(doc! { "channel_id": channel_id.to_string() }, None)?;

    Ok(())
}

pub fn add_webhook_fotd(webhook: String, channel_id: u64) -> Result<(), Err> {
    let conn = get_db_conn()?;

    conn.database(DB_NAME)
        .collection::<Document>("fotd_webhooks")
        .insert_one(
            // MongoDB doesn't have a 'u64' type, so we use a string.
            doc! { "webhook": webhook, "channel_id": channel_id.to_string() },
            None,
        )?;

    Ok(())
}

pub fn channel_id_exists_fotd(channel_id: u64) -> Result<bool, Err> {
    let conn = get_db_conn()?;

    Ok(conn
        .database(DB_NAME)
        .collection::<Document>("fotd_webhooks")
        .find_one(doc! { "channel_id": channel_id.to_string() }, None)?
        .is_some())
}

pub fn get_webhook_by_id_fotd(channel_id: u64) -> Result<String, Err> {
    let conn = get_db_conn()?;

    Ok(conn
        .database(DB_NAME)
        .collection::<Document>("fotd_webhooks")
        .find_one(doc! { "channel_id": channel_id.to_string() }, None)?
        .expect("Error when trying to get webhook.")
        .get_str("webhook")
        .expect("Error when trying to get webhook.")
        .to_string())
}

pub fn get_all_webhooks_fotd() -> Result<Vec<String>, Err> {
    let conn = get_db_conn()?;

    Ok(conn
        .database(DB_NAME)
        .collection::<Document>("fotd_webhooks")
        .find(None, None)?
        .map(|doc| {
            doc.expect("Error when trying to get webhook.")
                .get_str("webhook")
                .expect("Error when trying to get webhook.")
                .to_string()
        })
        .collect())
}

pub fn remove_webhook_fotd(webhook_url: String) -> Result<(), Err> {
    let conn = get_db_conn()?;

    conn.database(DB_NAME)
        .collection::<Document>("fotd_webhooks")
        .delete_one(doc! { "webhook": webhook_url }, None)?;

    Ok(())
}

pub fn remove_webhook_fotd_with_id(channel_id: u64) -> Result<(), Err> {
    let conn = get_db_conn()?;

    conn.database(DB_NAME)
        .collection::<Document>("fotd_webhooks")
        .delete_one(doc! { "channel_id": channel_id.to_string() }, None)?;

    Ok(())
}

pub fn add_reminder(reminder: &Reminder) -> Result<(), Err> {
    let conn = get_db_conn()?;
    let reminder_str = &reminder.message;
    let user_id = reminder.user_id;
    let timestamp = reminder.timestamp;
    let id = reminder.id;

    conn.database(DB_NAME)
        .collection::<Document>("reminders")
        .insert_one(
            doc! {
                "reminder": reminder_str,
                "user_id": user_id.to_string(),
                "timestamp": timestamp.to_string(),
                "id": id.to_string()
            },
            None,
        )
        .expect("Error when trying to add reminder.");

    Ok(())
}

pub fn remove_reminder(reminder: &Reminder) -> Result<(), Err> {
    let conn = get_db_conn()?;

    conn.database(DB_NAME)
        .collection::<Document>("reminders")
        .delete_one(doc! { "id": reminder.id.to_string() }, None)?;

    Ok(())
}

pub fn remove_reminder_by_id(id: u64) -> Result<(), Err> {
    remove_reminder(&Reminder::new(id.to_string(), 1.into(), 0, 0))
}

pub fn get_all_reminders() -> Result<Vec<Reminder>, Err> {
    let conn = get_db_conn()?;

    let messages: Vec<String> = conn
        .database(DB_NAME)
        .collection::<Document>("reminders")
        .find(None, None)?
        .map(|doc| {
            doc.expect("Error when trying to get reminder.")
                .get_str("reminder")
                .expect("Error when trying to get reminder.")
                .to_string()
        })
        .collect();

    let user_ids: Vec<String> = conn
        .database(DB_NAME)
        .collection::<Document>("reminders")
        .find(None, None)?
        .map(|doc| {
            doc.expect("Error when trying to get reminder.")
                .get_str("user_id")
                .expect("Error when trying to get reminder.")
                .to_string()
        })
        .collect();

    let timestamps: Vec<String> = conn
        .database(DB_NAME)
        .collection::<Document>("reminders")
        .find(None, None)?
        .map(|doc| {
            doc.expect("Error when trying to get reminder.")
                .get_str("timestamp")
                .expect("Error when trying to get reminder.")
                .to_string()
        })
        .collect();

    let ids: Vec<String> = conn
        .database(DB_NAME)
        .collection::<Document>("reminders")
        .find(None, None)?
        .map(|doc| {
            doc.expect("Error when trying to get reminder.")
                .get_str("id")
                .expect("Error when trying to get reminder.")
                .to_string()
        })
        .collect();

    let mut reminders = Vec::with_capacity(messages.len());
    for i in 0..messages.len() {
        reminders.push(Reminder::new(
            messages[i].clone(),
            UserId::new(
                user_ids[i]
                    .parse::<u64>()
                    .expect("Error when trying to parse user_id."),
            ),
            timestamps[i]
                .parse::<u64>()
                .expect("Error when trying to parse timestamp."),
            ids[i]
                .parse::<u64>()
                .expect("Error when trying to parse id."),
        ));
    }

    Ok(reminders)
}

pub fn get_reminders_of_user(user_id: UserId) -> Result<Vec<Reminder>, Err> {
    let conn = get_db_conn()?;

    let messages: Vec<String> = conn
        .database(DB_NAME)
        .collection::<Document>("reminders")
        .find(doc! { "user_id": user_id.to_string() }, None)?
        .map(|doc| {
            doc.expect("Error when trying to get reminder.")
                .get_str("reminder")
                .expect("Error when trying to get reminder.")
                .to_string()
        })
        .collect();

    let timestamps: Vec<String> = conn
        .database(DB_NAME)
        .collection::<Document>("reminders")
        .find(doc! { "user_id": user_id.to_string() }, None)?
        .map(|doc| {
            doc.expect("Error when trying to get reminder.")
                .get_str("timestamp")
                .expect("Error when trying to get reminder.")
                .to_string()
        })
        .collect();

    let ids: Vec<String> = conn
        .database(DB_NAME)
        .collection::<Document>("reminders")
        .find(doc! { "user_id": user_id.to_string() }, None)?
        .map(|doc| {
            doc.expect("Error when trying to get reminder.")
                .get_str("id")
                .expect("Error when trying to get reminder.")
                .to_string()
        })
        .collect();

    let mut reminders = Vec::with_capacity(messages.len());

    for i in 0..messages.len() {
        reminders.push(Reminder::new(
            messages[i].clone(),
            UserId::new(
                user_id
                    .to_string()
                    .parse::<u64>()
                    .expect("Error when trying to parse user_id."),
            ),
            timestamps[i]
                .parse::<u64>()
                .expect("Error when trying to parse timestamp."),
            ids[i]
                .parse::<u64>()
                .expect("Error when trying to parse id."),
        ));
    }

    Ok(reminders)
}

pub fn get_action_count(
    action: &str,
    from_user_id: UserId,
    to_user_id: UserId,
) -> Result<i32, Err> {
    let conn = get_db_conn()?;

    let count: Vec<i32> = conn
        .database(DB_NAME)
        .collection::<Document>("actions")
        .find(
            doc! {
                "action": action,
                "from_user_id": from_user_id.to_string(),
                "to_user_id": to_user_id.to_string()
            },
            None,
        )?
        .map(|doc| {
            doc.expect("Error when trying to get action count.")
                .get_i32("amount")
                .expect("Error when trying to get action count.")
        })
        .collect();

    if count.is_empty() {
        // First time these two users performed this action, create a new entry
        conn.database(DB_NAME)
            .collection::<Document>("actions")
            .insert_one(
                doc! {
                    "action": action,
                    "from_user_id": from_user_id.to_string(),
                    "to_user_id": to_user_id.to_string(),
                    "amount": 0
                },
                None,
            )?;

        Ok(0)
    } else {
        Ok(count[0])
    }
}

pub fn update_action_count(
    action: &str,
    from_user_id: UserId,
    to_user_id: UserId,
    amount: i32,
) -> Result<(), Err> {
    let conn = get_db_conn()?;

    conn.database(DB_NAME)
        .collection::<Document>("actions")
        .update_one(
            doc! {
                "action": action,
                "from_user_id": from_user_id.to_string(),
                "to_user_id": to_user_id.to_string()
            },
            doc! {
                "$set": {
                    "amount": amount
                }
            },
            None,
        )
        .expect("Error when trying to update action count.");

    Ok(())
}

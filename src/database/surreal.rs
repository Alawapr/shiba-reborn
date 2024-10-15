use std::env::var;

use poise::serenity_prelude::UserId;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

use crate::{commands::utility::add_reminder::Reminder, database::Err};

use super::DB_CONN;

pub async fn initialize_db() -> Result<(), Err> {
    let url = var("SURREAL_URL")?;
    let db = var("SURREAL_DB")?;
    let ns = var("SURREAL_NS")?;
    let user = var("SURREAL_USER")?;
    let password = var("SURREAL_PASSWORD")?;

    let conn = Surreal::new::<Ws>(url).await?;
    conn.signin(Root {
        username: &user,
        password: &password,
    })
    .await?;
    conn.use_ns(ns).use_db(db).await?;

    unsafe {
        if DB_CONN.get().is_none() {
            DB_CONN.set(conn).expect("Could not set DB_CONN.");
        }
    }

    Ok(())
}

fn get_db_conn(
) -> Result<&'static mut surrealdb::Surreal<surrealdb::engine::remote::ws::Client>, Err> {
    unsafe {
        super::DB_CONN.get_mut().map_or_else(
            || Err(Err::from("Error when trying to get DB connection.")),
            Ok,
        )
    }
}

pub async fn get_command_count() -> Result<i64, Err> {
    get_db_conn()?
        .query("SELECT total_commands FROM information")
        .await?
        .take::<surrealdb::sql::Value>(0)?
        .first()
        .into_json()["total_commands"]
        .as_i64()
        .map_or_else(
            || {
                println!("Error when trying to get command count.");
                Err(Err::from("Error when trying to get command count."))
            },
            Ok,
        )
}

pub async fn update_command_count(count: i64) -> Result<(), Err> {
    get_db_conn()?
        .query("UPDATE information SET total_commands = $count")
        .bind(("count", count))
        .await?;

    Ok(())
}

pub async fn add_webhook_qotd(webhook: String, channel_id: u64) -> Result<(), Err> {
    get_db_conn()?
        .query("INSERT INTO qotd_webhooks (webhook, channel_id) VALUES ($webhook, $channel_id)")
        .bind(("webhook", webhook))
        .bind(("channel_id", channel_id))
        .await?;

    Ok(())
}

pub async fn channel_id_exists_qotd(channel_id: u64) -> Result<bool, Err> {
    if get_db_conn()?
        .query("SELECT * FROM qotd_webhooks WHERE channel_id = $channel_id")
        .bind(("channel_id", channel_id))
        .await?
        .take::<surrealdb::sql::Value>(0)?
        .first()
        .into_json()
        .is_null()
    {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub async fn get_webhook_by_id_qotd(channel_id: u64) -> Result<String, Err> {
    get_db_conn()?
        .query("SELECT webhook FROM qotd_webhooks WHERE channel_id = $channel_id")
        .bind(("channel_id", channel_id))
        .await?
        .take::<surrealdb::sql::Value>(0)?
        .first()
        .into_json()["webhook"]
        .as_str()
        .map_or_else(
            || Err(Err::from("Error when getting webhook by ID. (qotd)")),
            Ok,
        )
        .map(String::from)
}

pub async fn remove_webhook_qotd(webhook_url: &str) -> Result<(), Err> {
    get_db_conn()?
        .query("DELETE FROM qotd_webhooks WHERE webhook = $webhook_url")
        .bind(("webhook_url", webhook_url))
        .await?;

    Ok(())
}

pub async fn get_all_webhooks_qotd() -> Result<Vec<String>, Err> {
    get_db_conn()?
        .query("SELECT webhook FROM qotd_webhooks")
        .await?
        .take::<surrealdb::sql::Value>(0)?
        .into_json()
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|v| {
                    v["webhook"]
                        .as_str()
                        .expect("Error when getting webhook.")
                        .to_string()
                })
                .collect()
        })
        .map_or_else(
            || Err(Err::from("Error when getting all webhooks. (qotd)")),
            Ok,
        )
}

pub async fn remove_webhook_qotd_with_id(channel_id: u64) -> Result<(), Err> {
    get_db_conn()?
        .query("DELETE FROM qotd_webhooks WHERE channel_id = $channel_id")
        .bind(("channel_id", channel_id))
        .await?;

    Ok(())
}

pub async fn add_webhook_fotd(webhook: String, channel_id: u64) -> Result<(), Err> {
    get_db_conn()?
        .query("INSERT INTO fotd_webhooks (webhook, channel_id) VALUES ($webhook, $channel_id)")
        .bind(("webhook", webhook))
        .bind(("channel_id", channel_id))
        .await?;

    Ok(())
}

pub async fn channel_id_exists_fotd(channel_id: u64) -> Result<bool, Err> {
    if get_db_conn()?
        .query("SELECT * FROM fotd_webhooks WHERE channel_id = $channel_id")
        .bind(("channel_id", channel_id))
        .await?
        .take::<surrealdb::sql::Value>(0)?
        .first()
        .into_json()
        .is_null()
    {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub async fn get_webhook_by_id_fotd(channel_id: u64) -> Result<String, Err> {
    get_db_conn()?
        .query("SELECT webhook FROM fotd_webhooks WHERE channel_id = $channel_id")
        .bind(("channel_id", channel_id))
        .await?
        .take::<surrealdb::sql::Value>(0)?
        .first()
        .into_json()["webhook"]
        .as_str()
        .map_or_else(
            || Err(Err::from("Error when getting webhook by ID. (fotd)")),
            Ok,
        )
        .map(String::from)
}

pub async fn remove_webhook_fotd(webhook_url: &str) -> Result<(), Err> {
    get_db_conn()?
        .query("DELETE FROM fotd_webhooks WHERE webhook = $webhook_url")
        .bind(("webhook_url", webhook_url))
        .await?;

    Ok(())
}

pub async fn get_all_webhooks_fotd() -> Result<Vec<String>, Err> {
    get_db_conn()?
        .query("SELECT webhook FROM fotd_webhooks")
        .await?
        .take::<surrealdb::sql::Value>(0)?
        .into_json()
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|v| {
                    v["webhook"]
                        .as_str()
                        .expect("Error when getting webhook.")
                        .to_string()
                })
                .collect()
        })
        .map_or_else(
            || Err(Err::from("Error when getting all webhooks. (fotd)")),
            Ok,
        )
}

pub async fn remove_webhook_fotd_with_id(channel_id: u64) -> Result<(), Err> {
    get_db_conn()?
        .query("DELETE FROM fotd_webhooks WHERE channel_id = $channel_id")
        .bind(("channel_id", channel_id))
        .await?;

    Ok(())
}

pub async fn add_reminder(reminder: &Reminder) -> Result<(), Err> {
    get_db_conn()?
        .query("INSERT INTO reminders (reminder, user_id, timestamp, id) VALUES ($reminder, $user_id, $timestamp, $id)")
        .bind(("reminder", reminder.message.clone()))
        .bind(("user_id", reminder.user_id.get()))
        .bind(("timestamp", reminder.timestamp))
        .bind(("id", reminder.id))
        .await?;

    Ok(())
}

pub async fn remove_reminder(reminder: &Reminder) -> Result<(), Err> {
    get_db_conn()?
        .query("DELETE FROM reminders WHERE id = $id")
        .bind(("id", reminder.id))
        .await?;

    Ok(())
}

pub async fn remove_reminder_by_id(id: u64) -> Result<(), Err> {
    remove_reminder(&Reminder {
        id,
        ..Default::default()
    })
    .await
}

pub async fn get_all_reminders() -> Result<Vec<Reminder>, Err> {
    let json = get_db_conn()?
        .query("SELECT * FROM reminders")
        .await?
        .take::<surrealdb::sql::Value>(0)?
        .into_json();

    dbg!(json.clone());
    if json.is_null() {
        println!("Error when trying to get all reminders.");
        return Ok(vec![]);
    }

    json.as_array()
        .map(|arr| {
            arr.iter()
                .map(|v| Reminder {
                    id: v["id"]
                        .as_str()
                        .expect("Error when trying to get reminder ID.")
                        .split_once(':')
                        .expect("Unreachable")
                        .1
                        .parse::<u64>()
                        .expect("Parsing error when trying to get reminder ID."),
                    user_id: UserId::new(
                        v["user_id"]
                            .as_u64()
                            .expect("Error when trying to get user ID."),
                    ),
                    message: v["reminder"]
                        .as_str()
                        .expect("Error when trying to get reminder.")
                        .to_string(),
                    timestamp: v["timestamp"]
                        .as_u64()
                        .expect("Error when trying to get timestamp."),
                })
                .collect()
        })
        .map_or_else(|| Err(Err::from("Error when getting all reminders.")), Ok)
}

pub async fn get_reminders_of_user(user_id: UserId) -> Result<Vec<Reminder>, Err> {
    get_db_conn()?
        .query("SELECT * FROM reminders WHERE user_id = $user_id")
        .bind(("user_id", user_id.get()))
        .await?
        .take::<surrealdb::sql::Value>(0)?
        .into_json()
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|v| Reminder {
                    id: v["id"]
                        .as_u64()
                        .expect("Error when trying to get reminder ID."),
                    user_id: UserId::new(
                        v["user_id"]
                            .as_u64()
                            .expect("Error when trying to get user ID."),
                    ),
                    message: v["reminder"]
                        .as_str()
                        .expect("Error when trying to get reminder.")
                        .to_string(),
                    timestamp: v["timestamp"]
                        .as_u64()
                        .expect("Error when trying to get timestamp."),
                })
                .collect()
        })
        .map_or_else(
            || Err(Err::from("Error when getting reminders of user.")),
            Ok,
        )
}

pub async fn get_action_count(
    action: &str,
    from_user_id: UserId,
    to_user_id: UserId,
) -> Result<i32, Err> {
    get_db_conn()?
        .query("SELECT amount FROM actions WHERE action = $action AND from_user_id = $from_user_id AND to_user_id = $to_user_id")
        .bind(("action", action))
        .bind(("from_user_id", from_user_id.get()))
        .bind(("to_user_id", to_user_id.get()))
        .await?
        .take::<surrealdb::sql::Value>(0)?
        .first()
        .into_json()["amount"]
        .as_i64()
        .map_or_else(
            || Err(Err::from("Error when getting action count.")),
            Ok,
        ).map(|i| i.try_into().expect("Error when converting action count: overflow."))
}

pub async fn update_action_count(
    action: &str,
    from_user_id: UserId,
    to_user_id: UserId,
    amount: i32,
) -> Result<(), Err> {
    get_db_conn()?
        .query("UPDATE actions SET amount = $amount WHERE action = $action AND from_user_id = $from_user_id AND to_user_id = $to_user_id")
        .bind(("action", action))
        .bind(("from_user_id", from_user_id.get()))
        .bind(("to_user_id", to_user_id.get()))
        .bind(("amount", amount))
        .await?;

    Ok(())
}

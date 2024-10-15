use std::env::var;
use std::time::{Duration, SystemTime};

use log::info;
use mysql::{params, Pool, Result, Row};
use mysql::{prelude::*, PooledConn};
use poise::serenity_prelude::UserId;

use crate::Error as Err;

use crate::commands::utility::add_reminder::Reminder;
use crate::VOLATILE_TIME;

use super::DB_CONN;

pub fn initialize_db() -> Result<(), Err> {
    let host = var("SQL_HOST")?;
    let port = var("SQL_PORT")?;
    let user = var("SQL_USER")?;
    let password = var("SQL_PASSWORD")?;
    let database = var("SQL_DATABASE")?;
    let url = format!("mysql://{user}:{password}@{host}:{port}/{database}");
    let pool = Pool::new(url.as_str())?;
    unsafe {
        if DB_CONN.get().is_none() {
            DB_CONN
                .set(
                    pool.get_conn()
                        .expect("Could not get connection from pool."),
                )
                // Can't use ? instead of .expect() because gay mysql thing doesn't work with it
                .expect("Could not set DB_CONN.");
            info!("Connection to database successfully established.");
        } else {
            info!("Connection to database already established, deleting old connection and initializing new one");

            // .take() empties `DB_CONN`
            DB_CONN.take();

            DB_CONN
                .set(
                    pool.get_conn()
                        .expect("Could not get connection from pool."),
                )
                // Can't use ? instead of .expect() because gay mysql thing doesn't work with it
                .expect("Could not set DB_CONN.");

            info!("New connection to database successfully established.");
        }

        Ok(())
    }
}

/// A safe wrapper for getting the database URL.
fn get_db_conn() -> Result<&'static mut PooledConn, Err> {
    unsafe {
        // .expect() will always return a value since this is always
        // executed after `VOLATILE_TIME` is initialized.
        if VOLATILE_TIME.expect("Unreachable").elapsed()? > Duration::from_secs(25200) {
            info!("7 hours have passed since the DB connection got created. Reconnecting...");
            super::initialize_db()?;
            VOLATILE_TIME = Some(SystemTime::now());
        }

        super::DB_CONN.get_mut().map_or_else(
            || Err(Err::from("Error when trying to get DB connection.")),
            Ok,
        )
    }
}

pub fn get_command_count() -> Result<i64, Err> {
    let conn = get_db_conn()?;
    let count: Vec<i64> = conn.exec("SELECT total_commands FROM information LIMIT 1;", ())?;

    // Count is the only row in the table, so we can just return count[0].
    Ok(count[0])
}

pub fn update_command_count(count: i64) -> Result<(), Err> {
    let conn = get_db_conn()?;

    conn.exec_drop(
        "UPDATE information SET total_commands = :count;",
        params! { "count" => count },
    )?;

    Ok(())
}

pub fn add_webhook_qotd(webhook: String, channel_id: u64) -> Result<(), Err> {
    let conn = get_db_conn()?;

    if !channel_id_exists_qotd(channel_id)? {
        conn.exec_drop(
            "INSERT INTO qotd_webhooks (webhook, channel_id) VALUES (:webhook, :channel_id)",
            params! {
                "webhook" => webhook,
                "channel_id" => channel_id.to_string(),
            },
        )?;
    }

    Ok(())
}

pub fn channel_id_exists_qotd(channel_id: u64) -> Result<bool, Err> {
    let conn = get_db_conn()?;
    let channel_exists: Option<bool> = conn.exec_first(
        "SELECT EXISTS(SELECT 1 FROM qotd_webhooks WHERE channel_id = :channel_id)",
        params! {
            "channel_id" => channel_id.to_string(),
        },
    )?;

    channel_exists.map_or_else(
        || Err(Err::from("Error when checking if channel exists. (qotd)")),
        Ok,
    )
}

pub fn get_webhook_by_id_qotd(channel_id: u64) -> Result<String, Err> {
    let conn = get_db_conn()?;
    let webhook = conn.exec_first(
        "SELECT webhook FROM qotd_webhooks WHERE channel_id = :channel_id",
        params! {
            "channel_id" => channel_id.to_string(),
        },
    )?;

    webhook.map_or_else(
        || Err(Err::from("Error when getting webhook by ID. (qotd)")),
        Ok,
    )
}

pub fn get_all_webhooks_qotd() -> Result<Vec<String>, Err> {
    let conn = get_db_conn()?;
    let webhooks: Vec<String> = conn.exec("SELECT webhook FROM qotd_webhooks", ())?;
    Ok(webhooks)
}

pub fn remove_webhook_qotd(webhook_url: &str) -> Result<(), Err> {
    let conn = get_db_conn()?;

    conn.exec_drop(
        "DELETE FROM qotd_webhooks WHERE webhook = :webhook_url;",
        params! {
            "webhook_url" => webhook_url,
        },
    )?;

    Ok(())
}

pub fn remove_webhook_qotd_with_id(channel_id: u64) -> Result<(), Err> {
    let conn = get_db_conn()?;
    conn.exec_drop(
        "DELETE FROM qotd_webhooks WHERE channel_id = :channel_id;",
        params! {
            "channel_id" => channel_id,
        },
    )?;

    Ok(())
}

pub fn add_webhook_fotd(webhook: String, channel_id: u64) -> Result<(), Err> {
    let conn = get_db_conn()?;

    if !channel_id_exists_fotd(channel_id)? {
        conn.exec_drop(
            "INSERT INTO fotd_webhooks (webhook, channel_id) VALUES (:webhook, :channel_id)",
            params! {
                "webhook" => webhook,
                "channel_id" => channel_id.to_string(),
            },
        )?;
    }

    Ok(())
}

pub fn channel_id_exists_fotd(channel_id: u64) -> Result<bool, Err> {
    let conn = get_db_conn()?;
    let channel_exists: Option<bool> = conn.exec_first(
        "SELECT EXISTS(SELECT 1 FROM fotd_webhooks WHERE channel_id = :channel_id)",
        params! {
            "channel_id" => channel_id.to_string(),
        },
    )?;

    channel_exists.map_or_else(
        || Err(Err::from("Error when checking if channel exists. (fotd)")),
        Ok,
    )
}

pub fn get_webhook_by_id_fotd(channel_id: u64) -> Result<String, Err> {
    let conn = get_db_conn()?;
    let webhook = conn.exec_first(
        "SELECT webhook FROM fotd_webhooks WHERE channel_id = :channel_id",
        params! {
            "channel_id" => channel_id.to_string(),
        },
    )?;

    webhook.map_or_else(
        || Err(Err::from("Error when getting webhook by ID. (fotd)")),
        Ok,
    )
}

pub fn get_all_webhooks_fotd() -> Result<Vec<String>, Err> {
    let conn = get_db_conn()?;
    let webhooks: Vec<String> = conn.exec("SELECT webhook FROM fotd_webhooks", ())?;
    Ok(webhooks)
}

pub fn remove_webhook_fotd(webhook_url: &str) -> Result<(), Err> {
    let conn = get_db_conn()?;

    conn.exec_drop(
        "DELETE FROM fotd_webhooks WHERE webhook = :webhook_url;",
        params! {
            "webhook_url" => webhook_url,
        },
    )?;

    Ok(())
}

pub fn remove_webhook_fotd_with_id(channel_id: u64) -> Result<(), Err> {
    let conn = get_db_conn()?;

    conn.exec_drop(
        "DELETE FROM fotd_webhooks WHERE channel_id = :channel_id;",
        params! {
            "channel_id" => channel_id,
        },
    )?;

    Ok(())
}

pub fn add_reminder(reminder: &Reminder) -> Result<(), Err> {
    let conn = get_db_conn()?;
    let reminder_str = &reminder.message;
    let user_id = reminder.user_id;
    let timestamp = reminder.timestamp;
    let id = reminder.id;
    conn.exec_drop(
        "INSERT INTO reminders (reminder, user_id, timestamp, id) VALUES (:reminder, :user_id, :timestamp, :id)",
        params! {
            "reminder" => reminder_str,
            "user_id" => user_id.to_string(),
            "timestamp" => timestamp,
            "id" => id
        },
    )?;

    Ok(())
}

pub fn remove_reminder(reminder: &Reminder) -> Result<(), Err> {
    let conn = get_db_conn()?;

    conn.exec_drop(
        "DELETE FROM reminders WHERE reminder = :reminder AND user_id = :user_id AND timestamp = :timestamp AND id = :id",
        params! {
            "reminder" => &reminder.message,
            "user_id" => reminder.user_id.to_string(),
            "timestamp" => reminder.timestamp,
            "id" => reminder.id
        },
    )?;

    Ok(())
}

pub fn remove_reminder_by_id(id: u64) -> Result<(), Err> {
    let conn = get_db_conn()?;

    conn.exec_drop(
        "DELETE FROM reminders WHERE id = :id",
        params! {
            "id" => id,
        },
    )?;

    Ok(())
}

pub fn get_all_reminders() -> Result<Vec<Reminder>, Err> {
    let conn = get_db_conn()?;
    let rows: Vec<Row> = conn.exec("SELECT * FROM reminders", ())?;
    let mut reminders: Vec<Reminder> = Vec::new();
    for reminder in rows {
        reminders.push(Reminder {
            message: reminder
                .get::<String, _>("reminder")
                .expect("Could not get reminder."),
            user_id: reminder
                .get::<String, _>("user_id")
                .expect("Could not get user ID.")
                .parse::<u64>()
                .expect("Could not parse user ID.")
                .into(),
            timestamp: reminder
                .get::<u64, _>("timestamp")
                .expect("Could not get timestamp."),
            id: reminder.get::<u64, _>("id").expect("Could not get ID."),
        });
    }
    Ok(reminders)
}

pub fn get_reminders_of_user(user_id: UserId) -> Result<Vec<Reminder>, Err> {
    let conn = get_db_conn()?;
    let rows: Vec<Row> = conn.exec(
        "SELECT * FROM reminders WHERE user_id = :user_id",
        params! {
            "user_id" => user_id.to_string(),
        },
    )?;
    let mut reminders: Vec<Reminder> = Vec::new();
    for reminder in rows {
        reminders.push(Reminder {
            message: reminder
                .get::<String, _>("reminder")
                .expect("Could not get reminder."),
            user_id: reminder
                .get::<String, _>("user_id")
                .expect("Could not get user ID.")
                .parse::<u64>()
                .expect("Could not parse user ID.")
                .into(),
            timestamp: reminder
                .get::<u64, _>("timestamp")
                .expect("Could not get timestamp."),
            id: reminder.get::<u64, _>("id").expect("Could not get ID."),
        });
    }
    Ok(reminders)
}

pub fn get_action_count(
    action: &str,
    from_user_id: UserId,
    to_user_id: UserId,
) -> Result<i32, Err> {
    let conn = get_db_conn()?;

    let count: Vec<i32> = conn.exec(
        "SELECT amount FROM actions WHERE action = :action AND from_user_id = :from_user_id AND to_user_id = :to_user_id",
        params! {
            "action" => action,
            "from_user_id" => from_user_id.to_string(),
            "to_user_id" => to_user_id.to_string(),
        },
    )?;

    if count.is_empty() {
        // thanks rust-analyzer <3
        // Insert new row into actions cause there isn't one yet, this is the first time these two users performed this action
        conn.exec_drop(
            "INSERT INTO actions (action, from_user_id, to_user_id, amount) VALUES (:action, :from_user_id, :to_user_id, 1)",
            params! {
                "action" => action,
                "from_user_id" => from_user_id.to_string(),
                "to_user_id" => to_user_id.to_string(),
                "amount" => 0,
            },
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

    conn.exec_drop(
        "UPDATE actions SET amount = :amount WHERE action = :action AND from_user_id = :from_user_id AND to_user_id = :to_user_id", 
        params! {
            "action" => action,
            "from_user_id" => from_user_id.to_string(),
            "to_user_id" => to_user_id.to_string(),
            "amount" => amount
        }
    )?;

    Ok(())
}

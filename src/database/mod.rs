use crate::{commands::utility::add_reminder::Reminder, Error as Err};
use once_cell::sync::OnceCell;
use poise::serenity_prelude::UserId;

#[cfg(feature = "mongodb")]
mod mongo;
#[cfg(feature = "mysql")]
mod sql;
#[cfg(feature = "surrealdb")]
mod surreal;

#[cfg(feature = "mongodb")]
static mut DB_CONN: OnceCell<mongodb::sync::Client> = OnceCell::new();

#[cfg(feature = "mysql")]
static mut DB_CONN: OnceCell<mysql::PooledConn> = OnceCell::new();

#[cfg(feature = "surrealdb")]
static mut DB_CONN: OnceCell<surrealdb::Surreal<surrealdb::engine::remote::ws::Client>> =
    OnceCell::new();
#[cfg(feature = "surrealdb")]
lazy_static::lazy_static! {
    static ref TOKIO_RUNTIME: tokio::runtime::Runtime = {
        match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,
            Err(err) => panic!("Error occurred when starting the underlying async runtime: {err}")
        }
    };
}

#[cfg(any(feature = "mongodb", feature = "surrealdb"))]
macro_rules! spawn_thread {
    ($($args:tt)*) => {
        std::thread::spawn(move || {
            #[cfg(feature = "surrealdb")]
            {
                TOKIO_RUNTIME.block_on($($args)*)
            }
            #[cfg(not(feature = "surrealdb"))]
            {
                $($args)*
            }
        })
        .join()
        .expect("Failed to wait for thread")
    };
}

pub fn initialize_db() -> Result<(), Err> {
    #[cfg(feature = "mysql")]
    {
        sql::initialize_db()
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::initialize_db())
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::initialize_db())
    }
}

pub fn get_command_count() -> Result<i64, Err> {
    #[cfg(feature = "mysql")]
    {
        sql::get_command_count()
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::get_command_count())
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::get_command_count())
    }
}

pub fn update_command_count(count: i64) -> Result<(), Err> {
    #[cfg(feature = "mysql")]
    {
        sql::update_command_count(count)
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::update_command_count(count))
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::update_command_count(count))
    }
}

pub fn add_webhook_qotd(webhook: String, channel_id: u64) -> Result<(), Err> {
    #[cfg(feature = "mysql")]
    {
        sql::add_webhook_qotd(webhook, channel_id)
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::add_webhook_qotd(webhook, channel_id))
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::add_webhook_qotd(webhook, channel_id))
    }
}

pub fn channel_id_exists_qotd(channel_id: u64) -> Result<bool, Err> {
    #[cfg(feature = "mysql")]
    {
        sql::channel_id_exists_qotd(channel_id)
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::channel_id_exists_qotd(channel_id))
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::channel_id_exists_qotd(channel_id))
    }
}

pub fn get_webhook_by_id_qotd(channel_id: u64) -> Result<String, Err> {
    #[cfg(feature = "mysql")]
    {
        sql::get_webhook_by_id_qotd(channel_id)
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::get_webhook_by_id_qotd(channel_id))
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::get_webhook_by_id_qotd(channel_id))
    }
}

pub fn get_all_webhooks_qotd() -> Result<Vec<String>, Err> {
    #[cfg(feature = "mysql")]
    {
        sql::get_all_webhooks_qotd()
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::get_all_webhooks_qotd())
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::get_all_webhooks_qotd())
    }
}

pub fn remove_webhook_qotd(webhook_url: &str) -> Result<(), Err> {
    #[cfg(feature = "mysql")]
    {
        sql::remove_webhook_qotd(webhook_url)
    }
    #[cfg(feature = "mongodb")]
    {
        let webhook_url = webhook_url.to_string();
        spawn_thread!(mongo::remove_webhook_qotd(webhook_url))
    }
    #[cfg(feature = "surrealdb")]
    {
        let webhook_url = webhook_url.to_string();
        spawn_thread!(surreal::remove_webhook_qotd(&webhook_url))
    }
}

pub fn remove_webhook_qotd_with_id(channel_id: u64) -> Result<(), Err> {
    #[cfg(feature = "mysql")]
    {
        sql::remove_webhook_qotd_with_id(channel_id)
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::remove_webhook_qotd_with_id(channel_id))
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::remove_webhook_qotd_with_id(channel_id))
    }
}

pub fn add_webhook_fotd(webhook: String, channel_id: u64) -> Result<(), Err> {
    #[cfg(feature = "mysql")]
    {
        sql::add_webhook_fotd(webhook, channel_id)
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::add_webhook_fotd(webhook, channel_id))
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::add_webhook_fotd(webhook, channel_id))
    }
}

pub fn channel_id_exists_fotd(channel_id: u64) -> Result<bool, Err> {
    #[cfg(feature = "mysql")]
    {
        sql::channel_id_exists_fotd(channel_id)
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::channel_id_exists_fotd(channel_id))
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::channel_id_exists_fotd(channel_id))
    }
}

pub fn get_webhook_by_id_fotd(channel_id: u64) -> Result<String, Err> {
    #[cfg(feature = "mysql")]
    {
        sql::get_webhook_by_id_fotd(channel_id)
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::get_webhook_by_id_fotd(channel_id))
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::get_webhook_by_id_fotd(channel_id))
    }
}

pub fn get_all_webhooks_fotd() -> Result<Vec<String>, Err> {
    #[cfg(feature = "mysql")]
    {
        sql::get_all_webhooks_fotd()
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::get_all_webhooks_fotd())
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::get_all_webhooks_fotd())
    }
}

pub fn remove_webhook_fotd(webhook_url: &str) -> Result<(), Err> {
    #[cfg(feature = "mysql")]
    {
        sql::remove_webhook_fotd(webhook_url)
    }
    #[cfg(feature = "mongodb")]
    {
        let webhook_url = webhook_url.to_string();
        spawn_thread!(mongo::remove_webhook_fotd(webhook_url))
    }
    #[cfg(feature = "surrealdb")]
    {
        let webhook_url = webhook_url.to_string();
        spawn_thread!(surreal::remove_webhook_fotd(&webhook_url))
    }
}

pub fn remove_webhook_fotd_with_id(channel_id: u64) -> Result<(), Err> {
    #[cfg(feature = "mysql")]
    {
        sql::remove_webhook_fotd_with_id(channel_id)
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::remove_webhook_fotd_with_id(channel_id))
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::remove_webhook_fotd_with_id(channel_id))
    }
}

pub fn add_reminder(reminder: &Reminder) -> Result<(), Err> {
    #[cfg(feature = "mysql")]
    {
        sql::add_reminder(reminder)
    }
    #[cfg(feature = "mongodb")]
    {
        let reminder = reminder.clone();
        spawn_thread!(mongo::add_reminder(&reminder))
    }
    #[cfg(feature = "surrealdb")]
    {
        let reminder = reminder.clone();
        spawn_thread!(surreal::add_reminder(&reminder))
    }
}

pub fn remove_reminder(reminder: &Reminder) -> Result<(), Err> {
    #[cfg(feature = "mysql")]
    {
        sql::remove_reminder(reminder)
    }
    #[cfg(feature = "mongodb")]
    {
        let reminder = reminder.clone();
        spawn_thread!(mongo::remove_reminder(&reminder))
    }
    #[cfg(feature = "surrealdb")]
    {
        let reminder = reminder.clone();
        spawn_thread!(surreal::remove_reminder(&reminder))
    }
}

pub fn remove_reminder_by_id(id: u64) -> Result<(), Err> {
    #[cfg(feature = "mysql")]
    {
        sql::remove_reminder_by_id(id)
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::remove_reminder_by_id(id))
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::remove_reminder_by_id(id))
    }
}

pub fn get_all_reminders() -> Result<Vec<Reminder>, Err> {
    #[cfg(feature = "mysql")]
    {
        sql::get_all_reminders()
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::get_all_reminders())
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::get_all_reminders())
    }
}

pub fn get_reminders_of_user(user_id: UserId) -> Result<Vec<Reminder>, Err> {
    #[cfg(feature = "mysql")]
    {
        sql::get_reminders_of_user(user_id)
    }
    #[cfg(feature = "mongodb")]
    {
        spawn_thread!(mongo::get_reminders_of_user(user_id))
    }
    #[cfg(feature = "surrealdb")]
    {
        spawn_thread!(surreal::get_reminders_of_user(user_id))
    }
}

pub fn get_action_count(
    action: &str,
    from_user_id: UserId,
    to_user_id: UserId,
) -> Result<i32, Err> {
    #[cfg(feature = "mysql")]
    {
        sql::get_action_count(action, from_user_id, to_user_id)
    }
    #[cfg(feature = "mongodb")]
    {
        let action = action.to_string();
        spawn_thread!(mongo::get_action_count(&action, from_user_id, to_user_id))
    }
    #[cfg(feature = "surrealdb")]
    {
        let action = action.to_string();
        spawn_thread!(surreal::get_action_count(&action, from_user_id, to_user_id))
    }
}

pub fn update_action_count(
    action: &str,
    from_user_id: UserId,
    to_user_id: UserId,
    amount: i32,
) -> Result<(), Err> {
    #[cfg(feature = "mysql")]
    {
        sql::update_action_count(action, from_user_id, to_user_id, amount)
    }
    #[cfg(feature = "mongodb")]
    {
        let action = action.to_string();
        spawn_thread!(mongo::update_action_count(
            &action,
            from_user_id,
            to_user_id,
            amount
        ))
    }
    #[cfg(feature = "surrealdb")]
    {
        let action = action.to_string();
        spawn_thread!(surreal::update_action_count(
            &action,
            from_user_id,
            to_user_id,
            amount
        ))
    }
}

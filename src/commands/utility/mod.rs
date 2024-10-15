use std::collections::HashMap;

use once_cell::sync::OnceCell;
use poise::serenity_prelude::UserId;

use self::add_reminder::Reminder;

pub mod add_reminder;
pub mod poll;
pub mod remove_reminder;

static mut REMINDER_CACHE: OnceCell<HashMap<UserId, Vec<Reminder>>> = OnceCell::new();
static mut REMINDER_CACHE_INITIALIZED: bool = false;

#[inline]
pub fn init_reminder_cache() {
    unsafe {
        REMINDER_CACHE.get_or_init(HashMap::new);
        REMINDER_CACHE_INITIALIZED = true;
    }
}

#[inline]
pub fn get_cache() -> &'static mut HashMap<UserId, Vec<Reminder>> {
    if !unsafe { REMINDER_CACHE_INITIALIZED } {
        init_reminder_cache();
    }

    unsafe {
        REMINDER_CACHE
            .get_mut()
            .expect("Failed to get reminder cache")
    }
}

#[must_use]
pub fn get_reminders_of_user(user_id: UserId) -> Option<&'static Vec<Reminder>> {
    update_cache();

    let res = get_cache().get(&user_id);

    res
}

pub fn add_reminder_to_cache(reminder: &Reminder, should_update_cache: bool) {
    let reminders = get_cache();

    if let Some(user_reminders) = reminders.get_mut(&reminder.user_id) {
        // Check if the reminder already exists for this user
        if !user_reminders.iter().any(|r| r == reminder) {
            user_reminders.push(reminder.clone());
        }
    } else {
        reminders.insert(reminder.user_id, vec![reminder.clone()]);
    }

    if should_update_cache {
        update_cache();
    }
}

pub fn remove_reminder_from_cache(reminder: &Reminder) {
    let reminders = get_cache();

    if let Some(user_reminders) = reminders.get_mut(&reminder.user_id) {
        user_reminders.retain(|r| r != reminder);
    }
}

pub fn remove_reminder_from_cache_by_id(id: u64) {
    let reminders = get_cache();

    for user_reminders in reminders.values_mut() {
        user_reminders.retain(|r| r.id != id);
    }
}

pub fn add_reminders_to_cache(reminders: Vec<Reminder>) {
    for reminder in reminders {
        add_reminder_to_cache(&reminder, false);
    }
    update_cache();
}

/// Checks for expired reminders and removes them.
pub fn update_cache() {
    let reminders = get_cache();

    for user_reminders in reminders.values_mut() {
        user_reminders.retain(|r| {
            r.timestamp
                > std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Failed to get time since epoch")
                    .as_secs()
        });
    }
}

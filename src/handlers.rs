use core::panic;
use std::sync::Arc;

use crate::{commands::fun::tod, configuration::ACTIVITIES, database, Error};
use log::{error, info, warn};
use poise::serenity_prelude::{self as serenity, ActivityData as Data};
use rand::seq::SliceRandom;

/// The amount of commands that the bot has executed.
static mut COMMAND_COUNT: i64 = 0;

pub async fn on_ready(
    ctx: &serenity::Context,
    ready: &serenity::Ready,
    framework: &poise::Framework<(), Error>,
) -> Result<(), Error> {
    let servers = ready.guilds.len();

    let arc = Arc::new(ctx.clone());

    std::thread::spawn(move || change_activities(&arc, servers));

    // We register Global commands, global commands can take some time to update on all servers the bot is active in.
    //
    // Global commands are available in every server, including DM's.
    let builder = poise::builtins::create_application_commands(&framework.options().commands);

    let g_commands = serenity::Command::set_global_commands(&ctx.http, builder).await;

    match g_commands {
        Ok(_c) => {
            // dbg!("I now have the following guild slash commands: \n{:#?}", _c);
        }
        Err(e) => {
            error!("Failed to register global commands: {:?}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

fn change_activities(ctx: &Arc<serenity::Context>, servers: usize) {
    loop {
        let mut rand = rand::thread_rng();
        // expect is safe to use since ACTIVITIES will never be
        // empty.
        let activity = ACTIVITIES.choose(&mut rand).expect("unreachable");

        let activity_type = activity.0;
        let name = activity.1;
        let name = name.replace('@', &servers.to_string());
        let act = Data {
            name,
            kind: activity_type,
            url: None,
            state: None,
        };

        ctx.set_activity(Some(act));

        std::thread::sleep(std::time::Duration::from_secs(120));
    }
}

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _: poise::FrameworkContext<'_, (), Error>,
    (): &(),
) -> Result<(), Error> {
    #[allow(clippy::single_match)]
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            info!("{} is ready!", data_about_bot.user.name);
        }
        serenity::FullEvent::InteractionCreate { interaction } => {
            if let Some(interaction_data) = interaction.as_message_component() {
                match interaction_data.data.custom_id.as_str() {
                    "tod_truth" | "tod_dare" | "tod_random" => {
                        tod::receive_interaction(ctx, interaction_data).await;
                    }
                    _ => {
                        warn!(
                            "Unhandled interaction: {:?}",
                            interaction_data.data.custom_id
                        );
                    }
                }
            }
        }

        _ => {}
    }

    Ok(())
}

pub async fn error_handler(error: poise::FrameworkError<'_, (), Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            error!("Error in command `{}`: {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error in on_error: {}", e);
            }
        }
    }
}

pub async fn pre_command(_: poise::Context<'_, (), Error>) {
    unsafe {
        if COMMAND_COUNT == 0 {
            COMMAND_COUNT = get_command_count();
        } else {
            let result = std::thread::spawn(|| database::update_command_count(COMMAND_COUNT))
                .join()
                // .join() docs:
                // If the associated thread panics, [Err] is returned with the parameter given to panic.
                //
                // `update_command_count`'s chance of panicking is acceptably low, so we can just .expect()
                .expect("Unreachable?");

            if result.is_err() {
                error!("Failed to get command count: {:?}", result);
                // just do nothing at all if it fails
                return;
            }

            COMMAND_COUNT += 1;
        }
    }
}

pub fn get_command_count() -> i64 {
    unsafe {
        if COMMAND_COUNT == 0 {
            let result = std::thread::spawn(database::get_command_count)
                .join()
                // .join() docs:
                // If the associated thread panics, [Err] is returned with the parameter given to panic.
                //
                // `update_command_count`'s chance of panicking is acceptably low, so we can just .expect()
                .expect("Unreachable?");

            if result.is_err() {
                error!("Failed to get command count: {:?}", result);
                return 0;
            }

            // `result` is obligatorily `Ok`, we can .expect()
            result.expect("Unreachable")
        } else {
            COMMAND_COUNT
        }
    }
}

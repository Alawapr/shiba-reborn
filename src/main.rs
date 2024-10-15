#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_panics_doc)]

pub mod commands;
mod configuration;
mod database;
mod environment;
mod handlers;
mod logger;
mod utils;

use commands::CommandList;
use database::initialize_db;
use handlers::{error_handler, event_handler, on_ready, pre_command};
use logger::Logger;
use utils::{get_os_string, get_version_string};

use log::{error, info};
use once_cell::sync::OnceCell;
use poise::serenity_prelude::{self as serenity};

use core::panic;
use std::env::{self, var};
use std::time::SystemTime;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub static mut OS: OnceCell<String> = OnceCell::new();
pub static mut VERSION: OnceCell<String> = OnceCell::new();

pub static mut STARTUP_TIME: Option<SystemTime> = None;
pub static mut VOLATILE_TIME: Option<SystemTime> = None;

pub static mut ACTION_COUNTS: Option<self::commands::fun::action::Counts> = None;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    env::set_var("RUST_BACKTRACE", "1");

    /*
    Parse all of the environment variables from the .env file
    to have them ready to be used.

    The bot currently depends on the following variables:
    - PROD_TOKEN
    - DEV_TOKEN
    - SQL_HOST
    - SQL_PORT
    - SQL_USER
    - SQL_PASSWORD
    - SQL_DATABASE
    - TENOR_API_KEY

    Others it can accept but doesn't depend on:
    - RUST_LOG
    */
    environment::parse();

    Logger::init().unwrap_or_else(|e| {
        panic!("Logger failed to initialize: {}", e);
    });

    info!("Parsed all enviroment variables and initialized logger");

    unsafe {
        STARTUP_TIME = Some(SystemTime::now());
        VOLATILE_TIME = Some(SystemTime::now());
        OS.get_or_init(get_os_string);
        VERSION.get_or_init(get_version_string);
    }

    // Initialize the global database connection to use it later on.
    initialize_db()
        .map_err(|e| error!("{}", e))
        .expect("Failed to initialize database");

    info!("Initialized the database connection");

    std::thread::spawn(|| {
        let reminders = database::get_all_reminders().expect("Failed to get all reminders from DB");
        commands::utility::add_reminders_to_cache(reminders.clone());

        for reminder in reminders {
            commands::utility::add_reminder::add_reminder_thread(reminder);
        }

        info!("Added all reminders to cache and sent them to the threads");
    });

    std::thread::spawn(|| {
        let result = tokio::runtime::Runtime::new()
            .expect("Failed to create tokio runtime")
            .block_on(commands::fun::qotd::run_loop());

        if result.is_err() {
            error!("Failed to run QOTD loop: {result:?}");
        }
    });

    std::thread::spawn(|| {
        let result = tokio::runtime::Runtime::new()
            .expect("Failed to create tokio runtime")
            .block_on(commands::fun::fotd::run_loop());

        if result.is_err() {
            error!("Failed to run FOTD loop: {result:?}");
        }
    });

    let result = commands::fun::action::initialize_action_count()
        .await
        .map_err(|e| error!("{}", e))
        .expect("Failed to initialize action count");

    unsafe {
        ACTION_COUNTS = Some(result);
    }

    #[cfg(feature = "prod")]
    let discord_token = var("PROD_TOKEN").expect("You must provide a production token");

    #[cfg(not(feature = "prod"))]
    let discord_token = var("DEV_TOKEN").expect("You must provide a developer token");

    info!(
        "Discord token initialized, Production?: {}",
        cfg!(feature = "prod")
    );

    let command_list = CommandList::new();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: command_list.get(),
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            on_error: |err| Box::pin(error_handler(err)),
            pre_command: |ctx| Box::pin(pre_command(ctx)),
            ..Default::default()
        })
        .setup(|ctx, ready, framework| Box::pin(on_ready(ctx, ready, framework)))
        .build();

    #[cfg(debug_assertions)]
    info!("Starting bot in debug mode...");

    #[cfg(not(debug_assertions))]
    info!("Starting bot in release mode...");

    let mut client =
        match serenity::ClientBuilder::new(discord_token, serenity::GatewayIntents::empty())
            .framework(framework)
            .await
        {
            Ok(client) => client,
            Err(error) => {
                error!("Error creating client: {:?}", error);
                std::process::exit(1);
            }
        };

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

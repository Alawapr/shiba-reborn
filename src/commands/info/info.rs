use std::vec;

use poise::{
    serenity_prelude::{CreateEmbed, CreateEmbedFooter, GuildId},
    CreateReply,
};

use super::super::{EMBED_COLOR, RUST_LOGO_URL};
use crate::{handlers::get_command_count, Error, OS, STARTUP_TIME, VERSION};

// it's more than readable <3
#[allow(clippy::unreadable_literal)]
const DEV_SERVER_ID: GuildId = GuildId::new(1161012273935036528);

#[inline]
async fn get_shard_latency(
    ctx: &poise::Context<'_, (), Error>,
) -> Result<std::time::Duration, Error> {
    let shard_id = ctx.serenity_context().shard_id;
    let manager = ctx.framework().shard_manager();

    let runners = &manager.runners;
    let runners_lock = runners.lock().await;

    runners_lock.get(&shard_id).map_or_else(
        || Err(Error::from("Shard not found")),
        |shard_runner_info| {
            shard_runner_info
                .latency
                .map_or_else(|| Ok(std::time::Duration::from_millis(0)), Ok)
        },
    )
}

/// Display information about Shiba
#[poise::command(slash_command, broadcast_typing)]
pub async fn info(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    ctx.defer().await?;

    let latency = get_shard_latency(&ctx).await?;

    let startup_time = unsafe { STARTUP_TIME.expect("Startup time not set") };

    let unix = startup_time
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let formatted_uptime = format!("<t:{unix}:R>");

    let command_count = get_command_count();
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("Shiba Stats:")
                .description("Hosted in USA :flag_us: made in :flag_es: & :flag_be:")
                // \n\n[Make sure to give us a star in GitHub!](https://github.com/saturndev/shiba_reborn)
                // Removed this for now as the bot wont be going (fully) open source
                .color(EMBED_COLOR)
                .field("Developers", "`alawapr.rs`\n`caturndev`", true)
                .field("Uptime", formatted_uptime, true)
                .field("Latency", format!("`{}ms`", latency.as_millis()), true)
                // .unwrap() is perfectly fine here since `VERSION` is never empty
                .field(
                    "Bot version",
                    format!("`{}`", unsafe { VERSION.get().expect("Unreachable") }),
                    true,
                )
                .field("Commands ran", format!("`{command_count}`"), true)
                // .unwrap() is perfectly fine here since `OS` is never empty
                .field(
                    "OS",
                    format!("`{}`", unsafe { OS.get().expect("Unreachable") }),
                    true,
                )
                // .field("Contributors", "[Click here to view the list of contributors.](https://github.com/saturndev/shiba_reborn/graphs/contributors)", false)
                // Removed this for now as the bot wont be going (fully) open source
                .footer(
                    CreateEmbedFooter::new("Made with Rust using Serenity and Poise.")
                        .icon_url(RUST_LOGO_URL),
                ),
        ),
    )
    .await?;

    let guild_id = ctx.guild_id().unwrap_or(GuildId::new(1));
    if guild_id == DEV_SERVER_ID {
        let mut system = sysinfo::System::new();
        system.refresh_all();
        std::thread::sleep(std::time::Duration::from_millis(200));
        system.refresh_cpu_usage();

        // We're fine if it's not precise.
        #[allow(clippy::cast_precision_loss)]
        let bytes_to_gb = |bytes: u64| -> f64 { bytes as f64 / 1024.0 / 1024.0 / 1024.0 };
        macro_rules! compute_memory {
            ($name: ident) => {
                let $name = format!("{:.1} GB", bytes_to_gb(system.$name()));
            };
        }

        compute_memory!(total_memory);
        compute_memory!(available_memory);
        compute_memory!(used_memory);

        let disks = sysinfo::Disks::new_with_refreshed_list();
        let total_space = disks
            .iter()
            .map(|d| bytes_to_gb(d.total_space()))
            .sum::<f64>();
        let used_space = disks
            .iter()
            .map(|d| bytes_to_gb(d.total_space() - d.available_space()))
            .sum::<f64>();
        let free_space = disks
            .iter()
            .map(|d| bytes_to_gb(d.available_space()))
            .sum::<f64>();

        let disk_usage = if total_space == 0.0 {
            0.0
        } else {
            (used_space / total_space) * 100.0
        };

        let networks = sysinfo::Networks::new_with_refreshed_list();
        let packets_sent = networks
            .iter()
            .map(|n| n.1.total_packets_transmitted())
            .sum::<u64>();
        let packets_received = networks
            .iter()
            .map(|n| n.1.total_packets_received())
            .sum::<u64>();
        let bytes_sent = networks
            .iter()
            .map(|n| n.1.total_transmitted())
            .sum::<u64>();
        let bytes_received = networks.iter().map(|n| n.1.total_received()).sum::<u64>();

        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::default()
                    .title("[DEV] Host System Information:")
                    .field("", "**CPU**", false)
                    .field(
                        "CPU Usage",
                        format!("{:.2}%", system.global_cpu_info().cpu_usage()),
                        true,
                    )
                    .field(
                        "Logical CPU Count",
                        format!("{}", system.physical_core_count().unwrap_or(0)),
                        true,
                    )
                    .field("", "**Memory**", false)
                    .field("Total Memory", total_memory, true)
                    .field("Available Memory", available_memory, true)
                    .field("Memory Usage", used_memory, true)
                    .field("", "**Disk**", false)
                    .field("Total Space", format!("{total_space:.2} GB"), true)
                    .field("Used Space", format!("{used_space:.2} GB"), true)
                    .field("Free Space", format!("{free_space:.2} GB"), true)
                    .field("Disk Usage", format!("{disk_usage:.2} %"), true)
                    .field("", "**Network**", false)
                    .field("Packets Sent", format!("{packets_sent}"), true)
                    .field("Packets Received", format!("{packets_received}"), true)
                    .field("Bytes Sent", format!("{bytes_sent} B"), true)
                    .field("Bytes Received", format!("{bytes_received} B"), true)
                    .color(EMBED_COLOR),
            ),
        )
        .await?;
    }

    Ok(())
}

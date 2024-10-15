use crate::Error;
use poise::serenity_prelude::Colour;
use poise::Command;

pub mod fun;
pub mod info;
pub mod moderation;
pub mod utility;
mod time;

// They're color codes, not unreadable rust!!!
#[allow(clippy::unreadable_literal)]
pub const EMBED_COLOR: Colour = Colour(0xc5b875);
#[allow(clippy::unreadable_literal)]
pub const ERROR_EMBED_COLOR: Colour = Colour(0xf53f3c);
#[allow(clippy::unreadable_literal)]
pub const CONFIRM_EMBED_COLOR: Colour = Colour(0x5aeb46);
pub const ERROR_EMOJI: &str = "<:shibaCross:1197544989475995739>";
pub const CONFIRM_EMOJI: &str = "<:shibaCheck:1197545020329300049>";
pub const SHIBA_MAIN_IMAGE_URL: &str = "https://i.imgur.com/uau9dGl.jpg";
pub const DOWNSCALED_SHIBA_MAIN_IMAGE_URL: &str = "https://i.imgur.com/bhKokf9.jpg";
pub const RUST_LOGO_URL: &str = "https://i.imgur.com/XGYtTmW.png";
pub const API_THANKS_EMOJI: &str = "https://i.imgur.com/lvQs0Nq.png";
pub const WARNING_EMOJI: &str = "https://i.imgur.com/5pt3keW.png";
// TODO: Make the invite link only require needed permissions
pub const INVITE_LINK: &str = "https://discord.com/api/oauth2/authorize?client_id=1195786642150137946&permissions=8&scope=applications.commands+bot";
pub const SUPPORT_SERVER: &str = "https://discord.gg/u26fgfU5f9";
pub const LIGHTBULB_ICON: &str = "https://i.imgur.com/p3IjQky.png";

/// A list of commands usable in Shiba.
#[derive(Default)]
pub struct CommandList(pub Vec<Command<(), Error>>);

impl CommandList {
    #[rustfmt::skip]
    #[must_use]    
    pub fn new() -> Self {
        use crate::commands::{fun, info, moderation, utility};

        let commands = vec![
            // Fun Commands
            fun::magik::magik(),
            fun::coinflip::coinflip(),
            fun::animal::animal(),
            fun::owoify::owoify(),
            fun::skin::skin(),
            fun::quote::quote(),
            fun::qotd::qotd(),
            fun::fotd::fotd(),
            fun::action::action(),
            fun::wyr::would_you_rather(),
            fun::tod::truth_or_dare(),

            // Info Commands
            info::invite::invite(),
            info::info::info(),
            info::serverinfo::serverinfo(),
            info::userinfo::userinfo(),
            info::suggest::suggest(),

            // Moderation Commands
            moderation::purge::purge(),

            // Utility Commands
            utility::poll::poll(),
            utility::add_reminder::add_reminder(),
            utility::remove_reminder::remove_reminder(),
        ];

        Self(commands)
    }

    /// Get the list of commands.
    #[must_use]
    #[allow(clippy::inline_always)]
    #[inline(always)]
    pub fn get(self) -> Vec<Command<(), Error>> {
        self.0
    }
}

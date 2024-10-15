use std::vec;

use crate::commands::{
    CONFIRM_EMBED_COLOR, CONFIRM_EMOJI, DOWNSCALED_SHIBA_MAIN_IMAGE_URL, ERROR_EMBED_COLOR,
    ERROR_EMOJI,
};
use crate::{database, Error};

use super::super::EMBED_COLOR;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::Timelike;
use log::info;
use poise::serenity_prelude::Permissions;
use poise::serenity_prelude::{json::json, CreateEmbed};
use poise::CreateReply;
use rand::seq::SliceRandom;
use webhook::client::WebhookClient;

/// Schedule a question to be sent to the desired channel every day
#[poise::command(slash_command, broadcast_typing)]
pub async fn qotd(
    ctx: poise::Context<'_, (), Error>,
    #[description = "Your wanted option."]
    #[autocomplete = "qotd_autocomplete"]
    option: Option<String>,
    #[description = "The channel to send the question to."] channel: Option<
        poise::serenity_prelude::Channel,
    >,
) -> Result<(), Error> {
    ctx.defer().await?;

    if option.is_none() {
        // It's safe to use .expect() since `QUESTIONS_OF_THE_DAY` will never be empty.
        // .choose() will only return `None` if it is empty.
        let qotd = QUESTIONS_OF_THE_DAY
            .choose(&mut rand::thread_rng())
            .expect("Unreachable");

        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Here's your question of the day:")
                    .description((*qotd).to_string())
                    .color(EMBED_COLOR),
            ),
        )
        .await?;
        return Ok(());
    }

    let perms = ctx
        .author_member()
        .await
        .expect("Couldn't get member info")
        .permissions
        .expect("Couldn't get member permissions")
        .contains(Permissions::MANAGE_GUILD);
    if !perms {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Error")
                    .description(format!(
                        "{ERROR_EMOJI} You don't have permission to use this command.\nPermission needed: `MANAGE_GUILD`.",
                    ))
                    .color(ERROR_EMBED_COLOR),
            ),
        )
        .await?;
        return Ok(());
    }

    // It's safe to use .expect() because we already checked if the option is `None` above.
    let option = option.expect("Unreachable");

    if channel.is_none() {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Error")
                    .description(format!(
                        "{ERROR_EMOJI} Incorrect usage of command - please use `/qotd <option> <channel>`.\nOr get a question of the day by using `/qotd`.",
                    ))
                    .color(ERROR_EMBED_COLOR),
            ),
        )
        .await?;
        return Ok(());
    }

    // It's safe to use .expect() because we already checked if the channel is `None` above.
    let channel = channel.expect("Unreachable");

    if channel.clone().category().is_some() {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::default()
                    .title("Error")
                    .description(format!(
                        "{ERROR_EMOJI} Please select a text channel, not a category.",
                    ))
                    .color(ERROR_EMBED_COLOR),
            ),
        )
        .await?;
        return Ok(());
    }

    match option.as_str() {
        "add" => {
            if database::channel_id_exists_qotd(channel.id().get())? {
                ctx.send(
                    CreateReply::default().embed(
                        CreateEmbed::default()
                            .title("Error")
                            .description(format!(
                                "{ERROR_EMOJI} This channel is already scheduled to get a question of the day.",
                            ))
                            .color(ERROR_EMBED_COLOR),
                    ),
                )
                .await?;
                return Ok(());
            }
            let avatar_bytes: Vec<u8> = reqwest::get(DOWNSCALED_SHIBA_MAIN_IMAGE_URL)
                .await?
                .bytes()
                .await?
                .into();
            let b64_avatar = STANDARD.encode(avatar_bytes);

            let map = json!({
                "avatar": format!("data:image/jpeg;base64,{}", b64_avatar),
                "name": "Shiba QOTD"
            }
            );

            let webhook = ctx.http().create_webhook(channel.id(), &map, None).await?;
            database::add_webhook_qotd(webhook.url()?, channel.id().get())?;

            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Question of the day added")
                        .description(format!(
                            "{} Question of the day scheduled to appear in <#{}>. A question is going to be sent right away as well.",
                            CONFIRM_EMOJI, channel.id().get()
                        ))
                        .color(CONFIRM_EMBED_COLOR),
                ),
            )
            .await?;

            // It's safe to use .expect() since `QUESTIONS_OF_THE_DAY` will never be empty.
            // .choose() will only return `None` if it is empty.
            let qotd = QUESTIONS_OF_THE_DAY
                .choose(&mut rand::thread_rng())
                .expect("Unreachable");

            let webhook_url = database::get_webhook_by_id_qotd(channel.id().get())?;
            let webhook = WebhookClient::new(&webhook_url);
            webhook
                .send(|m| {
                    m.avatar_url(DOWNSCALED_SHIBA_MAIN_IMAGE_URL)
                        .username("Shiba QOTD")
                        .embed(|e| {
                            e.title("Question of the day is here!")
                                .field("", qotd, true)
                                .color(&EMBED_COLOR.0.to_string())
                        })
                })
                .await?;
        }

        "remove" => {
            if database::channel_id_exists_qotd(channel.id().get())? {
                database::remove_webhook_qotd_with_id(channel.id().get())?;

                ctx.send(
                    CreateReply::default().embed(
                        CreateEmbed::default()
                            .title("Question of the day removed")
                            .description(format!(
                                "{CONFIRM_EMOJI} This channel will no longer receive daily questions of the day.",
                            ))
                            .color(CONFIRM_EMBED_COLOR),
                    ),
                )
                .await?;
            } else {
                ctx.send(
                    CreateReply::default().embed(
                        CreateEmbed::default()
                            .title("Error")
                            .description(format!(
                                "{ERROR_EMOJI} This channel was never scheduled to get questions of the day.",
                            ))
                            .color(ERROR_EMBED_COLOR),
                    ),
                )
                .await?;
            }
        }

        _ => {
            ctx.send(
                CreateReply::default().embed(
                    CreateEmbed::default()
                        .title("Error")
                        .description(format!(
                            "{ERROR_EMOJI} Incorrect usage of command - please use `/qotd <option> <channel>`.\nOr get a question of the day by using `/qotd`.",
                        ))
                        .color(ERROR_EMBED_COLOR),
                ),
            )
            .await?;
        }
    }
    Ok(())
}

// This function needs to be async for poise to use it.
#[allow(clippy::unused_async)]
async fn qotd_autocomplete(_: poise::Context<'_, (), Error>, _partial: &str) -> Vec<String> {
    // let options = ["add".to_string(), "remove".to_string()];

    // let partial: String = partial.to_lowercase();
    // let options = options
    //     .iter()
    //     .filter(move |action| action.starts_with(&partial))
    //     .collect::<Vec<_>>();

    // options.iter().map(|&s| (*s).to_string()).collect()
    vec!["add".to_string(), "remove".to_string()]
}

#[derive(serde::Deserialize)]
#[allow(unused)]
struct InvalidWebhookResponse {
    message: String,
    code: u16,
}

pub(crate) async fn run_loop() -> Result<(), crate::Error> {
    loop {
        let timestamp = chrono::Local::now();
        if timestamp.hour() == 12 && timestamp.minute() == 0 {
            let mut webhooks = database::get_all_webhooks_qotd()?;
            let mut invalid_webhooks = Vec::new();

            for webhook_url in &webhooks {
                let response: Result<InvalidWebhookResponse, reqwest::Error> =
                    reqwest::get(webhook_url).await?.json().await;

                if response.is_ok() {
                    info!("Removing invalid webhook (qotd): {}", webhook_url);
                    database::remove_webhook_qotd(webhook_url)?;
                    invalid_webhooks.push(webhook_url.to_string());
                }
            }

            for webhook_url in &mut invalid_webhooks {
                webhooks.retain(|x| x != webhook_url);
            }

            for webhook_url in webhooks {
                // It's safe to use .expect() since `QUESTIONS_OF_THE_DAY` will never be empty.
                // .choose() will only return `None` if it is empty.
                let qotd = QUESTIONS_OF_THE_DAY
                    .choose(&mut rand::thread_rng())
                    .expect("Unreachable");

                let webhook = WebhookClient::new(&webhook_url);

                webhook
                    .send(|m| {
                        m.avatar_url(DOWNSCALED_SHIBA_MAIN_IMAGE_URL)
                            .username("Shiba QOTD")
                            .embed(|e| {
                                e.title("Question of the day is here!")
                                    .field("", qotd, true)
                                    .color(&EMBED_COLOR.0.to_string())
                            })
                    })
                    .await?;
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}

pub const QUESTIONS_OF_THE_DAY: [&str; 332] = [
    "What is your favorite way to stay connected with friends or family who live far away?",
    "What is a skill you think will be important in the future?",
    "What is a piece of advice you'd give to your past self?",
    "If you could have a conversation with any character from a contemporary novel, who would it be?",
    "If you could master any form of physical performance, which one would it be?",
    "What is your biggest goal for the next year?",
    "What is your favorite type of cuisine?",
    "What is the most challenging thing you've ever done?",
    "If you could master any form of dance, which one would it be?",
    "If you could have dinner with any scientist, who would it be?",
    "Suppose you could attend a dinner party with characters from different fictional universes. Which three characters would you invite, and what would the theme of the dinner be?",
    "If you could possess any futuristic medical technology, what would it be used for?",
    "What is a cause or initiative you've actively supported or volunteered for?",
    "What is a skill you believe will be crucial in the future job market?",
    "If you could have any animal's ability to communicate with other species, which would you choose?",
    "If you could have a conversation with any fictional deity, who would it be?",
    "What is a movie you can watch over and over again?",
    "What is a movie that always makes you laugh?",
    "If you could have any animal's ability to adapt, which would you choose?",
    "What is a movie or TV show you love but wouldn't admit to in a casual conversation?",
    "If you could have a conversation with any fictional character, who would it be?",
    "What is a hobby you've considered taking up but haven't yet?",
    "What is your preferred method for learning a new skill?",
    "What is a book that made you question your own beliefs or perspectives?",
    "If you could master any form of storytelling, which one would it be?",
    "What is a cuisine you've never tried but would like to?",
    "What is a skill you've learned from a random online tutorial that surprised you?",
    "If you could have dinner with any fictional villain, who would it be?",
    "If you could have any view from your window, what would it be?",
    "If you could visit any fictional world for a day, where would you go?",
    "If you could possess any artistic talent, what would you create?",
    "What is a quote that inspires you?",
    "If you could visit any ancient civilization, where would you go?",
    "If you could have a conversation with any literary character, who would it be?",
    "What is your favorite outdoor activity?",
    "If you could have any superpower but only use it for good, what would it be?",
    "What is your favorite method for unwinding after a busy day?",
    "If you could have any animal's strength, which would you choose?",
    "What is a language you find beautiful, even if you don't understand it?",
    "What is your favorite way to celebrate personal achievements?",
    "What is a hobby you've tried and didn't enjoy, but you're glad you gave it a shot?",
    "What is a place you've always wanted to visit but haven't?",
    "What is a hobby you enjoy that others might find surprising?",
    "What is a piece of technology you believe has the potential to change the world?",
    "If you could learn any form of artistic expression, which one would it be?",
    "What is your favorite way to spend a rainy day?",
    "If you could have any technology from a futuristic sci-fi movie, what would it be?",
    "If you could have any job in the field of medicine, what would it be?",
    "If you could have dinner with any musician from the 1960s, who would it be?",
    "If you could have any historical object, what would it be?",
    "What is a skill you've developed that has unexpectedly helped you in various aspects of life?",
    "What is your favorite way to stay organized in your daily life?",
    "If you could have any job for a day, what would it be?",
    "If you could have any job in the field of education, what would it be?",
    "If you could have any technology from science fiction, what would it be?",
    "If you could have dinner with any actor or actress, who would it be?",
    "What is your favorite holiday and why?",
    "What is a book that made you think deeply?",
    "What is a place you'd like to visit that's known for its natural beauty?",
    "What is a skill you've acquired through self-teaching?",
    "If you could have any fictional device from literature, what would it do?",
    "What is the last movie or TV show you watched and enjoyed?",
    "What is a place you've visited that felt like stepping into a dream or fantasy?",
    "What is your favorite type of music?",
    "If you could have any animal's ability to regenerate, which would you choose?",
    "What is your favorite way to unwind and relax?",
    "What is your favorite way to practice mindfulness?",
    "What is a skill you think will be valuable in the future job market?",
    "What is your favorite way to give back to the community?",
    "If you could have any animal's speed, which would you choose?",
    "What is your favorite memory from childhood?",
    "If you could have any animal's communication abilities, which would you choose?",
    "If you could have any superpower, what would it be?",
    "If you could have any meal right now, what would it be?",
    "What is a skill you've acquired through a challenging life experience?",
    "If you could have a conversation with any animated character, who would it be?",
    "If you could have any animal's ability to sense impending weather changes, which would you choose?",
    "What is the best piece of advice you've ever received?",
    "What is a personal accomplishment you're proud of but rarely talk about?",
    "What is your favorite way to stay motivated?",
    "If you could have any animal's intelligence, which would you choose?",
    "If you could have dinner with any historical figure, who would it be?",
    "If you could meet any historical figure, who would it be and why?",
    "What is a place you'd like to visit solely for its cultural heritage?",
    "If you could have any job for a year, what would it be?",
    "If you could have any animal's agility, which would you choose?",
    "If you could have any fictional tool, what would it be used for?",
    "If you could have dinner with any character from a classic novel, who would it be?",
    "If you could have any fictional technology, what would it be?",
    "What is your favorite way to stay connected with friends and family?",
    "What is a language you find intriguing and would like to learn?",
    "What is a unique cultural festival you'd like to attend?",
    "What is a technology trend you find intriguing and want to explore?",
    "What is a childhood game you still enjoy playing?",
    "If you could have any animal's ability to see in the dark, which would you choose?",
    "What is a skill you've improved upon through practice?",
    "What is a book or movie that has had a significant impact on you?",
    "What is a cause or social issue you're passionate about supporting?",
    "What is a place you've always wanted to explore?",
    "If you could change one thing about the world, what would it be?",
    "What is a place you'd like to visit that's off the beaten path?",
    "What is a movie that left you in awe?",
    "If you could have any skill instantly without effort, what would it be?",
    "If you could have any fictional mentor, who would it be?",
    "What is a place you've dreamed of visiting since childhood, and have you been there yet?",
    "If you could have any job in the world, what would it be?",
    "If you could have any dessert right now, what would it be?",
    "If you could visit any historical period, when and where would you go?",
    "If you could have dinner with any fictional robot or AI, who would it be?",
    "If you could have dinner with any actor or actress from classic Hollywood, who would it be?",
    "What is a place that holds sentimental value for you?",
    "If you could have any animal's abilities, which would you choose?",
    "What is your favorite way to spend a summer day?",
    "If you could have dinner with any musician from the 1980s, who would it be?",
    "What is a place you've discovered that feels like a hidden paradise on Earth?",
    "What is a place you've visited that made you feel truly alive?",
    "What is your favorite way to spend time in nature, away from technology?",
    "If you could have any historical document, what would it be?",
    "If you could have any historical figure as a workout buddy, who would it be?",
    "If you could possess any magical ability, what would it be used for?",
    "What is your favorite way to explore a new city?",
    "What is your all-time favorite movie and why?",
    "What is your favorite season and why?",
    "What is your favorite way to stay positive during challenging times?",
    "If you could have any fictional gadget, what would it do?",
    "What is your favorite way to express creativity?",
    "What is your favorite type of weather and why?",
    "If you could travel to any fictional world, where would you go?",
    "If you could have dinner with any historical explorer, who would it be?",
    "What is a place you've visited that left a lasting impression on you?",
    "What is the most interesting fact you know?",
    "If you could have any meal from any cuisine right now, what would it be?",
    "What is your favorite childhood game?",
    "If you could time travel, would you go to the past or the future?",
    "If you could have dinner with any celebrity, who would it be?",
    "What is a book that changed your perspective on life?",
    "What is a skill you've improved upon recently?",
    "What is a goal you've set for yourself in the next week?",
    "What is a cause or issue you think future generations will prioritize?",
    "What is a hobby you've pursued that has provided unexpected mental or emotional benefits?",
    "If you could be any fictional character, who would you choose?",
    "If you could have any mythical creature as a friend, what would it be?",
    "What is a skill you've developed through your work or career?",
    "If you could bring to life any fictional technology from a video game, what would it be used for?",
    "What is your favorite method for staying mentally sharp and focused?",
    "What is your favorite way to celebrate a milestone?",
    "If you could have the ability to speak and understand any language instantly, which one would it be?",
    "If you could have any historical artifact, what would it be?",
    "What is a place you've discovered that feels like your own hidden gem?",
    "What is your favorite thing about yourself?",
    "What is your favorite way to relax and disconnect from the digital world?",
    "What is a book that you recommend to everyone?",
    "What is a goal you're currently working towards?",
    "What is a book you've read recently that you enjoyed?",
    "If you could attend any fictional event (from a book, movie, etc.), what would it be?",
    "What is a skill you think everyone should have?",
    "If you could be a character in a video game, who would you be?",
    "What is your favorite way to stay active?",
    "What is a place you'd like to visit that's known for its architecture?",
    "If you could witness any scientific breakthrough, what would it be?",
    "What is a place you've visited that exceeded your expectations?",
    "What is a movie or TV show that surprised you with its philosophical depth?",
    "What is your favorite way to relax after a long day?",
    "If you could witness any natural phenomenon, what would it be?",
    "If you could have any view from your window, where would it be?",
    "What is a movie that left you with mixed emotions, and why?",
    "If you could have dinner with any musician, who would it be?",
    "What is a skill you've learned from a mentor or role model in your life?",
    "If you could have any animal as a sidekick, what would it be?",
    "What is a place you'd like to travel to in the next year?",
    "If you could have any piece of art, what would it be?",
    "What is a language you find intriguing due to its unique writing system?",
    "If you could have any animal's ability to fly, which would you choose?",
    "If you could have dinner with any comedian, who would it be?",
    "If you could learn any traditional dance, which one would it be?",
    "What is your favorite form of exercise?",
    "What is your favorite way to learn new things?",
    "What is a hobby you enjoyed as a child that you still engage in today?",
    "What is a cause or charity you'd like to contribute more to in the coming years?",
    "If you could have dinner with any modern-day inventor, who would it be?",
    "If you could master any instrument, which one would it be?",
    "What is your favorite way to spend quality time with loved ones?",
    "What is a skill you've honed that has surprisingly enhanced your everyday life?",
    "What is a book that you wish had a sequel, and what would you want to happen in it?",
    "What is your favorite way to learn about history?",
    "If you could witness any historical event, what would it be?",
    "If you could have a conversation with any character from a dystopian novel, who would it be?",
    "What is a book that you've read more than once and still enjoy?",
    "If you could have any job in the world for a week, what would it be?",
    "What is a skill you've developed recently that you're proud of?",
    "If you could bring back any extinct species, which one would it be?",
    "If you could have dinner with any philosopher, who would it be?",
    "What is a dessert you've never tried but are curious about?",
    "What is your favorite way to unwind before going to bed?",
    "If you could master any form of visual art, which one would it be?",
    "What is a skill you have that not many people know about?",
    "What is your favorite type of art?",
    "If you could have any job in the creative arts, what would it be?",
    "What is a movie genre you rarely watch but enjoyed when you did?",
    "If you could have a conversation with any inventor, who would it be and why?",
    "What is the most important lesson life has taught you?",
    "If you could have dinner with any artist, who would it be?",
    "What is your favorite way to stay focused and productive?",
    "If you could have any animal's ability to navigate by the stars, which would you choose?",
    "What is your go-to comfort food?",
    "What is your favorite way to spend a fall day?",
    "If you could have dinner with any ancient poet or writer, who would it be?",
    "If you could master any ancient form of craftsmanship, what would it be?",
    "If you could have any animal's ability to camouflage, which would you choose?",
    "If you could have any job in the field of environmental science, what would it be?",
    "What is a skill you wish you had when you were younger?",
    "If you could have any meal prepared by a world-class chef, what would it be?",
    "If you could have any animal's sense of curiosity, which would you choose?",
    "If you could have dinner with any ancient monarch, who would it be?",
    "What is a song that always puts you in a good mood?",
    "If you could possess any supernatural ability from folklore, what would it be used for?",
    "If you could witness any celestial event, what would it be?",
    "What is a hobby you've recently picked up?",
    "What is your favorite way to relax and unwind?",
    "What is your favorite method for boosting your creativity?",
    "If you could have any animal's ability to sense danger, which would you choose?",
    "If you could have any fictional creature as a pet, what would it be?",
    "If you could have dinner with any mythical creature, who or what would it be?",
    "If you could bring to life any fictional technology for a day, what would it be used for?",
    "What is your favorite childhood memory of nature?",
    "If you could have dinner with any fictional family, who would it be?",
    "If you could have any talent, what would it be?",
    "If you could have any job related to space exploration, what would it be?",
    "If you could explore any uncharted territory, where would you go?",
    "If you could have dinner with any historical artist, who would it be?",
    "What is a book or movie that you enjoyed as a child and still revisit today?",
    "What is a movie you love that everyone should watch?",
    "If you could witness any historical event without altering it, which one would you choose?",
    "What is your favorite way to support local businesses?",
    "What is your favorite way to enjoy the beauty of nature?",
    "If you could have a conversation with any character from a Shakespearean play, who would it be?",
    "What is a TV show you've recently binge-watched and enjoyed?",
    "What is a cultural tradition you find fascinating and would like to experience?",
    "If you could learn any form of martial arts, which one would you choose?",
    "If you could be proficient in any language, which one would it be?",
    "What skill or hobby would you like to learn?",
    "If you could have dinner with any influential figure from the Renaissance, who would it be?",
    "If you could possess any musical talent, what instrument would you play?",
    "If you could have any animal as a pet, what would it be?",
    "If you could have any animal's senses, which would you choose?",
    "If you could possess any technological innovation from the past, what would it be used for?",
    "What is a skill you believe everyone should learn in their lifetime?",
    "What is a book that you find yourself recommending often?",
    "If you could have any fictional food, what would it be?",
    "If you could possess any magical item, what would its powers be?",
    "If you could have any technological gadget, what would it be?",
    "What is a hobby you've always wanted to pick up but haven't?",
    "What is the most interesting place you've ever visited?",
    "If you could have dinner with any historical leader, who would it be?",
    "If you could have any piece of art from history, what would it be?",
    "If you could have any job in the field of psychology, what would it be?",
    "What is your favorite way to spend a winter evening?",
    "If you could possess any artistic talent overnight, what would you wake up being able to do?",
    "If you could have dinner with any author, who would it be?",
    "What is a classic book you've never read but want to?",
    "If you could meet any living person, who would it be?",
    "If you could have dinner with any ancient philosopher, who would it be?",
    "What is a famous landmark you'd love to visit?",
    "What is a place you've visited that felt like stepping into a different time period?",
    "If you could have any animal's ability to heal quickly, which would you choose?",
    "If you could bring back a canceled or concluded TV series, which one would it be?",
    "What is your go-to strategy for overcoming challenges?",
    "If you could have a conversation with any animal, which one would it be?",
    "If you could have a conversation with any character from a science fiction novel, who would it be?",
    "What is your favorite book and why?",
    "What is your favorite board game from your childhood?",
    "What is your favorite way to spend a sunny day?",
    "What is a historical figure you believe is underrated and deserves more recognition?",
    "If you could have a conversation with any character from a children's book, who would it be?",
    "If you could have dinner with any historical figure known for their sense of humor, who would it be?",
    "If you could attend any major event in history, which one would it be?",
    "If you could have any futuristic transportation, what would it look like?",
    "If you could have dinner with any fictional character, who would it be?",
    "What is a goal you've achieved that you're proud of?",
    "If you could learn any traditional instrument, which one would it be?",
    "What is a skill you've learned from a friend or family member?",
    "What is the most beautiful place you've ever been to?",
    "If you could learn any traditional craft, which one would it be?",
    "What is a goal you've achieved that surprised those around you?",
    "What is a historical period you'd like to see portrayed in a film or TV series?",
    "If you could have any historical figure as a mentor, who would it be?",
    "What is a place you've visited that felt like a real-life fairy tale setting?",
    "If you could have any piece of technology from the future, what would it be?",
    "If you could have any fictional pet, what would it be?",
    "If you could have any animal's ability to hibernate, which would you choose?",
    "What is your favorite way to spend a winter day?",
    "What is your favorite way to engage with the local community?",
    "If you could have any animal's ability to navigate, which would you choose?",
    "If you could have any fictional weapon, what would it be used for?",
    "If you could have any fictional vehicle, what would it be?",
    "What is a cause or charity you are passionate about?",
    "If you could bring back any canceled or discontinued snack or food item, what would it be?",
    "If you could possess any historical figure's wisdom, whose would it be?",
    "What is a technology you think will become obsolete in the next decade?",
    "What is your preferred way to spend a lazy Sunday afternoon?",
    "What is your favorite quote from a movie or TV show?",
    "If you could have any car, what would it be?",
    "What is a unique cultural tradition you've experienced while traveling?",
    "What is a skill you've always wanted to develop?",
    "If you could have any plant in your garden, what would it be?",
    "What is your favorite way to practice self-care?",
    "What is a skill you admire in others?",
    "What is a goal you've set for yourself in the next month?",
    "What is a quote from a book that has stuck with you over the years?",
    "What is your favorite board game or card game?",
    "If you could learn any dance style, which one would it be?",
    "If you could have any mythical creature as a guardian, what would it be?",
    "What is a skill you wish you had?",
    "What is a book that you think is underrated and deserves more attention?",
    "What is your favorite way to spend a summer evening?",
    "If you could live in any time period, when would it be?",
    "What is a fictional language from a book or movie that you'd love to be fluent in?",
    "If you could have a conversation with your younger self, what would you say?",
    "What is a goal you've achieved that initially seemed impossible?",
    "What is your favorite way to show gratitude?",
    "What is a movie that you find endlessly quotable?",
    "If you could bring to life any fictional vehicle for a day, what would it be used for?",
    "If you could have any job in the field of technology, what would it be?",
    "If you could possess any knowledge from an ancient civilization, what would it be about?",
    "If you could have a superpower just for a day, what would you choose?",
    "If you could have any fictional companion, who or what would it be?",
    "What is your favorite way to learn about different cultures?",
    "If you could learn any language, which one would it be?",
    "If you could have any job for a month, what would it be?",
    "If you could have any job in the field of business, what would it be?",
    "If you could have any job in the field of science, what would it be?",
    "What is your favorite method for keeping up with current events?",
];

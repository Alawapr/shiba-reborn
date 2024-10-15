use poise::serenity_prelude::ActivityType::{self, *};

pub const ACTIVITIES: [(ActivityType, &str); 10] = [
    (Watching, "the Bee movie"),
    (Competing, "counting bytes"),
    (Listening, "yeat"),
    // @ is a placeholder that will then be replaced
    // with the number of servers
    (Watching, "@ servers"),
    (Competing, "a cuteness contest"),
    (Playing, "fetch with virtual bones"),
    (Watching, "the code compile"),
    (Watching, "for incoming belly rubs"),
    (Competing, "tail wagging contests"),
    (Playing, "hide and seek"),
];

use std::collections::HashMap;

use teloxide::macros::BotCommands;

use crate::training::Training;

#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Help,
    Start,
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Start,
    Training(HashMap<String, Training>),
}
